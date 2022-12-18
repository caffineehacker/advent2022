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

    let lava: HashSet<(i32, i32, i32)> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(|line| {
            let mut components = line.split(",");
            (
                components.next().unwrap().parse().unwrap(),
                components.next().unwrap().parse().unwrap(),
                components.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let uncovered_sides: u32 = lava.iter().map(|l| count_uncovered_sides(l, &lava)).sum();
    println!("Part 1, uncovered sides: {}", uncovered_sides);
}

fn count_uncovered_sides(current: &(i32, i32, i32), lava: &HashSet<(i32, i32, i32)>) -> u32 {
    let mut count = 0;
    if !lava.contains(&(current.0 + 1, current.1, current.2)) {
        count += 1;
    }
    if !lava.contains(&(current.0 - 1, current.1, current.2)) {
        count += 1;
    }
    if !lava.contains(&(current.0, current.1 + 1, current.2)) {
        count += 1;
    }
    if !lava.contains(&(current.0, current.1 - 1, current.2)) {
        count += 1;
    }
    if !lava.contains(&(current.0, current.1, current.2 + 1)) {
        count += 1;
    }
    if !lava.contains(&(current.0, current.1, current.2 - 1)) {
        count += 1;
    }
    count
}
