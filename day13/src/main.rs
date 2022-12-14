use clap::Parser;
use std::{
    cell::RefCell,
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
enum ValueOrArray {
    Value(u8),
    Array(Vec<Rc<RefCell<ValueOrArray>>>),
}

impl ValueOrArray {
    fn is_value(&self) -> bool {
        match self {
            ValueOrArray::Value(_) => true,
            _ => false,
        }
    }

    fn unwrap_array(&self) -> &Vec<Rc<RefCell<ValueOrArray>>> {
        match self {
            ValueOrArray::Array(arr) => arr,
            _ => panic!("Tried to get the array of a non-array type"),
        }
    }

    fn unwrap_array_mut(&mut self) -> &mut Vec<Rc<RefCell<ValueOrArray>>> {
        match self {
            ValueOrArray::Array(arr) => arr,
            _ => panic!("Tried to get the array of a non-array type"),
        }
    }

    fn unwrap_value(&self) -> &u8 {
        match self {
            ValueOrArray::Value(v) => v,
            _ => panic!("Tried to get the value of a non-value type"),
        }
    }
}

impl Ord for ValueOrArray {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_value() && other.is_value() {
            return self.unwrap_value().cmp(other.unwrap_value());
        }

        if self.is_value() {
            return ValueOrArray::Array(vec![Rc::new(RefCell::new(self.clone()))]).cmp(other);
        }

        if other.is_value() {
            return self.cmp(&ValueOrArray::Array(vec![Rc::new(RefCell::new(
                other.clone(),
            ))]));
        }

        let mut index = 0;

        while index < self.unwrap_array().len() && index < other.unwrap_array().len() {
            let sub_result = self.unwrap_array()[index]
                .borrow()
                .cmp(&other.unwrap_array()[index].borrow());

            if sub_result != std::cmp::Ordering::Equal {
                return sub_result;
            }

            index += 1;
        }

        if self.unwrap_array().len() < other.unwrap_array().len() {
            return std::cmp::Ordering::Less;
        } else if self.unwrap_array().len() > other.unwrap_array().len() {
            return std::cmp::Ordering::Greater;
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for ValueOrArray {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ValueOrArray {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for ValueOrArray {}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut pairs = Vec::new();
    loop {
        let line1 = lines.next();
        if line1.is_none() {
            break;
        }
        let line1 = line1.unwrap().expect("Failed to parse");
        let line2 = lines.next().unwrap().expect("Failed to parse");

        pairs.push((parse_line(&line1), parse_line(&line2)));
        lines.next(); // Eat the empty line
    }

    let sum_of_correct: usize = pairs
        .iter()
        .enumerate()
        .map(|(index, p)| {
            if *p.0.borrow() <= *p.1.borrow() {
                index + 1
            } else {
                0
            }
        })
        .sum();

    println!("Sum of correct indexes (part 1): {}", sum_of_correct);

    // Part 2
    let mut all_entries: Vec<Rc<RefCell<ValueOrArray>>> = pairs
        .iter()
        .flat_map(|p| vec![p.0.clone(), p.1.clone()])
        .collect();
    let id1 = parse_line("[[2]]");
    let id2 = parse_line("[[6]]");
    all_entries.push(id1.clone());
    all_entries.push(id2.clone());

    all_entries.sort();
    let index_id1 = all_entries
        .iter()
        .enumerate()
        .find(|e| *e.1 == id1)
        .unwrap()
        .0
        + 1;

    let index_id2 = all_entries
        .iter()
        .enumerate()
        .find(|e| *e.1 == id2)
        .unwrap()
        .0
        + 1;

    println!("Decoder key (part 2): {}", index_id1 * index_id2);
}

fn parse_line(line: &str) -> Rc<RefCell<ValueOrArray>> {
    let mut array_stack = Vec::new();
    let root = Rc::new(RefCell::new(ValueOrArray::Array(Vec::new())));
    let mut index = 0;
    let chars: Vec<char> = line.chars().collect();
    while index < chars.len() {
        match chars[index] {
            '[' => {
                if array_stack.len() == 0 {
                    array_stack.push(root.clone());
                } else {
                    let new_arr = Rc::new(RefCell::new(ValueOrArray::Array(Vec::new())));

                    let current_array = array_stack.last_mut().unwrap();
                    current_array
                        .borrow_mut()
                        .unwrap_array_mut()
                        .push(new_arr.clone());

                    array_stack.push(new_arr);
                }
            }
            ']' => {
                array_stack.pop();
            }
            '0'..='9' => {
                let mut number_end_index = index + 1;
                while number_end_index < chars.len() {
                    match chars[number_end_index] {
                        '0'..='9' => {
                            number_end_index += 1;
                        }
                        _ => {
                            break;
                        }
                    };
                }

                let value: u8 = chars[index..number_end_index]
                    .iter()
                    .fold("".to_string(), |acc, c| acc + &c.to_string())
                    .parse()
                    .expect("Failed to parse number");

                let current_array = array_stack.last_mut().unwrap();
                current_array
                    .borrow_mut()
                    .unwrap_array_mut()
                    .push(Rc::new(RefCell::new(ValueOrArray::Value(value))));
            }
            ',' => (),
            _ => panic!("Unexpected char"),
        }

        index += 1;
    }

    root
}
