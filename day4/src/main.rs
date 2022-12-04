use clap::Parser;
use std::{
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

    let fully_overlapped_count = reader
        .lines()
        .map(|l| l.expect("Failed to parse line"))
        .filter(only_one_assignment_needed)
        .count();
    println!("Fully overlapped count: {}", fully_overlapped_count);
}

fn only_one_assignment_needed(input: &String) -> bool {
    let (left, right) = input.split_once(',').expect("Failed to split at ','");
    let (left_start, left_end) = left.split_once('-').expect("Failed to split on '-'");
    let (right_start, right_end) = right.split_once('-').expect("Failed to split on '-'");

    let left_start: u32 = left_start.parse().expect("Failed to parse number");
    let left_end: u32 = left_end.parse().expect("Failed to parse number");
    let right_start: u32 = right_start.parse().expect("Failed to parse number");
    let right_end: u32 = right_end.parse().expect("Failed to parse number");

    // Check if there is a complete overlap
    return (left_start <= right_start && left_end >= right_end)
        || (right_start <= left_start && right_end >= left_end);
}
