use clap::Parser;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    connected_valves: Vec<String>,
    valve_distances: HashMap<String, u32>,
}

#[derive(PartialEq, Eq)]
struct SearchState {
    current_valve_room: String,
    time_passed: u32,
    already_flowed: u32,
    flow_per_minute: u32,
    enabled_valves: Vec<Rc<Valve>>,
    remaining_valves: Vec<Rc<Valve>>,
}

impl SearchState {
    fn potential_flow(&self) -> u32 {
        let mut remaining_valves = self.remaining_valves.clone();
        remaining_valves.sort_by_key(|v| v.flow_rate);
        let remaining_time = 30 - self.time_passed;
        let potential_flow = self.already_flowed
            + (self.flow_per_minute * remaining_time)
            + remaining_valves
                .iter()
                .rev()
                .enumerate()
                .take(remaining_time as usize)
                .map(|rv| {
                    let time_to_open_valve = rv.0 as u32
                        * if rv.1.name == self.current_valve_room {
                            1
                        } else {
                            2
                        };
                    if time_to_open_valve >= remaining_time {
                        return 0;
                    }
                    let remaining_time = remaining_time - time_to_open_valve;
                    rv.1.flow_rate * remaining_time
                })
                .sum::<u32>();

        potential_flow
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.potential_flow().cmp(&other.potential_flow())
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
struct Part2SearchState {
    my_next_room: Rc<Valve>,
    elephant_next_room: Rc<Valve>,
    my_time_to_destination: u32,
    elephant_time_to_destination: u32,
    time_passed: u32,
    already_flowed: u32,
    flow_per_minute: u32,
    enabled_valves: Vec<Rc<Valve>>,
    remaining_valves: Vec<Rc<Valve>>,
}

impl Part2SearchState {
    fn potential_flow(&self) -> u32 {
        let mut remaining_valves = self.remaining_valves.clone();
        remaining_valves.sort_by_key(|v| v.flow_rate);
        if self.my_time_to_destination > 0 {
            remaining_valves.push(self.my_next_room.clone());
        }
        if self.elephant_time_to_destination > 0 {
            remaining_valves.push(self.elephant_next_room.clone());
        }
        let remaining_time = 26 - self.time_passed;
        let potential_flow = self.already_flowed
            + (self.flow_per_minute * remaining_time)
            + remaining_valves
                .iter()
                .rev()
                .enumerate()
                .take(remaining_time as usize)
                .map(|rv| {
                    let time_to_open_valve = rv.0 as u32 * 2;
                    if time_to_open_valve >= remaining_time {
                        return 0;
                    }
                    let remaining_time = remaining_time - time_to_open_valve;
                    rv.1.flow_rate * remaining_time
                })
                .sum::<u32>();

        potential_flow
    }
}

impl Ord for Part2SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.potential_flow().cmp(&other.potential_flow())
    }
}

impl PartialOrd for Part2SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let valves: Vec<Valve> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_valve)
        .collect();
    let valves = update_distances(valves);
    let valves: HashMap<String, Rc<Valve>> = valves
        .into_iter()
        .map(|v| (v.name.to_string(), Rc::new(v)))
        .collect();

    do_part_1(&valves);
    do_part_2(&valves);
}

