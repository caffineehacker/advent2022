use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("data.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut max_calories_seen = 0;
    let mut current_calories = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            if max_calories_seen < current_calories {
                max_calories_seen = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line
                .parse::<i32>()
                .expect("Failed to parse i32 from string");
        }
    }

    println!("Maximum calories carried by an elf: {}", max_calories_seen);
}
