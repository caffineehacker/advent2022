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

    let values: Vec<(usize, i32)> = reader
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

    let part1 = move_values(&mut values.clone());

    println!("Part 1: {}", part1);

    let modulo_scale: i32 = 811589153 % (values.len() - 1) as i32;
    let mut part2_values: Vec<(usize, i32)> =
        values.iter().map(|v| (v.0, v.1 * modulo_scale)).collect();
    let mut part2 = 0;
    for _ in 0..10 {
        part2 = move_values(&mut part2_values) as i64;
    }

    part2 = (part2 / modulo_scale as i64) * 811589153;
    println!("Part 2: {}", part2);
}

fn move_values(values: &mut Vec<(usize, i32)>) -> i32 {
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
    values[(zero_index + 1000) % values.len()].1
        + values[(zero_index + 2000) % values.len()].1
        + values[(zero_index + 3000) % values.len()].1
}
