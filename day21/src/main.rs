use clap::Parser;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

enum Operand {
    Monkey(String),
    Value(i64),
}

impl Operand {
    fn unwrap_value(&self) -> i64 {
        match self {
            Operand::Value(v) => *v,
            _ => panic!("Not a value"),
        }
    }

    fn unwrap_monkey(&self) -> &str {
        match self {
            Operand::Monkey(v) => v,
            _ => panic!("Not a monkey"),
        }
    }
}

enum Operation {
    Add(Operand, Operand),
    Subtract(Operand, Operand),
    Multiply(Operand, Operand),
    Divide(Operand, Operand),
    Immediate(Operand),
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let monkeys: HashMap<String, Operation> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|line| {
            let components: Vec<&str> = line.split_whitespace().collect();
            let monkey_name = components[0].trim_end_matches(":");
            let operation = if components.len() == 2 {
                Operation::Immediate(Operand::Value(components[1].parse().unwrap()))
            } else {
                let operand1 = Operand::Monkey(components[1].to_string());
                let operand2 = Operand::Monkey(components[3].to_string());
                match components[2] {
                    "+" => Operation::Add(operand1, operand2),
                    "-" => Operation::Subtract(operand1, operand2),
                    "*" => Operation::Multiply(operand1, operand2),
                    "/" => Operation::Divide(operand1, operand2),
                    _ => panic!("Unexpected operation"),
                }
            };

            (monkey_name.to_string(), operation)
        })
        .collect();

    let result = solve_monkey("root", &monkeys);
    println!("Part 1: {}", result);
}

fn solve_monkey(monkey: &str, monkeys: &HashMap<String, Operation>) -> i64 {
    match &monkeys[monkey] {
        Operation::Add(o1, o2) => {
            solve_monkey(o1.unwrap_monkey(), &monkeys) + solve_monkey(o2.unwrap_monkey(), &monkeys)
        }
        Operation::Subtract(o1, o2) => {
            solve_monkey(o1.unwrap_monkey(), &monkeys) - solve_monkey(o2.unwrap_monkey(), &monkeys)
        }
        Operation::Multiply(o1, o2) => {
            solve_monkey(o1.unwrap_monkey(), &monkeys) * solve_monkey(o2.unwrap_monkey(), &monkeys)
        }
        Operation::Divide(o1, o2) => {
            solve_monkey(o1.unwrap_monkey(), &monkeys) / solve_monkey(o2.unwrap_monkey(), &monkeys)
        }
        Operation::Immediate(o1) => o1.unwrap_value(),
    }
}
