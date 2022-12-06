use clap::Parser;
use std::collections::{HashSet, VecDeque};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

fn main() {
    let args = Args::parse();

    let data = std::fs::read_to_string(&args.data_file).expect("Failed to open file");

    // In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different
    let mut last_four: VecDeque<char> = data.chars().take(4).collect();
    for (index, c) in data.chars().enumerate().skip(4) {
        if is_unique(&last_four) {
            println!("Index at {}", index);
            break;
        }

        last_four.pop_front();
        last_four.push_back(c);
    }

    // For part 2, use 14 chars
    let mut last_fourteen: VecDeque<char> = data.chars().take(14).collect();
    for (index, c) in data.chars().enumerate().skip(14) {
        if is_unique(&last_fourteen) {
            println!("Index at {}", index);
            break;
        }

        last_fourteen.pop_front();
        last_fourteen.push_back(c);
    }
}

fn is_unique(maybe_unique: &VecDeque<char>) -> bool {
    let mut set = HashSet::new();
    maybe_unique.iter().for_each(|c| {
        set.insert(*c);
    });
    set.len() == maybe_unique.len()
}
