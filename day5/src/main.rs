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
    #[arg(long)]
    part: u32,
}

fn main() {
    let args = Args::parse();
    let part_number = args.part;

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

    // Now that we have the initial stacks, lets execute the moves
    // A move is listed as "move N from A to B"
    for line in lines.iter().skip_while(|line| !line.is_empty()).skip(1) {
        let components: Vec<&str> = line.split_whitespace().collect();
        let mut count: usize = components
            .get(1)
            .unwrap()
            .parse()
            .expect("Failed to parse count");
        let from_index: usize = components
            .get(3)
            .unwrap()
            .parse()
            .expect("Failed to parse from");
        let to_index: usize = components
            .get(5)
            .unwrap()
            .parse()
            .expect("Failed to parse to");

        if part_number == 1 {
            while count > 0 {
                count -= 1;
                let item = stacks.get_mut(from_index - 1).unwrap().pop().unwrap();
                stacks.get_mut(to_index - 1).unwrap().push(item);
            }
        } else {
            let from_stack = stacks.get_mut(from_index - 1).unwrap();
            let mut items: Vec<String> = {
                let (_, items) = from_stack.split_at(from_stack.len() - count);
                items.into_iter().map(|item| item.clone()).collect()
            };
            from_stack.truncate(from_stack.len() - count);
            let to_stack = stacks.get_mut(to_index - 1).unwrap();
            to_stack.append(&mut items);
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
