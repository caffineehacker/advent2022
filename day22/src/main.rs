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
struct Point2 {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
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
    let map: HashMap<Point2, Tile> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .map(|(x, c)| {
                    (
                        Point2 {
                            x: x as i32,
                            y: y as i32,
                        },
                        Tile::from(c),
                    )
                })
                .collect::<Vec<(Point2, Tile)>>()
        })
        .flatten()
        .collect();

    let directions = lines.last().unwrap();

    do_part1(&map, &directions);
    do_part2(&map, &directions);
}

fn do_part1(map: &HashMap<Point2, Tile>, directions: &str) {
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
    let mut facing = Point2 { x: 1, y: 0 };

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
                let mut next_position = Point2 {
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
                    facing = Point2 {
                        x: facing.y,
                        y: -facing.x,
                    };
                }
                'R' => {
                    facing = Point2 {
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

fn do_part2(map: &HashMap<Point2, Tile>, directions: &str) {
    // Now we're working with a cube

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

    // FIXME FIXME FIXME: This is hard coding my cube's data
    // To facilitate the 3d cube, we're going to number the sides and indicate the rotations
    // The sides will be numbered as:
    //   1 2
    //   3
    // 4 5
    // 6

    // Also can be like
    //
    // 2A
    // 1A3A
    //   4 5
    //   6

    //   1
    //   3
    // 4 5 2CC
    // 6

    let mut position = *starting_position;
    let mut facing = Point2 { x: 1, y: 0 };

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
                let mut next_position = Point2 {
                    x: position.x + facing.x,
                    y: position.y + facing.y,
                };
                let mut next_facing = facing;

                if facing.x == 1 {
                    if next_position.x == 150 && next_position.y < 50 {
                        // Going from 2 -> 5
                        next_position.y = (49 - next_position.y) + 100;
                        next_position.x = 99;

                        next_facing.x = -1;
                        next_facing.y = 0;
                    } else if next_position.x == 100
                        && next_position.y >= 50
                        && next_position.y < 100
                    {
                        // Going from 3 -> 2
                        next_position.x = next_position.y - 50 + 100;
                        next_position.y = 49;

                        next_facing.x = 0;
                        next_facing.y = -1;
                    } else if next_position.x == 100
                        && next_position.y >= 100
                        && next_position.y < 150
                    {
                        // Going from 5 -> 2
                        next_position.x = 149;
                        next_position.y = 49 - (next_position.y - 100);

                        next_facing.x = -1;
                        next_facing.y = 0;
                    } else if next_position.x == 50
                        && next_position.y >= 150
                        && next_position.y < 200
                    {
                        // Going from 6 -> 5
                        next_position.x = next_position.y - 150 + 50;
                        next_position.y = 149;

                        next_facing.x = 0;
                        next_facing.y = -1;
                    }
                } else if facing.x == -1 {
                    if next_position.x == 49 && next_position.y < 50 {
                        // Going from 1 -> 4
                        next_position.y = (49 - next_position.y) + 100;
                        next_position.x = 0;

                        next_facing.x = 1;
                        next_facing.y = 0;
                    } else if next_position.x == 49
                        && next_position.y >= 50
                        && next_position.y < 100
                    {
                        // Going from 3 -> 4
                        next_position.x = next_position.y - 50;
                        next_position.y = 100;

                        next_facing.x = 0;
                        next_facing.y = 1;
                    } else if next_position.x == -1
                        && next_position.y >= 100
                        && next_position.y < 150
                    {
                        // Going from 4 -> 1
                        next_position.x = 50;
                        next_position.y = 49 - (next_position.y - 100);

                        next_facing.x = 1;
                        next_facing.y = 0;
                    } else if next_position.x == -1
                        && next_position.y >= 150
                        && next_position.y < 200
                    {
                        // Going from 6 -> 1
                        next_position.x = next_position.y - 150 + 50;
                        next_position.y = 0;

                        next_facing.x = 0;
                        next_facing.y = 1;
                    }
                } else if facing.y == 1 {
                    if next_position.y == 200 && next_position.x >= 0 && next_position.x < 50 {
                        // Going from 6 -> 2
                        next_position.x = next_position.x + 100;
                        next_position.y = 0;

                        next_facing.x = 0;
                        next_facing.y = 1;
                    } else if next_position.y == 150
                        && next_position.x >= 50
                        && next_position.x < 100
                    {
                        // Going from 5 -> 6
                        next_position.y = next_position.x - 50 + 150;
                        next_position.x = 49;

                        next_facing.x = -1;
                        next_facing.y = 0;
                    } else if next_position.y == 50
                        && next_position.x >= 100
                        && next_position.x < 150
                    {
                        // Going from 2 -> 3
                        next_position.y = next_position.x - 100 + 50;
                        next_position.x = 99;

                        next_facing.x = -1;
                        next_facing.y = 0;
                    }
                } else if facing.y == -1 {
                    if next_position.y == 99 && next_position.x >= 0 && next_position.x < 50 {
                        // Going from 4 -> 3
                        next_position.y = next_position.x + 50;
                        next_position.x = 50;

                        next_facing.x = 1;
                        next_facing.y = 0;
                    } else if next_position.y == -1
                        && next_position.x >= 50
                        && next_position.x < 100
                    {
                        // Going from 1 -> 6
                        next_position.y = next_position.x - 50 + 150;
                        next_position.x = 0;

                        next_facing.x = 1;
                        next_facing.y = 0;
                    } else if next_position.y == -1
                        && next_position.x >= 100
                        && next_position.x < 150
                    {
                        // Going from 2 -> 6
                        next_position.x = next_position.x - 100;
                        next_position.y = 199;

                        next_facing.x = 0;
                        next_facing.y = -1;
                    }
                }

                let tile = map.get(&next_position).unwrap();
                if tile.is_empty() {
                    position = next_position;
                    facing = next_facing;
                } else {
                    break;
                }
            }
            index = end_index;
        } else {
            // We're turning
            match next_char {
                'L' => {
                    facing = Point2 {
                        x: facing.y,
                        y: -facing.x,
                    };
                }
                'R' => {
                    facing = Point2 {
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

    println!("Part 2 password: {}", password);
}

// fn create3d_map(
//     map: &HashMap<Point2, Tile>,
//     starting_position: &Point2,
// ) -> HashMap<Point3, (Tile, Point2)> {
//     let map_width = map.iter().max_by_key(|(pos, _)| pos.x).unwrap().0.x + 1;
//     let map_height = map.iter().max_by_key(|(pos, _)| pos.y).unwrap().0.y + 1;
//     let side_size = map_width.min(map_height) / 3;

//     // We're going to use a search algorithm starting at the first point

//     // This is (original_position, 3d space position, x/y/z projection, tile)
//     let mut to_process = vec![(
//         *starting_position,
//         Point3 { x: 0, y: 0, z: 0 },
//         [
//             Point2 { x: 1, y: 0 },
//             Point2 { x: 0, y: 1 },
//             Point2 { x: 0, y: 0 },
//         ],
//         map[starting_position],
//     )];
//     let mut map3d = HashMap::new();

//     while !to_process.is_empty() {
//         let current = to_process.pop().unwrap();
//         if !map.contains_key(&current.0) {
//             continue;
//         }

//         if map3d.contains_key(&current.1) {
//             continue;
//         }

//         map3d.insert(current.1, (current.3, current.0));
//         let p2d = Point2 {
//             x: current.0.x + 1,
//             y: current.0.y,
//         };
//         let mut p3d = current.1;
//         p3d.x += current.2[0].x;
//         p3d.y += current.2[1].x;
//         p3d.z += current.2[2].x;

//         let mut translation = current.2.clone();
//         if p3d.x == side_size && current.1.x != side_size {
//             // We only have to check y because we know X is mapped to X
//             if p3d.z == 0 {
//                 // 0 means we're on the front face
//                 translation[2].x = 1;
//                 translation[2].y = 0;
//             }
//             if p3d.y == -1 {
//                 // Top face
//                 translation[1].x = 1;
//                 translation[1].y = 0;
//             }

//             translation[0].x = 0;
//             translation[0].y = 0;
//         }
//         if (p3d.x == side_size && current.1.x != side_size) || (p3d.x == -1 && current.1.x != -1) {
//             translation[2].x = if p3d.x.is_negative() { -1 } else { 1 };
//             translation[2].y = 0;
//             if p3d.z != 0 {
//                 translation[2].x = -translation[2].x;
//             }
//             translation[0].x = 0;
//             translation[0].y = 0;
//             p3d.x = side_size
//         } else if (p3d.y == side_size && current.1.y != side_size)
//             || (p3d.y == -1 && current.1.y != -1)
//         {
//             translation[2].x = 0;
//             translation[2].y = if p3d.y.is_negative() { -1 } else { 1 };
//             if p3d.z != 0 {
//                 translation[2].y = -translation[2].y;
//             }
//             translation[0].x = 0;
//             translation[0].y = 0;
//             p3d.x = side_size
//         }
//         to_process.push((p2d, p3d, translation, map[&p2d]));
//     }

//     map3d
// }

// fn to_p3d(p2d: Point2, translation: &[Point2], side_size: i32) -> Point3 {
//     let x = translation[0].x * (p2d.x % side_size) + translation[0].y * (p2d.y % side_size);
//     let y = translation[1].x * (p2d.x % side_size) + translation[1].y * (p2d.y % side_size);
//     let z = translation[2].x * (p2d.x % side_size) + translation[2].y * (p2d.y % side_size);

//     Point3 { x, y, z }
// }

// fn to_p3d_overshoot(
//     p2d: Point2,
//     addition: Point2,
//     translation: &[Point2],
//     side_size: i32,
// ) -> Point3 {
//     let x = translation[0].x * (p2d.x % side_size + addition.x)
//         + translation[0].y * (p2d.y % side_size + addition.y);
//     let y = translation[1].x * (p2d.x % side_size + addition.x)
//         + translation[1].y * (p2d.y % side_size + addition.y);
//     let z = translation[2].x * (p2d.x % side_size + addition.x)
//         + translation[2].y * (p2d.y % side_size + addition.y);

//     Point3 { x, y, z }
// }
