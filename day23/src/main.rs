use clap::Parser;
use console::Term;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    tui: bool,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Elf {
    x: i32,
    y: i32,
}

fn main() {
    let args = Args::parse();

    let mut term = if args.tui {
        let mut term = console::Term::buffered_stdout();
        term.clear_screen();
        term.hide_cursor();
        term.flush();
        Some(term)
    } else {
        None
    };

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut elves: Vec<Elf> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Elf {
                    x: x as i32,
                    y: y as i32,
                })
                .collect::<Vec<Elf>>()
        })
        .flatten()
        .collect();

    let mut part1 = None;
    let mut part2 = None;
    let mut round = 0;
    loop {
        if args.debug {
            println!("Round {}", round);
        }
        // For movement we need to do round % 4 to find the proposed check
        // 0 = N
        // 1 = S
        // 2 = W
        // 3 = E

        let elves_set: HashSet<Elf> = elves.iter().cloned().collect();
        let elves_to_move: Vec<&mut Elf> = elves
            .iter_mut()
            .filter(|e| {
                for x in 0..3 {
                    let x = x - 1;
                    for y in 0..3 {
                        let y = y - 1;
                        if x == 0 && y == 0 {
                            continue;
                        }

                        if elves_set.contains(&Elf {
                            x: e.x + x,
                            y: e.y + y,
                        }) {
                            return true;
                        }
                    }
                }

                false
            })
            .collect();

        let mut elf_proposals: Vec<((i32, i32), &mut Elf)> = elves_to_move
            .into_iter()
            .map(|e| {
                'decision: for i in round..(round + 4) {
                    let i = i % 4;
                    if i == 0 {
                        // North
                        for x in 0..3 {
                            if elves_set.contains(&Elf {
                                x: e.x + x - 1,
                                y: e.y - 1,
                            }) {
                                continue 'decision;
                            }
                        }
                        return Some(((e.x, e.y - 1), e));
                    } else if i == 1 {
                        // South
                        for x in 0..3 {
                            if elves_set.contains(&Elf {
                                x: e.x + x - 1,
                                y: e.y + 1,
                            }) {
                                continue 'decision;
                            }
                        }
                        return Some(((e.x, e.y + 1), e));
                    } else if i == 2 {
                        // West
                        for y in 0..3 {
                            if elves_set.contains(&Elf {
                                x: e.x - 1,
                                y: e.y + y - 1,
                            }) {
                                continue 'decision;
                            }
                        }
                        return Some(((e.x - 1, e.y), e));
                    } else if i == 3 {
                        // East
                        for y in 0..3 {
                            if elves_set.contains(&Elf {
                                x: e.x + 1,
                                y: e.y + y - 1,
                            }) {
                                continue 'decision;
                            }
                        }
                        return Some(((e.x + 1, e.y), e));
                    }
                }

                None
            })
            .filter_map(|e| e)
            .collect();

        let destinations: Vec<(i32, i32)> = elf_proposals.iter().map(|e| e.0).collect();
        for e in elf_proposals.iter_mut() {
            if destinations
                .iter()
                .filter(|d| d.0 == e.0 .0 && d.1 == e.0 .1)
                .count()
                > 1
            {
                // Multiple elves picked this direction
                continue;
            }

            e.1.x = e.0 .0;
            e.1.y = e.0 .1;
        }

        if elf_proposals.is_empty() {
            part2 = Some(round + 1);
            if args.tui {
                tui_board(&elves, &mut term.as_mut().unwrap(), part1, part2, round);
            } else {
                println!(
                    "Part 2: Static configuration happend after {} rounds",
                    round + 1
                );
            }

            break;
        }

        if round == 9 {
            // The final bit is finding the rectangle containing all elves
            // The number of empty squares in the rectangle will be W*H - elves
            let min_x = elves.iter().map(|e| e.x).min().unwrap();
            let max_x = elves.iter().map(|e| e.x).max().unwrap();
            let min_y = elves.iter().map(|e| e.y).min().unwrap();
            let max_y = elves.iter().map(|e| e.y).max().unwrap();

            let empty_squares = (max_x + 1 - min_x) * (max_y + 1 - min_y) - elves.len() as i32;
            if !args.tui || args.debug {
                println!("Part 1: {}", empty_squares);
            }
            part1 = Some(empty_squares);
        }

        if args.debug {
            print_board(&elves);
        } else if args.tui {
            tui_board(&elves, &mut term.as_mut().unwrap(), part1, part2, round);
        }

        round += 1;
    }
}

fn print_board(elves: &Vec<Elf>) {
    let min_x = elves.iter().map(|e| e.x).min().unwrap();
    let max_x = elves.iter().map(|e| e.x).max().unwrap();
    let min_y = elves.iter().map(|e| e.y).min().unwrap();
    let max_y = elves.iter().map(|e| e.y).max().unwrap();

    println!("{}, {} to {}, {}", min_x, min_y, max_x, max_y);

    for y in 0..=(max_y - min_y) {
        let y = y + min_y;
        for x in 0..=(max_x - min_x) {
            let x = x + min_x;
            if elves.iter().any(|e| e.x == x && e.y == y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn tui_board(
    elves: &Vec<Elf>,
    term: &mut Term,
    part1: Option<i32>,
    part2: Option<i32>,
    round: i32,
) {
    let min_x = elves.iter().map(|e| e.x).min().unwrap();
    let max_x = elves.iter().map(|e| e.x).max().unwrap();
    let min_y = elves.iter().map(|e| e.y).min().unwrap();
    let max_y = elves.iter().map(|e| e.y).max().unwrap();

    let (height, width) = term.size();

    term.move_cursor_to(0, 0);
    term.clear_line();
    term.write_line(&format!(
        "Round: {}  Map: {}, {} to {}, {}       Part 1: {}             Part 2: {}",
        round + 1,
        min_x,
        min_y,
        max_x,
        max_y,
        part1.map(|p| p.to_string()).unwrap_or("".to_string()),
        part2.map(|p| p.to_string()).unwrap_or("".to_string())
    ));
    term.move_cursor_to(0, 1);

    for y in 0..=(max_y - min_y) {
        if y >= height as i32 - 1 {
            break;
        }
        term.move_cursor_to(0, y as usize + 1);
        let y = y + min_y;
        for x in 0..=(max_x - min_x) {
            if x >= width as i32 - 1 {
                break;
            }
            let x = x + min_x;
            if elves.iter().any(|e| e.x == x && e.y == y) {
                term.write(b"#");
            } else {
                term.write(b".");
            }
        }
    }

    term.flush();
}
