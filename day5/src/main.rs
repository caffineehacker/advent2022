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
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let header: Vec<&String> = lines.iter().take_while(|line| !line.is_empty()).collect();
    let column_count = header.last().unwrap().split_whitespace().count();
    let mut stacks: Vec<Vec<String>> = vec![Vec::new(); column_count];

    for header_line in header.iter().rev().skip(1) {
        // Each item takes 3 chars with the middle char being the ID.
        // Then there is a space if it is not the last item
        for c in 0..column_count {
            let item = header_line
                .chars()
                .nth((c * 4) + 1)
                .expect("Failed to get item");
            if !item.is_whitespace() {
                stacks.get_mut(c).unwrap().push(item.to_string());
            }
        }
    }

    for c in 0..column_count {
        print!("{}: ", c);
        for item in stacks.get(c).unwrap().iter() {
            print!("{} ", item);
        }
        print!("\n");
    }
}
