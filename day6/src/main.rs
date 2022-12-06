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
}

fn is_unique(last_four: &VecDeque<char>) -> bool {
    let mut set = HashSet::new();
    last_four.iter().for_each(|c| {
        set.insert(*c);
    });
    set.len() == 4
}
