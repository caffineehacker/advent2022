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

    let rocks: HashSet<(i32, i32)> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(get_rock_squares)
        .flatten()
        .collect();

    // Part 1: falling sand
    let last_rock_y = rocks.iter().map(|s| s.1).max().unwrap();
    let mut occupied_squares = rocks.clone();

    // We can be smart because the next piece of sand will always follow the same path as the previous one
    let mut sand_path = vec![(500, 0)];
    let mut settled_sand_count = 0;
    'outer: loop {
        loop {
            let test_point = sand_path.last().unwrap();
            if test_point.1 >= last_rock_y {
                break 'outer;
            }
            if !occupied_squares.contains(&(test_point.0, test_point.1 + 1)) {
                sand_path.push((test_point.0, test_point.1 + 1));
            } else if !occupied_squares.contains(&(test_point.0 - 1, test_point.1 + 1)) {
                sand_path.push((test_point.0 - 1, test_point.1 + 1));
            } else if !occupied_squares.contains(&(test_point.0 + 1, test_point.1 + 1)) {
                sand_path.push((test_point.0 + 1, test_point.1 + 1));
            } else {
                // Nowhere else to go, occupy this square, pop it off the test point and continue
                occupied_squares.insert(*test_point);
                sand_path.pop();
                settled_sand_count += 1;
                break;
            }
        }
    }

    println!("Settled sand count: {}", settled_sand_count);
}

fn get_rock_squares(line: String) -> Vec<(i32, i32)> {
    let endpoints: Vec<&str> = line.split(" -> ").collect();
    let mut occupied_squares = Vec::new();
    let (startx, starty) = endpoints[0].split_once(",").unwrap();
    let startx: i32 = startx.parse().unwrap();
    let starty: i32 = starty.parse().unwrap();
    occupied_squares.push((startx, starty));

    for endpoint in endpoints {
        let (x, y) = endpoint.split_once(",").unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        let last = *occupied_squares.last().unwrap();
        let diff = (
            (x - last.0) / (x - last.0).abs().max(1),
            (y - last.1) / (y - last.1).abs().max(1),
        );
        while *occupied_squares.last().unwrap() != (x, y) {
            let last = *occupied_squares.last().unwrap();
            occupied_squares.push((last.0 + diff.0, last.1 + diff.1));
        }
    }

    occupied_squares
}
