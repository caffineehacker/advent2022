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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect();

    let covered_positions: Vec<(i32, i32)> = lines
        .iter()
        .scan(((0, 0), (0, 0)), |state, instruction| {
            let (direction, count) = instruction.split_once(' ').unwrap();
            let change = match direction {
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => panic!("Unexpected direction"),
            };

            let mut covered_states = Vec::new();
            for _ in 0..(count.parse().expect("Failed to parse count")) {
                state.1 .0 += change.0;
                state.1 .1 += change.1;

                if ((state.0 .0 - state.1 .0) as i32).abs() > 1
                    || ((state.0 .1 - state.1 .1) as i32).abs() > 1
                {
                    // Tail needs to move
                    if state.0 .1 != state.1 .1 {
                        state.0 .1 += if ((state.1 .1 - state.0 .1) as i32).is_negative() {
                            -1
                        } else {
                            1
                        };
                    }

                    if state.0 .0 != state.1 .0 {
                        state.0 .0 += if ((state.1 .0 - state.0 .0) as i32).is_negative() {
                            -1
                        } else {
                            1
                        };
                    }
                }

                covered_states.push(state.0);
            }

            Some(covered_states)
        })
        .flat_map(|s| s)
        .collect();

    covered_positions
        .iter()
        .for_each(|p| println!("({},{})", p.0, p.1));

    let unique_positions: HashSet<(i32, i32)> =
        HashSet::from_iter(covered_positions.iter().cloned());

    println!("Number of positions covered: {}", unique_positions.len());
}
