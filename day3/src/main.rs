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

    let file = File::open(args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let priority_sum: u32 = reader
        .lines()
        .map(|line| line.expect("Failed to create line"))
        .map(find_repeated_item)
        .map(item_priority)
        .sum();

    println!("Total priority: {}", priority_sum);
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
        println!("{}: {}", item, u32::from(item) - u32::from('A') + 1);
        return u32::from(item) - u32::from('A') + 27;
    }

    println!("{}: {}", item, u32::from(item) - u32::from('a') + 1);
    u32::from(item) - u32::from('a') + 1
}
