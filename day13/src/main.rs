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
            if is_in_right_order(&p.0.borrow(), &p.1.borrow()).unwrap_or(true) {
                index + 1
            } else {
                0
            }
        })
        .sum();

    println!("Sum of correct indexes: {}", sum_of_correct);
}

fn is_in_right_order(voa1: &ValueOrArray, voa2: &ValueOrArray) -> Option<bool> {
    if voa1.is_value() && voa2.is_value() {
        if voa1.unwrap_value() < voa2.unwrap_value() {
            return Some(true);
        } else if voa1.unwrap_value() == voa2.unwrap_value() {
            return None;
        }
        return Some(false);
    }

    if voa1.is_value() {
        return is_in_right_order(
            &ValueOrArray::Array(vec![Rc::new(RefCell::new(voa1.clone()))]),
            voa2,
        );
    }

    if voa2.is_value() {
        return is_in_right_order(
            voa1,
            &ValueOrArray::Array(vec![Rc::new(RefCell::new(voa2.clone()))]),
        );
    }

    let mut index = 0;

    while index < voa1.unwrap_array().len() && index < voa2.unwrap_array().len() {
        let sub_result = is_in_right_order(
            &voa1.unwrap_array()[index].borrow(),
            &voa2.unwrap_array()[index].borrow(),
        );

        if !sub_result.is_none() {
            return sub_result;
        }

        index += 1;
    }

    if voa1.unwrap_array().len() < voa2.unwrap_array().len() {
        return Some(true);
    } else if voa1.unwrap_array().len() > voa2.unwrap_array().len() {
        return Some(false);
    }

    None
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
