use clap::Parser;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Shl, Shr},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rock {
    HorizontalLine,
    Plus,
    LeftL,
    VerticalLine,
    Square,
}

struct Rocks {
    next_rock: Rock,
}

impl Iterator for Rocks {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let rock = self.next_rock;
        self.next_rock = match self.next_rock {
            Rock::HorizontalLine => Rock::Plus,
            Rock::Plus => Rock::LeftL,
            Rock::LeftL => Rock::VerticalLine,
            Rock::VerticalLine => Rock::Square,
            Rock::Square => Rock::HorizontalLine,
        };

        Some(rock)
    }
}

#[derive(Clone, Copy)]
enum JetPush {
    Left,
    Right,
}

struct Jet {
    jet_pattern: String,
    next_index: usize,
}

impl Iterator for Jet {
    type Item = JetPush;

    fn next(&mut self) -> Option<Self::Item> {
        let push = match self.jet_pattern.chars().nth(self.next_index).unwrap() {
            '<' => JetPush::Left,
            '>' => JetPush::Right,
            _ => panic!("Unexpected push direction"),
        };

        self.next_index = (self.next_index + 1) % self.jet_pattern.len();

        Some(push)
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .last()
        .unwrap();

    let height = run_simulation(&input, 2022);
    println!("Height after 2022 is {}", height);

    let height = run_simulation(&input, 1000000000000);
    println!("Height after 1000000000000 is {}", height);
}

struct SettledState {
    row_index: usize,
    rocks_dropped: u64,
    height_above_row: usize,
    last_rock: Rock,
    next_jet_index: usize,
}

fn run_simulation(jet_input: &str, count: usize) -> u64 {
    let mut push_iter = Jet {
        jet_pattern: jet_input.to_string(),
        next_index: 0,
    };
    let mut rocks = Rocks {
        next_rock: Rock::HorizontalLine,
    };

    // Optimization time!
    // We're going to start keeping track after 1000 drops, that way we're more
    // likely to have already dealt with the fact that the start might be slightly different.
    //
    // We want to keep track of the last fully settled row, the state at that point and the number of rocks dropped
    // We might be able to find a pattern that repeats from a fully dropped row to another fully dropped row.
    let mut settled_states: Vec<SettledState> = Vec::new();
    let mut rocks_dropped: u64 = 0;
    let mut bonus_height = 0;

    let mut rows = VecDeque::new();
    rows.push_back(0b1111111u8);
    while rocks_dropped < count as u64 {
        let r = rocks.next().unwrap();
        // Note that the rock_rows are bottom first
        let mut rock_rows = match r {
            Rock::HorizontalLine => vec![0b0011110u8],
            Rock::Plus => vec![0b0001000u8, 0b0011100u8, 0b0001000u8],
            Rock::LeftL => vec![0b0011100u8, 0b0000100u8, 0b0000100u8],
            Rock::VerticalLine => vec![0b0010000u8, 0b0010000u8, 0b0010000u8, 0b0010000u8],
            Rock::Square => vec![0b0011000u8, 0b0011000u8],
        };

        // We can always go 4 pushes / 3 drops before worrying about hitting another rock
        for _ in 0..4 {
            match push_iter.next().unwrap() {
                JetPush::Left => {
                    if !rock_rows.iter().any(|r| *r & 0b1000000u8 != 0) {
                        rock_rows.iter_mut().for_each(|r| *r = r.shl(1));
                    }
                }
                JetPush::Right => {
                    if !rock_rows.iter().any(|r| *r & 0b0000001u8 != 0) {
                        rock_rows.iter_mut().for_each(|r| *r = r.shr(1));
                    }
                }
            }
        }

        let mut next_drop_index = rows.len().checked_sub(1).unwrap_or(0);
        loop {
            // Check for drop collision
            let mut had_drop_collision = false;
            for i in next_drop_index..rows.len() {
                let rock_row_index = i - next_drop_index;
                if rock_row_index < rock_rows.len() && rows[i] & rock_rows[rock_row_index] != 0 {
                    // collision has happend, stop the rock where it is
                    had_drop_collision = true;
                    break;
                }
            }

            if had_drop_collision {
                for i in 0..rock_rows.len() {
                    if next_drop_index + 1 + i < rows.len() {
                        *rows.get_mut(next_drop_index + 1 + i).unwrap() |= rock_rows[i];
                    } else {
                        rows.push_back(rock_rows[i]);
                    }
                }

                rocks_dropped += 1;

                if rocks_dropped > 1000 && bonus_height == 0 {
                    // If the previous row is settled, lets record this state
                    if rows[next_drop_index] | rows[next_drop_index + 1] == 0b1111111u8 {
                        settled_states.push(SettledState {
                            row_index: next_drop_index,
                            rocks_dropped,
                            height_above_row: rows.len() - next_drop_index,
                            last_rock: r,
                            next_jet_index: push_iter.next_index as usize,
                        });
                    }

                    if settled_states.len() > 2 {
                        if let Some((first_index, last_index)) =
                            check_for_pattern(&settled_states, &rows)
                        {
                            let initial_rocks = settled_states[first_index].rocks_dropped as u64;
                            let repeated_rocks =
                                settled_states[last_index].rocks_dropped as u64 - initial_rocks;
                            let repeated_height = settled_states[last_index].row_index
                                - settled_states[first_index].row_index;
                            let repeat_count = (count as u64 - rocks_dropped) / repeated_rocks;
                            let remaining_count =
                                count as u64 - (repeat_count * repeated_rocks) - rocks_dropped;
                            rocks_dropped = count as u64 - remaining_count;
                            // We already have 1 repeat since we found it
                            bonus_height = (repeat_count as u64) * repeated_height as u64;

                            println!("Current height: {}", rows.len() - 1);
                            println!("Bonus height: {}", bonus_height);
                            println!("Rocks dropped: {}", rocks_dropped);
                            println!("Repeat count: {}", repeat_count);
                            println!("Rocks per repeat: {}", repeated_rocks);
                            println!("Height per repeat: {}", repeated_height);
                        }
                    }
                }

                break;
            }

            // No drop collision so lets see how the jet will move it
            match push_iter.next().unwrap() {
                JetPush::Left => {
                    if !rock_rows.iter().any(|r| *r & 0b1000000u8 != 0)
                        && !rock_rows.iter().enumerate().any(|r| {
                            if next_drop_index + r.0 < rows.len() {
                                return r.1.shl(1) & rows[next_drop_index + r.0] != 0u8;
                            }
                            false
                        })
                    {
                        rock_rows.iter_mut().for_each(|r| *r = r.shl(1));
                    }
                }
                JetPush::Right => {
                    if !rock_rows.iter().any(|r| *r & 0b0000001u8 != 0)
                        && !rock_rows.iter().enumerate().any(|r| {
                            if next_drop_index + r.0 < rows.len() {
                                return r.1.shr(1) & rows[next_drop_index + r.0] != 0u8;
                            }

                            false
                        })
                    {
                        rock_rows.iter_mut().for_each(|r| *r = r.shr(1));
                    }
                }
            }

            next_drop_index -= 1;
        }
    }

    show_tower(&rows, rows.len() - 50);

    // We subtract 1 for the floor we added
    (rows.len() - 1 + bonus_height as usize) as u64
}

fn check_for_pattern(
    settled_states: &Vec<SettledState>,
    rows: &VecDeque<u8>,
) -> Option<(usize, usize)> {
    // We're going to assume the first settled state will always repeat and
    // consider the range from that settled state to the other settled states

    let first_state_start = settled_states[0].row_index;
    let mut i = 2;
    while i < settled_states.len() {
        let current_state_start = settled_states[i].row_index;
        let same_rows_count = rows
            .iter()
            .skip(first_state_start)
            .enumerate()
            .take_while(|(index, r)| {
                current_state_start + index < rows.len() && **r == rows[current_state_start + index]
            })
            .count();

        if first_state_start + same_rows_count == current_state_start
            && settled_states[0].last_rock == settled_states[i].last_rock
            && settled_states[0].next_jet_index == settled_states[i].next_jet_index
            && settled_states[0].height_above_row == settled_states[i].height_above_row
        {
            println!(
                "Start {}, length {}, height above: {}, rocks dropped {} / {}",
                first_state_start,
                same_rows_count,
                settled_states[0].height_above_row,
                settled_states[0].rocks_dropped,
                settled_states[i].rocks_dropped
            );

            return Some((0, i));
        }

        i += 1;
    }

    None
}

fn show_tower(rows: &VecDeque<u8>, starting_depth: usize) {
    for i in (starting_depth..rows.len()).rev() {
        print!("|");
        for b in (0..7).rev() {
            if 1u8.shl(b) & rows[i] == 0u8 {
                print!(".");
            } else {
                print!("#");
            }
        }
        print!("|\n");
    }

    print!("\n----\n\n");
}
