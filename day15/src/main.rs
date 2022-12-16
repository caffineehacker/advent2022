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
    part1_y: u32,
    #[arg(long)]
    part2_max: u32,
}

#[derive(Clone, Copy)]
struct Sensor {
    location: (i32, i32),
    range: u32,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let sensors: Vec<Sensor> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_line)
        .collect();

    // Part 1
    let mut candidate_sensors: Vec<(i32, i32)> = sensors
        .iter()
        .filter(|s| s.location.1.abs_diff(args.part1_y as i32) <= s.range)
        .map(|s| {
            let remaining_range =
                s.range as i32 - s.location.1.abs_diff(args.part1_y as i32) as i32;
            (
                s.location.0 - remaining_range,
                s.location.0 + remaining_range,
            )
        })
        .collect();
    candidate_sensors.sort_by_key(|cs| cs.0);

    let covered_columns_count = candidate_sensors
        .iter()
        .fold((None, 0), |acc, sensor_range| {
            if acc.0.is_some() && acc.0.unwrap() > sensor_range.1 {
                return acc;
            }

            if acc.0.is_none() {
                return (Some(sensor_range.1), (sensor_range.1 - sensor_range.0) + 1);
            }

            (
                Some(sensor_range.1),
                acc.1 + sensor_range.1 - acc.0.unwrap(),
            )
        })
        .1;
    println!(
        "Part 1, covered cells in row {}: {}",
        args.part1_y, covered_columns_count
    );

    // Part 2
    let mut y = 0;
    while y <= args.part2_max {
        let mut candidate_sensors: Vec<(i32, i32)> = sensors
            .iter()
            .filter(|s| s.location.1.abs_diff(y as i32) <= s.range)
            .map(|s| {
                let remaining_range = s.range as i32 - s.location.1.abs_diff(y as i32) as i32;
                (
                    s.location.0 - remaining_range,
                    s.location.0 + remaining_range,
                )
            })
            .collect();
        candidate_sensors.sort_by_key(|cs| cs.0);
        let first_uncovered = candidate_sensors.iter().fold(0, |acc, cs| {
            if acc < cs.0 || acc > cs.1 {
                return acc;
            }

            return cs.1 + 1;
        });
        if first_uncovered < args.part2_max as i32 {
            println!("{}, {}", first_uncovered, y);
            println!(
                "Part 2 result: {}",
                first_uncovered as u64 * 4000000 + y as u64
            );
        }
        y += 1;
    }
}

fn parse_line(line: String) -> Sensor {
    let components: Vec<&str> = line.split_whitespace().collect();

    let sensor_location = (
        components[2]
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .parse()
            .unwrap(),
        components[3]
            .trim_start_matches("y=")
            .trim_end_matches(":")
            .parse()
            .unwrap(),
    );

    let beacon_location = (
        components[8]
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .parse()
            .unwrap(),
        components[9].trim_start_matches("y=").parse().unwrap(),
    );

    Sensor {
        location: sensor_location,
        range: sensor_location.0.abs_diff(beacon_location.0)
            + sensor_location.1.abs_diff(beacon_location.1),
    }
}
