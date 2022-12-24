use clap::Parser;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

struct Blizzard {
    x: i32,
    y: i32,
    direction_x: i32,
    direction_y: i32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct SearchState {
    x: i32,
    y: i32,
    step_number: i32,
    goals: Vec<(i32, i32)>,
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // We want the comparison to be based on the number of steps to the destination
        let mut self_steps_to_goal = self.step_number;
        let mut last_position = (self.x, self.y);
        for goal in self.goals.iter() {
            self_steps_to_goal +=
                (goal.0 - last_position.0).abs() + (goal.1 - last_position.1).abs();
            last_position = *goal;
        }
        let mut other_steps_to_goal = other.step_number;
        last_position = (other.x, other.y);
        for goal in other.goals.iter() {
            other_steps_to_goal +=
                (goal.0 - last_position.0).abs() + (goal.1 - last_position.1).abs();
            last_position = *goal;
        }

        match self_steps_to_goal.partial_cmp(&other_steps_to_goal) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.step_number.partial_cmp(&other.step_number) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.goals.partial_cmp(&other.goals)
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let width = lines[0].len();
    let height = lines.len();

    let blizzards: Vec<Blizzard> = lines
        .iter()
        .enumerate()
        .skip(1)
        .take_while(|(_, line)| !line.starts_with("###"))
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '.' => None,
                    '#' => None,
                    '>' => Some(Blizzard {
                        x: x as i32,
                        y: y as i32,
                        direction_x: 1,
                        direction_y: 0,
                    }),
                    '<' => Some(Blizzard {
                        x: x as i32,
                        y: y as i32,
                        direction_x: -1,
                        direction_y: 0,
                    }),
                    '^' => Some(Blizzard {
                        x: x as i32,
                        y: y as i32,
                        direction_x: 0,
                        direction_y: -1,
                    }),
                    'v' => Some(Blizzard {
                        x: x as i32,
                        y: y as i32,
                        direction_x: 0,
                        direction_y: 1,
                    }),
                    _ => panic!("Unexpected char"),
                })
                .collect::<Vec<Blizzard>>()
        })
        .flatten()
        .collect();

    let part1 = find_best_path(
        (1, 0),
        vec![(width as i32 - 2, height as i32 - 1)],
        width,
        height,
        &blizzards,
        &args,
        0,
    );
    println!("Part 1: {}", part1);

    let mut part2 = part1;
    part2 = find_best_path(
        (width as i32 - 2, height as i32 - 1),
        vec![(1, 0)],
        width,
        height,
        &blizzards,
        &args,
        part2,
    );
    part2 = find_best_path(
        (1, 0),
        vec![(width as i32 - 2, height as i32 - 1)],
        width,
        height,
        &blizzards,
        &args,
        part2,
    );
    println!("Part 2: {}", part2);
}

fn find_best_path(
    starting_point: (i32, i32),
    goals: Vec<(i32, i32)>,
    width: usize,
    height: usize,
    blizzards: &Vec<Blizzard>,
    args: &Args,
    start_time: i32,
) -> i32 {
    let mut states: BinaryHeap<Reverse<SearchState>> = BinaryHeap::new();
    states.push(Reverse(SearchState {
        x: starting_point.0,
        y: starting_point.1,
        step_number: start_time,
        goals: goals.clone(),
    }));

    let mut seen_states = HashSet::new();
    'search: while !states.is_empty() {
        let state = states.pop().unwrap().0;

        if !seen_states.insert(state.clone()) {
            continue;
        }

        if args.debug {
            println!(
                "Heap Size: {}, Step {}, Position {}, {}",
                states.len(),
                state.step_number,
                state.x,
                state.y
            );
        }

        let blizzards: Vec<&Blizzard> = blizzards
            .iter()
            .filter(|b| {
                (b.x.abs_diff(state.x) <= 1 && b.direction_y != 0)
                    || (b.y.abs_diff(state.y) <= 1 && b.direction_x != 0)
            })
            .collect();

        let blizzard_locations: Vec<(i32, i32)> = blizzards
            .iter()
            .map(|b| {
                (
                    // Minus 1 to become 0 based
                    (b.x - 1 + b.direction_x * (state.step_number + 1))
                        .rem_euclid(width as i32 - 2)
                        + 1,
                    (b.y - 1 + b.direction_y * (state.step_number + 1))
                        .rem_euclid(height as i32 - 2)
                        + 1,
                )
            })
            .filter(|b| b.0.abs_diff(state.x) <= 1 && b.1.abs_diff(state.y) <= 1)
            .collect();

        if args.debug {
            // Draw the board
            for y in 0..height {
                for x in 0..width {
                    if y == 0 {
                        if x == 1 {
                            print!(".");
                        } else {
                            print!("#");
                        }
                    } else if y == height - 1 {
                        if x == width - 2 {
                            print!(".");
                        } else {
                            print!("#");
                        }
                    } else if x == 0 || x == width - 1 {
                        print!("#");
                    } else {
                        if blizzard_locations.contains(&(x as i32, y as i32)) {
                            print!("B");
                        } else {
                            print!(".");
                        }
                    }
                }
                print!("\n");
            }
        }

        // First order is waiting
        if !blizzard_locations.contains(&(state.x, state.y)) {
            let mut state = state.clone();
            state.step_number += 1;
            states.push(Reverse(state));
        } else if args.debug {
            println!("Blizzard will be where I am");
        }

        for x_diff in [-1, 1] {
            let x = state.x + x_diff;
            if x == 0 || x == width as i32 - 1 {
                continue;
            }
            if state.y == 0 && x != 1 {
                continue;
            }
            if state.y == height as i32 - 1 && x != width as i32 - 2 {
                continue;
            }
            if !blizzard_locations.contains(&(x, state.y)) {
                let mut state = state.clone();
                state.x = x;
                state.step_number += 1;
                states.push(Reverse(state));
            } else if args.debug {
                println!("Blizzard will be at {}, {}", x, state.y);
            }
        }
        for y_diff in [-1, 1] {
            let y = state.y + y_diff;
            if y < 0 {
                continue;
            }
            if y == 0 && state.x != 1 {
                continue;
            }
            if y == height as i32 - 1 && state.x != width as i32 - 2 {
                continue;
            }

            if !blizzard_locations.contains(&(state.x, y)) {
                let mut state = state.clone();
                state.y = y;
                state.step_number += 1;

                if state.x == state.goals[0].0 && state.y == state.goals[0].1 {
                    println!(
                        "Goal ({}, {}) hit at {} steps",
                        state.x, state.y, state.step_number
                    );
                    // When reaching a goal we can clear all other states since they won't have reached the goal
                    states.clear();
                    seen_states.clear();
                    state.goals.remove(0);
                    if state.goals.is_empty() {
                        return state.step_number;
                    }

                    states.push(Reverse(state));
                    continue 'search;
                }

                states.push(Reverse(state));
            } else if args.debug {
                println!("Blizzard will be at {}, {}", state.x, y);
            }
        }
    }

    -1
}
