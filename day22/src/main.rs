use clap::Parser;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Empty,
    Wall,
}

impl Tile {
    fn is_empty(&self) -> bool {
        match self {
            Tile::Empty => true,
            _ => false,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => panic!("Unexpected tile"),
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    let map: HashMap<Position, Tile> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .map(|(x, c)| {
                    (
                        Position {
                            x: x as i32,
                            y: y as i32,
                        },
                        Tile::from(c),
                    )
                })
                .collect::<Vec<(Position, Tile)>>()
        })
        .flatten()
        .collect();

    let directions = lines.last().unwrap();

    do_part1(&map, &directions);
}

fn do_part1(map: &HashMap<Position, Tile>, directions: &str) {
    let starting_position = map
        .iter()
        .filter(|(_, tile)| tile.is_empty())
        .min_by(|(pos_a, _), (pos_b, _)| {
            let ord = pos_a.y.cmp(&pos_b.y);
            if ord == Ordering::Equal {
                pos_a.x.cmp(&pos_b.x)
            } else {
                ord
            }
        })
        .unwrap()
        .0;

    let mut position = *starting_position;
    let mut facing = Position { x: 1, y: 0 };

    let mut index = 0;
    while index < directions.len() {
        let next_char = directions.chars().nth(index).unwrap();
        if next_char >= '0' && next_char <= '9' {
            // Movement
            let mut end_index = index + 1;
            let mut next_char = directions.chars().nth(end_index);
            while next_char.is_some() && next_char.unwrap() >= '0' && next_char.unwrap() <= '9' {
                end_index += 1;
                next_char = directions.chars().nth(end_index);
            }

            let movement_count: i32 = directions[index..end_index].parse().unwrap();
            for _ in 0..movement_count {
                let mut next_position = Position {
                    x: position.x + facing.x,
                    y: position.y + facing.y,
                };
                if !map.contains_key(&next_position) {
                    if facing.x == 1 {
                        next_position = *map
                            .iter()
                            .filter(|(pos, _)| pos.y == position.y)
                            .min_by_key(|(pos, _)| pos.x)
                            .unwrap()
                            .0;
                    } else if facing.x == -1 {
                        next_position = *map
                            .iter()
                            .filter(|(pos, _)| pos.y == position.y)
                            .max_by_key(|(pos, _)| pos.x)
                            .unwrap()
                            .0;
                    } else if facing.y == 1 {
                        next_position = *map
                            .iter()
                            .filter(|(pos, _)| pos.x == position.x)
                            .min_by_key(|(pos, _)| pos.y)
                            .unwrap()
                            .0;
                    } else if facing.y == -1 {
                        next_position = *map
                            .iter()
                            .filter(|(pos, _)| pos.x == position.x)
                            .max_by_key(|(pos, _)| pos.y)
                            .unwrap()
                            .0;
                    }
                }

                let tile = map.get(&next_position).unwrap();
                if tile.is_empty() {
                    position = next_position;
                } else {
                    break;
                }
            }
            index = end_index;
        } else {
            // We're turning
            match next_char {
                'L' => {
                    facing = Position {
                        x: facing.y,
                        y: -facing.x,
                    };
                }
                'R' => {
                    facing = Position {
                        x: -facing.y,
                        y: facing.x,
                    };
                }
                _ => panic!("Unexpected turn character"),
            };
            index += 1;
        }
    }

    // Password is 1000 * (y + 1) + 4 * (x + 1) + facing
    // Facing is 0 for right, 1 for down, 2 for left, 3 for up
    let password = 1000 * (position.y + 1)
        + 4 * (position.x + 1)
        + (if facing.x == -1 { 2 } else { 0 })
        + (if facing.y == -1 { 3 } else { facing.y });

    println!("Part 1 password: {}", password);
}
