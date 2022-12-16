use clap::Parser;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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

#[derive(PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: u32,
    connected_valves: Vec<String>,
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

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let valves: HashMap<String, Rc<Valve>> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_valve)
        .map(|v| (v.name.to_string(), Rc::new(v)))
        .collect();

    let mut search_states: BinaryHeap<SearchState> = BinaryHeap::new();
    search_states.push(SearchState {
        current_valve_room: "AA".to_string(),
        time_passed: 0,
        already_flowed: 0,
        flow_per_minute: 0,
        enabled_valves: Vec::new(),
        remaining_valves: valves.values().cloned().collect(),
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

        let next_tick_flowed = current_state.already_flowed + current_state.flow_per_minute;
        let current_valve = valves.get(&current_state.current_valve_room).unwrap();
        if !current_state
            .enabled_valves
            .iter()
            .any(|v| v.name == current_state.current_valve_room)
        {
            let new_flow_rate = current_state.flow_per_minute + current_valve.flow_rate;
            if !seen_states.contains(&(current_valve.name.clone(), next_tick_flowed, new_flow_rate))
            {
                seen_states.insert((current_valve.name.clone(), next_tick_flowed, new_flow_rate));
                let mut enabled_valves = current_state.enabled_valves.clone();
                enabled_valves.push(current_valve.clone());
                let remaining_valves = current_state
                    .remaining_valves
                    .iter()
                    .filter(|v| v.name != current_state.current_valve_room)
                    .cloned()
                    .collect();
                search_states.push(SearchState {
                    current_valve_room: current_state.current_valve_room,
                    time_passed: current_state.time_passed + 1,
                    already_flowed: next_tick_flowed,
                    flow_per_minute: new_flow_rate,
                    enabled_valves: enabled_valves,
                    remaining_valves: remaining_valves,
                });
            }
        }

        current_valve.connected_valves.iter().for_each(|v| {
            if !seen_states.contains(&(v.clone(), next_tick_flowed, current_state.flow_per_minute))
            {
                seen_states.insert((v.clone(), next_tick_flowed, current_state.flow_per_minute));
                search_states.push(SearchState {
                    current_valve_room: v.clone(),
                    time_passed: current_state.time_passed + 1,
                    already_flowed: next_tick_flowed,
                    flow_per_minute: current_state.flow_per_minute,
                    enabled_valves: current_state.enabled_valves.clone(),
                    remaining_valves: current_state.remaining_valves.clone(),
                });
            }
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
    }
}
