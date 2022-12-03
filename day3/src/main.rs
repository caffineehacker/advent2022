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

    let priority_sum: u32 = reader
        .lines()
        .map(|line| line.expect("Failed to create line"))
        .map(find_repeated_item)
        .map(item_priority)
        .sum();

    println!("Part 1 total priority: {}", priority_sum);

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut current_elves = Vec::new();
    let mut part2_priority_total = 0;
    for line in reader.lines() {
        current_elves.push(line.expect("Failed to get line"));
        if current_elves.len() == 3 {
            let repeated_item = *current_elves
                .iter()
                .map(|e| e.chars().collect::<HashSet<char>>())
                .reduce(|a, e| a.intersection(&e).map(|c| *c).collect::<HashSet<char>>())
                .expect("Failed to find repeated item")
                .iter()
                .last()
                .expect("No repeated item found");

            part2_priority_total += item_priority(repeated_item);

            current_elves = Vec::new();
        }
    }

    println!("Part 2 total priority: {}", part2_priority_total);
}

fn find_repeated_item(backpack: String) -> char {
    // Each backpack is an even number of items split among two pockets
    let (left, right) = backpack.split_at(backpack.len() / 2);
    let left: HashSet<char> = left.chars().collect();
    let right: HashSet<char> = right.chars().collect();

    *left
        .intersection(&right)
        .last()
        .expect("Failed to find intersection")
}

fn item_priority(item: char) -> u32 {
    // a-z are 1-26
    // A-Z are 27-52

    if item.is_ascii_uppercase() {
        return u32::from(item) - u32::from('A') + 27;
    }

    u32::from(item) - u32::from('a') + 1
}
