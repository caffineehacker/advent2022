use clap::Parser;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    knots: usize,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect();

    let initial_state = vec![(0, 0); args.knots];

    let tail_covered_positions: Vec<(i32, i32)> = lines
        .iter()
        .scan(initial_state, |state, instruction| {
            let (direction, count) = instruction.split_once(' ').unwrap();
            let change = match direction {
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => panic!("Unexpected direction"),
            };
            let knot_count = state.len();

            let mut tail_covered_states = Vec::new();
            for _ in 0..(count.parse().expect("Failed to parse count")) {
                state[knot_count - 1].0 += change.0;
                state[knot_count - 1].1 += change.1;

                for knot in (0..(state.len() - 1)).rev() {
                    if ((state[knot].0 - state[knot + 1].0) as i32).abs() > 1
                        || ((state[knot].1 - state[knot + 1].1) as i32).abs() > 1
                    {
                        // Tail needs to move
                        if state[knot].1 != state[knot + 1].1 {
                            state[knot].1 +=
                                if ((state[knot + 1].1 - state[knot].1) as i32).is_negative() {
                                    -1
                                } else {
                                    1
                                };
                        }

                        if state[knot].0 != state[knot + 1].0 {
                            state[knot].0 +=
                                if ((state[knot + 1].0 - state[knot].0) as i32).is_negative() {
                                    -1
                                } else {
                                    1
                                };
                        }
                    }

                    if knot == 0 {
                        tail_covered_states.push(state[0]);
                    }
                }
            }

            Some(tail_covered_states)
        })
        .flat_map(|s| s)
        .collect();

    tail_covered_positions
        .iter()
        .for_each(|p| println!("({},{})", p.0, p.1));

    let tail_unique_positions: HashSet<(i32, i32)> =
        HashSet::from_iter(tail_covered_positions.iter().cloned());

    println!(
        "Number of positions covered: {}",
        tail_unique_positions.len()
    );
}
