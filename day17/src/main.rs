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

#[derive(Clone, Copy)]
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
    next_index: u32,
}

impl Iterator for Jet {
    type Item = JetPush;

    fn next(&mut self) -> Option<Self::Item> {
        let push = match self
            .jet_pattern
            .chars()
            .nth(self.next_index as usize)
            .unwrap()
        {
            '<' => JetPush::Left,
            '>' => JetPush::Right,
            _ => panic!("Unexpected push direction"),
        };

        self.next_index = (self.next_index + 1) % self.jet_pattern.len() as u32;

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
    let mut push_iter = Jet {
        jet_pattern: input,
        next_index: 0,
    };
    let rocks = Rocks {
        next_rock: Rock::HorizontalLine,
    };

    let mut rows = VecDeque::new();
    rows.push_back(0b1111111u8);
    rocks.take(2022).for_each(|r| {
        println!("New rock");
        // Note that the rock_rows are bottom first
        let mut rock_rows = match r {
            Rock::HorizontalLine => vec![0b0011110u8],
            Rock::Plus => vec![0b0001000u8, 0b0011100u8, 0b0001000u8],
            Rock::LeftL => vec![0b0011100u8, 0b0000100u8, 0b0000100u8],
            Rock::VerticalLine => vec![0b0010000u8, 0b0010000u8, 0b0010000u8, 0b0010000u8],
            Rock::Square => vec![0b0011000u8, 0b0011000u8],
        };

        // We can always go 4 drops before worrying about hitting another rock
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

                //show_tower(&rows);
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
    });

    println!("Height is {}", rows.len() - 1);
}

fn show_tower(rows: &VecDeque<u8>) {
    let starting_depth = rows.len().checked_sub(10).unwrap_or(0);

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
