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

#[derive(Clone, Copy)]
struct Beacon {
    location: (i32, i32),
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let sensors_and_beacons: Vec<(Sensor, Beacon)> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_line)
        .collect();

    // Part 1
    let candidate_sensors: Vec<Sensor> = sensors_and_beacons
        .iter()
        .map(|sb| sb.0)
        .filter(|s| s.location.1.abs_diff(args.part1_y as i32) <= s.range)
        .collect();

    let beacons_in_row: HashSet<i32> = sensors_and_beacons
        .iter()
        .map(|sb| sb.1.location.1)
        .filter(|y| *y == args.part1_y as i32)
        .collect();

    let sensed_columns: HashSet<i32> = candidate_sensors
        .iter()
        .flat_map(|s| {
            let distance_to_target_row = s.location.1.abs_diff(args.part1_y as i32);
            let remaining_range = s.range as i32 - distance_to_target_row as i32;
            (s.location.0 - remaining_range)..=(s.location.0 + remaining_range)
        })
        .collect();

    let covered_columns = sensed_columns.difference(&beacons_in_row).count();
    println!(
        "Part 1, covered cells in row {}: {}",
        args.part1_y, covered_columns
    );

    // Part 2
    let mut y = 0;
    while y <= args.part2_max {
        let mut candidate_sensors: Vec<(i32, i32)> = sensors_and_beacons
            .iter()
            .map(|sb| sb.0)
            .filter(|s| s.location.1.abs_diff(y as i32) <= s.range)
            .map(|s| {
                let remaining_range = s.range as i32 - s.location.1.abs_diff(y as i32) as i32;
                (
                    s.location.0 - remaining_range,
                    s.location.0 + remaining_range,
                )
            })
            .collect();
        candidate_sensors.sort_by_cached_key(|cs| cs.0);
        let mut x = 0;
        while x <= args.part2_max {
            let covering_sensor = candidate_sensors
                .iter()
                .filter(|cs| cs.0 <= x as i32 && cs.1 >= x as i32)
                .nth(0);

            match covering_sensor {
                Some(cs) => {
                    x = cs.1 as u32;
                }
                None => {
                    println!("Part 2, beacon is at {}, {}", x, y);
                    println!("Part 2 result: {}", x as u64 * 4000000 + y as u64);
                    return;
                }
            }
            println!("{}, {}", x, y);

            x += 1;
        }
        y += 1;
    }
}

fn parse_line(line: String) -> (Sensor, Beacon) {
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

    (
        Sensor {
            location: sensor_location,
            range: sensor_location.0.abs_diff(beacon_location.0)
                + sensor_location.1.abs_diff(beacon_location.1),
        },
        Beacon {
            location: beacon_location,
        },
    )
}