fn do_part_1(valves: &HashMap<String, Rc<Valve>>) {
    let mut search_states: BinaryHeap<SearchState> = BinaryHeap::new();
    search_states.push(SearchState {
        current_valve_room: "AA".to_string(),
        time_passed: 0,
        already_flowed: 0,
        flow_per_minute: 0,
        enabled_valves: Vec::new(),
        remaining_valves: valves
            .values()
            .filter(|v| v.name != "AA" && v.flow_rate > 0)
            .cloned()
            .collect(),
    });

    // Holds a hash of room, flowed, and flow rate. Once we've been somewhere at a state once, we shouldn't go there in the same state again
    let mut seen_states: HashSet<(String, u32, u32)> = HashSet::new();
    seen_states.insert(("AA".to_string(), 0, 0));

    loop {
        let current_state = search_states.pop().unwrap();
        if current_state.time_passed == 30 {
            println!(
                "Part 1, pressure released: {}",
                current_state.already_flowed
            );
            break;
        }

        println!(
            "Time: {}, Room: {}, Flowed: {}, Flow rate: {}",
            current_state.time_passed,
            current_state.current_valve_room,
            current_state.already_flowed,
            current_state.flow_per_minute
        );

        current_state.remaining_valves.iter().for_each(|v| {
            let valve_distance = v.valve_distances[&current_state.current_valve_room];
            if current_state.time_passed + valve_distance + 1 > 30 {
                return;
            }

            let next_flowed =
                current_state.already_flowed + current_state.flow_per_minute * (valve_distance + 1);
            let new_flow_rate = current_state.flow_per_minute + v.flow_rate;
            let mut enabled_valves = current_state.enabled_valves.clone();
            enabled_valves.push(v.clone());
            let remaining_valves = current_state
                .remaining_valves
                .iter()
                .filter(|rv| v.name != rv.name)
                .cloned()
                .collect();
            if !seen_states.contains(&(v.name.clone(), next_flowed, new_flow_rate)) {
                seen_states.insert((v.name.clone(), next_flowed, new_flow_rate));
                search_states.push(SearchState {
                    current_valve_room: v.name.clone(),
                    time_passed: current_state.time_passed + valve_distance + 1,
                    already_flowed: next_flowed,
                    flow_per_minute: new_flow_rate,
                    enabled_valves,
                    remaining_valves,
                });
            }
        });

        if current_state.remaining_valves.is_empty() {
            search_states.push(SearchState {
                current_valve_room: current_state.current_valve_room,
                time_passed: 30,
                already_flowed: current_state.already_flowed
                    + current_state.flow_per_minute * (30 - current_state.time_passed),
                flow_per_minute: current_state.flow_per_minute,
                enabled_valves: current_state.enabled_valves,
                remaining_valves: current_state.remaining_valves,
            });
        }
    }
}

fn do_part_2(valves: &HashMap<String, Rc<Valve>>) {
    let mut search_states: BinaryHeap<Part2SearchState> = BinaryHeap::new();
    search_states.push(Part2SearchState {
        my_next_room: valves["AA"].clone(),
        my_time_to_destination: 0,
        elephant_next_room: valves["AA"].clone(),
        elephant_time_to_destination: 0,
        time_passed: 0,
        already_flowed: 0,
        flow_per_minute: 0,
        enabled_valves: Vec::new(),
        remaining_valves: valves
            .values()
            .filter(|v| v.name != "AA" && v.flow_rate > 0)
            .cloned()
            .collect(),
    });

    loop {
        let mut current_state = search_states.pop().unwrap();
        if current_state.time_passed == 26 {
            println!(
                "Part 2, pressure released: {}",
                current_state.already_flowed
            );
            break;
        }

        println!(
            "Time: {}, Rooms: {} ({}) / {} ({}), Flowed: {}, Flow rate: {}",
            current_state.time_passed,
            current_state.my_next_room.name,
            current_state.my_time_to_destination,
            current_state.elephant_next_room.name,
            current_state.elephant_time_to_destination,
            current_state.already_flowed,
            current_state.flow_per_minute
        );

        if current_state.my_time_to_destination > 0
            && current_state.elephant_time_to_destination > 0
        {
            // First flow time before looking at destinations
            let my_turn =
                current_state.my_time_to_destination < current_state.elephant_time_to_destination;
            let time_to_flow = if my_turn {
                current_state.my_time_to_destination
            } else {
                current_state.elephant_time_to_destination
            };
            let time_to_flow = time_to_flow.min(26 - current_state.time_passed);

            current_state.my_time_to_destination -= time_to_flow;
            current_state.elephant_time_to_destination -= time_to_flow;
            current_state.time_passed += time_to_flow;
            current_state.already_flowed += current_state.flow_per_minute * time_to_flow;

            if current_state.my_time_to_destination == 0 {
                let destination = current_state.my_next_room.clone();
                current_state.flow_per_minute += destination.flow_rate;
                current_state.enabled_valves.push(destination);
            }

            if current_state.elephant_time_to_destination == 0 {
                let destination = current_state.elephant_next_room.clone();
                current_state.flow_per_minute += destination.flow_rate;
                current_state.enabled_valves.push(destination);
            }

            search_states.push(current_state);

            continue;
        }

        if current_state.remaining_valves.is_empty() {
            // We'll never pass this much time, but it will trigger the above condition which will accelerate time to 26 minutes
            if current_state.my_time_to_destination == 0 {
                current_state.my_time_to_destination = 1000;
            }
            if current_state.elephant_time_to_destination == 0 {
                current_state.elephant_time_to_destination = 1000;
            }
            search_states.push(current_state);
            continue;
        }

        current_state.remaining_valves.iter().for_each(|v| {
            let my_turn = current_state.my_time_to_destination == 0;
            let current_room = if my_turn {
                &current_state.my_next_room
            } else {
                &current_state.elephant_next_room
            };
            let valve_distance = v.valve_distances[&current_room.name];

            let remaining_valves = current_state
                .remaining_valves
                .iter()
                .filter(|rv| v.name != rv.name)
                .cloned()
                .collect();

            search_states.push(Part2SearchState {
                my_next_room: if my_turn {
                    v.clone()
                } else {
                    current_state.my_next_room.clone()
                },
                elephant_next_room: if my_turn {
                    current_state.elephant_next_room.clone()
                } else {
                    v.clone()
                },
                my_time_to_destination: if my_turn {
                    valve_distance + 1
                } else {
                    current_state.my_time_to_destination
                },
                elephant_time_to_destination: if my_turn {
                    current_state.elephant_time_to_destination
                } else {
                    valve_distance + 1
                },
                time_passed: current_state.time_passed,
                already_flowed: current_state.already_flowed,
                flow_per_minute: current_state.flow_per_minute,
                enabled_valves: current_state.enabled_valves.clone(),
                remaining_valves,
            });
        });
    }
}

