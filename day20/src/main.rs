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

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut values: Vec<(usize, i32)> = reader
        .lines()
        .enumerate()
        .map(|(index, line)| {
            (
                index,
                line.expect("Failed to read line")
                    .parse::<i32>()
                    .expect("Failed to parse value"),
            )
        })
        .collect();

    for i in 0..=values.iter().max_by_key(|v| v.0).unwrap().0 {
        let i = values.iter().enumerate().find(|(_, v)| v.0 == i).unwrap().0;
        let value = values[i];
        let mut next_index = i as i32 + value.1;
        while next_index < 0 {
            next_index = values.len() as i32 - 1 + next_index;
        }

        next_index = next_index % (values.len() - 1) as i32;
        values.remove(i);
        values.insert(next_index as usize, value);
    }

    let zero_index = values.iter().enumerate().find(|(_, v)| v.1 == 0).unwrap().0;
    let sum_of_offset_values = values[(zero_index + 1000) % values.len()].1
        + values[(zero_index + 2000) % values.len()].1
        + values[(zero_index + 3000) % values.len()].1;

    println!("Part 1: {}", sum_of_offset_values);
}
