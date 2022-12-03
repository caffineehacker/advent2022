use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

struct Ration {
    calories: i32,
}

#[derive(Default)]
struct Elf {
    rations: Vec<Ration>,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut current_elf = Elf::default();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Elf::default();
        } else {
            current_elf.rations.push(Ration {
                calories: line
                    .parse::<i32>()
                    .expect("Failed to parse i32 from string"),
            });
        }
    }
    if !current_elf.rations.is_empty() {
        elves.push(current_elf);
    }

    elves.sort_unstable_by_key(|elf| -1 * elf.rations.iter().map(|r| r.calories).sum::<i32>());

    println!(
        "Calories carried by top 3 elves: {}, {}, {}",
        elves
            .get(0)
            .unwrap()
            .rations
            .iter()
            .map(|r| r.calories)
            .sum::<i32>(),
        elves
            .get(1)
            .unwrap()
            .rations
            .iter()
            .map(|r| r.calories)
            .sum::<i32>(),
        elves
            .get(2)
            .unwrap()
            .rations
            .iter()
            .map(|r| r.calories)
            .sum::<i32>()
    );
}
