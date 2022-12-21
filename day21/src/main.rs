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

#[derive(Clone)]
enum Operand {
    Monkey(String),
    Value(f64),
}

impl Operand {
    fn unwrap_value(&self) -> f64 {
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

#[derive(Clone)]
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

    // Part 2 gets a bit more interesting...
    let part2 = solve_part2(&monkeys);
    println!("Part 2: {}", part2);
}

fn solve_monkey(monkey: &str, monkeys: &HashMap<String, Operation>) -> f64 {
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

fn solve_part2(monkeys: &HashMap<String, Operation>) -> f64 {
    // Start with a large change to detect differences
    let mut changing_value = 10000.0;
    let mut humn1_monkeys = monkeys.clone();
    *humn1_monkeys.get_mut("humn").unwrap() = Operation::Immediate(Operand::Value(1.0));
    let mut humn2_monkeys = monkeys.clone();
    *humn2_monkeys.get_mut("humn").unwrap() = Operation::Immediate(Operand::Value(changing_value));

    let (root1, root2) = match &monkeys["root"] {
        Operation::Add(r1, r2) => (r1.unwrap_monkey(), r2.unwrap_monkey()),
        Operation::Subtract(r1, r2) => (r1.unwrap_monkey(), r2.unwrap_monkey()),
        Operation::Multiply(r1, r2) => (r1.unwrap_monkey(), r2.unwrap_monkey()),
        Operation::Divide(r1, r2) => (r1.unwrap_monkey(), r2.unwrap_monkey()),
        Operation::Immediate(_) => panic!("Not expecting immediate value"),
    };

    let solution1_r1 = solve_monkey(root1, &humn1_monkeys);
    let solution1_r2 = solve_monkey(root2, &humn1_monkeys);

    let solution2_r1 = solve_monkey(root1, &humn2_monkeys);
    let solution2_r2 = solve_monkey(root2, &humn2_monkeys);

    // We only care about dialing in the monkey that changed
    let changing_monkey = if solution1_r1 == solution2_r1 {
        root2
    } else {
        root1
    };
    let target_value = if solution1_r1 == solution2_r1 {
        solution1_r1
    } else {
        solution1_r2
    };

    // Binary search for the solution

    let mut magnitude = 100000000000.0;
    changing_value = 1.0;
    *humn2_monkeys.get_mut("humn").unwrap() = Operation::Immediate(Operand::Value(changing_value));
    let mut current_value = solve_monkey(changing_monkey, &humn2_monkeys);
    let mut last_diff_direction =
        (current_value - target_value) / (current_value - target_value).abs();
    while current_value != target_value {
        if current_value > target_value && last_diff_direction < 0.0 {
            magnitude = -magnitude / 2.0;
        } else if current_value < target_value && last_diff_direction > 0.0 {
            magnitude = -magnitude / 2.0;
        }

        changing_value += magnitude;
        *humn2_monkeys.get_mut("humn").unwrap() =
            Operation::Immediate(Operand::Value(changing_value));
        last_diff_direction = (current_value - target_value) / (current_value - target_value).abs();
        current_value = solve_monkey(changing_monkey, &humn2_monkeys);
    }

    changing_value
}
