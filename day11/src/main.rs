use clap::Parser;
use std::{
    cell::RefCell,
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

struct Monkey {
    number: u32,
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test_value: u32,
    true_destination: u32,
    false_destination: u32,
    item_inspection_count: u32,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines().peekable();

    let mut monkeys = HashMap::new();
    // Parsing is FUN
    while lines.peek().is_some() {
        // First line is "Monkey #:"
        let monkey_number: u32 = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_once(" ")
            .unwrap()
            .1
            .trim_end_matches(":")
            .parse()
            .expect("Failed to parse monkey number");
        let items: Vec<u32> = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let operation = create_operation_fn(
            lines
                .next()
                .unwrap()
                .expect("Failed to parse line")
                .split_once("new = ")
                .unwrap()
                .1,
        );
        let test_value = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .expect("Failed to parse test number");
        let true_destination = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .expect("Failed to parse true number");
        let false_destination = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .expect("Failed to parse false number");

        monkeys.insert(
            monkey_number,
            RefCell::new(Monkey {
                number: monkey_number,
                items,
                operation,
                test_value,
                true_destination,
                false_destination,
                item_inspection_count: 0,
            }),
        );

        // Eat the newline if present
        lines.next();
    }

    for _ in 0..20 {
        let mut monkeys = monkeys.values().collect::<Vec<&RefCell<Monkey>>>();
        monkeys.sort_by_key(|m| m.borrow().number);
        for monkey in monkeys.iter() {
            let mut monkey = monkey.borrow_mut();
            for item in monkey.items.iter() {
                let item = (monkey.operation)(*item) / 3;
                if item % monkey.test_value == 0 {
                    monkeys
                        .get(monkey.true_destination as usize)
                        .unwrap()
                        .borrow_mut()
                        .items
                        .push(item);
                } else {
                    monkeys
                        .get(monkey.false_destination as usize)
                        .unwrap()
                        .borrow_mut()
                        .items
                        .push(item);
                }
            }

            monkey.item_inspection_count += monkey.items.len() as u32;
            monkey.items.clear();
        }
    }

    let mut monkeys = monkeys.values().collect::<Vec<&RefCell<Monkey>>>();
    monkeys.sort_by(|a, b| {
        (*b).borrow()
            .item_inspection_count
            .cmp(&(*a).borrow().item_inspection_count)
    });

    println!(
        "Monkey business: {}",
        monkeys[0].borrow().item_inspection_count * monkeys[1].borrow().item_inspection_count
    );
}

fn create_operation_fn(input: &str) -> Box<dyn Fn(u32) -> u32> {
    let (first_value, input) = input.split_once(" ").unwrap();
    let (operator, second_value) = input.split_once(" ").unwrap();

    if first_value != "old" {
        panic!("Always expect first value to be old");
    }

    let second_value = if second_value == "old" {
        None
    } else {
        Some(
            second_value
                .parse::<u32>()
                .expect("Failed to parse operation param"),
        )
    };

    match operator {
        "*" => Box::new(move |old_value| {
            old_value
                * match second_value {
                    Some(x) => x,
                    None => old_value,
                }
        }),
        "+" => Box::new(move |old_value| {
            old_value
                + match second_value {
                    Some(x) => x,
                    None => old_value,
                }
        }),
        _ => panic!("Unexpected operator"),
    }
}