fn parse_valve(line: String) -> Valve {
    let components: Vec<&str> = line.split_whitespace().collect();

    Valve {
        name: components[1].to_string(),
        flow_rate: components[4]
            .trim_start_matches("rate=")
            .trim_end_matches(";")
            .parse()
            .unwrap(),
        connected_valves: components
            .iter()
            .skip(9)
            .map(|v| v.trim_end_matches(",").to_string())
            .collect(),
        valve_distances: HashMap::new(),
    }
}

fn update_distances(valves: Vec<Valve>) -> Vec<Valve> {
    let mut modified_valves: Vec<Valve> = Vec::new();

    for valve in valves.iter() {
        if valve.name != "AA" && valve.flow_rate <= 0 {
            continue;
        }

        let mut new_valve = valve.clone();

        new_valve.valve_distances = valves
            .iter()
            .filter(|v| v.name != valve.name)
            .filter(|v| v.flow_rate > 0 || v.name == "AA")
            .map(|v| {
                (
                    v.name.clone(),
                    calculate_distance(&valve, v, &valves, &modified_valves),
                )
            })
            .collect();

        modified_valves.push(new_valve);
    }

    modified_valves
}

fn calculate_distance(
    valve_a: &Valve,
    valve_b: &Valve,
    valves: &Vec<Valve>,
    modified_valves: &Vec<Valve>,
) -> u32 {
    if let Some(modified_valve) = modified_valves.iter().find(|mv| mv.name == valve_b.name) {
        if let Some(distance) = modified_valve.valve_distances.get(&valve_a.name) {
            return *distance;
        }
    }

    let mut seen_rooms = HashSet::new();
    let mut search_states = VecDeque::from([(valve_a, 0)]);

    loop {
        let current_state = search_states.pop_front().unwrap();
        if seen_rooms.contains(&current_state.0.name) {
            continue;
        }

        seen_rooms.insert(current_state.0.name.clone());
        current_state.0.connected_valves.iter().for_each(|v| {
            let valve = valves.iter().find(|valve| valve.name == *v).unwrap();
            search_states.push_back((valve, current_state.1 + 1));
        });

        if current_state.0.name == valve_b.name {
            return current_state.1;
        }
    }
}
