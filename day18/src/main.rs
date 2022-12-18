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

    let exterier_sides: u32 = count_exterior_sides(&lava);
    println!("Part 2, exterier sides: {}", exterier_sides);
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

fn count_exterior_sides(lava: &HashSet<(i32, i32, i32)>) -> u32 {
    // This will conduct a search starting at the first exposed block.
    // We will then remove all blocks in the range that was found.
    // Then if there are still blocks, conduct more searches and repeat.

    let mut exterior_sides = 0;
    let mut lava = lava.clone();
    while !lava.is_empty() {
        let z = lava.iter().min_by_key(|l| l.2).unwrap().2;
        let starting_point = lava
            .iter()
            .filter(|l| l.2 == z)
            .max_by_key(|l| l.1)
            .unwrap();

        // This is (x, y, z), (x, y, z) where the first coordinate is the cube and the
        // second is a unit vector indicating the face.
        let mut seen_faces: HashSet<((i32, i32, i32), (i32, i32, i32))> = HashSet::new();

        let mut current_states: Vec<((i32, i32, i32), (i32, i32, i32))> = Vec::new();
        current_states.push((*starting_point, (0, 1, 0)));
        while !current_states.is_empty() {
            let (cube, face) = current_states.pop().unwrap();
            if !seen_faces.insert((cube, face)) {
                // We've already been here
                continue;
            }

            // Count exposed sides for current
            // Current outside face, this should ALWAYS be free
            if !lava.contains(&(cube.0 + face.0, cube.1 + face.1, cube.2 + face.2)) {
                exterior_sides += 1;
            } else {
                panic!("Face isn't free!!!");
            }

            // For a face of (0, 1, 0) we want to check cubes at offset (1, 1, 0), (-1, 1, 0), (0, 1, 1), and (0, 1, -1)
            if face.0 == 0 {
                for x in [-1, 1] {
                    if lava.contains(&(cube.0 + x, cube.1 + face.1, cube.2 + face.2)) {
                        current_states
                            .push(((cube.0 + x, cube.1 + face.1, cube.2 + face.2), (-x, 0, 0)));
                    } else if lava.contains(&(cube.0 + x, cube.1, cube.2)) {
                        current_states.push(((cube.0 + x, cube.1, cube.2), face));
                    } else {
                        current_states.push((cube, (x, 0, 0)));
                    }
                }
            }

            if face.1 == 0 {
                for y in [-1, 1] {
                    if lava.contains(&(cube.0 + face.0, cube.1 + y, cube.2 + face.2)) {
                        current_states
                            .push(((cube.0 + face.0, cube.1 + y, cube.2 + face.2), (0, -y, 0)));
                    } else if lava.contains(&(cube.0, cube.1 + y, cube.2)) {
                        current_states.push(((cube.0, cube.1 + y, cube.2), face));
                    } else {
                        current_states.push((cube, (0, y, 0)));
                    }
                }
            }

            if face.2 == 0 {
                for z in [-1, 1] {
                    if lava.contains(&(cube.0 + face.0, cube.1 + face.1, cube.2 + z)) {
                        current_states
                            .push(((cube.0 + face.0, cube.1 + face.1, cube.2 + z), (0, 0, -z)));
                    } else if lava.contains(&(cube.0, cube.1, cube.2 + z)) {
                        current_states.push(((cube.0, cube.1, cube.2 + z), face));
                    } else {
                        current_states.push((cube, (0, 0, z)));
                    }
                }
            }
        }
        remove_seen_and_enclosed_cubes(&mut lava, seen_faces);
    }

    exterior_sides
}

fn remove_seen_and_enclosed_cubes(
    lava: &mut HashSet<(i32, i32, i32)>,
    seen_faces: HashSet<((i32, i32, i32), (i32, i32, i32))>,
) {
    let min_z = seen_faces.iter().min_by_key(|f| f.0 .2).unwrap().0 .2;
    let max_z = seen_faces.iter().max_by_key(|f| f.0 .2).unwrap().0 .2;

    for z in min_z..=max_z {
        let faces_x: Vec<&((i32, i32, i32), (i32, i32, i32))> = seen_faces
            .iter()
            .filter(|f| f.0 .2 == z && f.1 .0 != 0)
            .collect();
        let min_y = faces_x.iter().min_by_key(|f| f.0 .1).unwrap().0 .1;
        let max_y = faces_x.iter().max_by_key(|f| f.0 .1).unwrap().0 .1;

        for y in min_y..=max_y {
            let mut faces_x: Vec<&((i32, i32, i32), (i32, i32, i32))> =
                faces_x.iter().filter(|f| f.0 .1 == y).cloned().collect();
            faces_x.sort_by_key(|f| (f.0 .0, f.1 .0));

            if faces_x.len() % 2 != 0 {
                panic!("Odd number of faces!");
            }

            // The faces should be pairs where we can remove all cubes between them as they are not reachable
            let mut faces_x = faces_x.iter();
            loop {
                let a = faces_x.next();
                if a.is_none() {
                    break;
                }
                let a = a.unwrap();
                let b = faces_x.next().unwrap();
                for x in a.0 .0..=b.0 .0 {
                    lava.remove(&(x, y, z));
                }
            }
        }
    }
}
