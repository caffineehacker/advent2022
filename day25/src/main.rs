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

    let total_in_snarf = to_snarf(
        reader
            .lines()
            .map(|line| from_snarf(line.expect("Failed to read line").as_str()))
            .sum(),
    );

    println!("Part 1: {}", total_in_snarf);
}

fn from_snarf(snarf: &str) -> i64 {
    // Snarf is base 5 with an offset
    let chars: Vec<char> = snarf.chars().rev().collect();

    let mut value = 0;
    for i in 0..chars.len() {
        value += match chars[i] {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unexpected snarf digit"),
        } * 5_i64.pow(i as u32);
    }

    println!("{} -> {}", snarf, value.to_string());

    value
}

fn to_snarf(decimal: i64) -> String {
    // Snarf is base 5 with an offset
    // If we figure out base 5 for the value with the added offset for each digit then we can just shift the digits by 2
    // E.g. if we have 15 dec, that would normally be 30 base 5. In snarf that would be
    // 1=0. If we shift those digits up to 0..5 we get 302 base 5 or 77 dec. This offset is adding 2 in the zeros,
    // 10 in the 5's and 50 in the 25's for an offset of 62. 77 - 62 = 15. So we want to take the decimal number, add the offset for each digit we think it will require, then convert and shift digits.

    let mut digits = Vec::new();
    let mut adjusted_value = decimal;
    while adjusted_value > 0 {
        adjusted_value += 2;

        digits.push(adjusted_value % 5);
        adjusted_value = adjusted_value / 5;
    }

    digits.iter().rev().fold("".to_string(), |acc, d| {
        acc + match d {
            4 => "2",
            3 => "1",
            2 => "0",
            1 => "-",
            0 => "=",
            _ => panic!("Unexpected digit"),
        }
    })
}
