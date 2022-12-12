use clap::Parser;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MapPoint {
    Normal(u32),
    Start,
    End,
}

impl MapPoint {
    fn is_normal(&self) -> bool {
        match self {
            MapPoint::Normal(_) => true,
            _ => false,
        }
    }

    fn unwrap(&self) -> u32 {
        match self {
            MapPoint::Normal(e) => *e,
            MapPoint::Start => 0,
            MapPoint::End => 25,
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let elevations: HashMap<(usize, usize), MapPoint> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .enumerate()
        .map(|(line_number, line)| {
            line.chars()
                .enumerate()
                .map(|(column_number, c)| match c {
                    'S' => ((column_number, line_number), MapPoint::Start),
                    'E' => ((column_number, line_number), MapPoint::End),
                    _ => (
                        (column_number, line_number),
                        MapPoint::Normal(c as u32 - 'a' as u32),
                    ),
                })
                .collect::<Vec<((usize, usize), MapPoint)>>()
        })
        .flatten()
        .collect();

    let start_coordinates = elevations
        .iter()
        .find(|e| *e.1 == MapPoint::Start)
        .unwrap()
        .0;

    let end_coordinates = elevations.iter().find(|e| *e.1 == MapPoint::End).unwrap().0;

    // Since we want the shortest route we can do a BFS. We only need to keep track of the length of the route and the cells visited by any route since if another route already visited a cell then we know that was shorter.
    let mut current_positions = vec![*start_coordinates];
    let mut visited_positions = HashSet::new();
    visited_positions.insert(*start_coordinates);
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        current_positions = current_positions
            .iter()
            .map(|pos| {
                (0..4)
                    .map(|i| {
                        let i = i as isize;
                        let new_pos_candidate = (
                            (pos.0 as isize) + ((i - 2) % 2),
                            (pos.1 as isize) + ((i - 1) % 2),
                        );

                        if new_pos_candidate.0 < 0 || new_pos_candidate.1 < 0 {
                            return None;
                        }

                        let new_pos_candidate =
                            (new_pos_candidate.0 as usize, new_pos_candidate.1 as usize);

                        if !elevations.contains_key(&new_pos_candidate) {
                            return None;
                        }
                        if visited_positions.contains(&new_pos_candidate) {
                            return None;
                        }

                        let current_elevation = elevations[pos].unwrap();
                        let destination_elevation = elevations[&new_pos_candidate].unwrap();

                        if destination_elevation > current_elevation + 1 {
                            return None;
                        }

                        visited_positions.insert(new_pos_candidate);
                        return Some(new_pos_candidate);
                    })
                    .filter_map(|np| np)
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();

        if current_positions.contains(&end_coordinates) {
            break;
        }
    }

    println!("Path is of length {}", loop_count);
}
