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

#[derive(Default)]
struct VirtualMachine {
    instructions: Vec<String>,
    halfway_in_instruction: bool,
    index: usize,
    cycle: usize,
    x: i32,
}

impl Iterator for VirtualMachine {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;
        if self.index >= self.instructions.len() {
            return None;
        }

        let instruction = &self.instructions[self.index];
        if instruction.trim() == "noop" {
            self.index += 1;
            return Some(self.x);
        }

        if self.halfway_in_instruction {
            self.halfway_in_instruction = false;
            self.index += 1;

            let increment_value: i32 = instruction
                .split_once(' ')
                .unwrap()
                .1
                .parse()
                .expect("Failed to parse increment value");

            self.x += increment_value;
            return Some(self.x);
        }

        self.halfway_in_instruction = true;
        Some(self.x)
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vm = VirtualMachine {
        instructions: reader
            .lines()
            .map(|line| line.expect("Failed to parse line"))
            .collect(),
        x: 1,
        ..Default::default()
    };

    // The text is odd because we want the X value DURING the cycle which is actually the value after the previous cycle.
    vm.nth(17);
    let mut total = 0;
    for i in 0..6 {
        let result = vm.next();
        vm.nth(38);
        total += result.unwrap() * ((i * 40) + 20);
    }

    // 19, 59
    // 19 + 39 + 1 = 59

    println!("Total power is {}", total);
}
