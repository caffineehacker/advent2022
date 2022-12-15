use clap::Parser;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(Clone)]
struct Monkey {
    number: u64,
    items: Vec<u64>,
    operation: Rc<Box<dyn Fn(u64) -> u64>>,
    test_value: u64,
    true_destination: u64,
    false_destination: u64,
    item_inspection_count: u64,
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
        let monkey_number: u64 = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_once(" ")
            .unwrap()
            .1
            .trim_end_matches(":")
            .parse()
            .expect("Failed to parse monkey number");
        let items: Vec<u64> = lines
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
            .parse::<u64>()
            .expect("Failed to parse test number");
        let true_destination = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .expect("Failed to parse true number");
        let false_destination = lines
            .next()
            .unwrap()
            .expect("Failed to parse line")
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
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

    calculate_monkey_business(monkeys.clone(), 20, 3);
    calculate_monkey_business(monkeys.clone(), 10000, 1);
}

fn calculate_monkey_business(
    monkeys: HashMap<u64, RefCell<Monkey>>,
    loops: usize,
    worriness_divider: u64,
) {
    let mut monkeys = monkeys;
    for _ in 0..loops {
        conduct_pass(&mut monkeys, worriness_divider);
    }

    let mut monkeys = monkeys.values().collect::<Vec<&RefCell<Monkey>>>();
    monkeys.sort_by(|a, b| {
        (*b).borrow()
            .item_inspection_count
            .cmp(&(*a).borrow().item_inspection_count)
    });

    println!(
        "Monkey business after {} rounds: {}",
        loops,
        monkeys[0].borrow().item_inspection_count as u64
            * monkeys[1].borrow().item_inspection_count as u64
    );
}

fn conduct_pass(monkeys: &mut HashMap<u64, RefCell<Monkey>>, worriness_divider: u64) {
    let modulo = monkeys
        .values()
        .fold(1, |acc, m| acc * m.borrow().test_value);
    let mut monkeys = monkeys.values().collect::<Vec<&RefCell<Monkey>>>();
    monkeys.sort_by_key(|m| m.borrow().number);
    for monkey in monkeys.iter() {
        let mut monkey = monkey.borrow_mut();
        for item in monkey.items.iter() {
            let item = (monkey.operation)(*item) / worriness_divider;
            let item = item % modulo;
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

        monkey.item_inspection_count += monkey.items.len() as u64;
        monkey.items.clear();
    }
}

fn create_operation_fn(input: &str) -> Rc<Box<dyn Fn(u64) -> u64>> {
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
                .parse::<u64>()
                .expect("Failed to parse operation param"),
        )
    };

    Rc::new(match operator {
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
    })
}
