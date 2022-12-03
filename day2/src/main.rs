use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    let args = Args::parse();

    let data = fs::read_to_string(args.data_file).expect("Failed to read file");

    let total_score: i32 = data.split('\n').map(score).sum();

    println!("Total score: {}", total_score);
}

fn score(input_line: &str) -> i32 {
    if input_line.is_empty() {
        return 0;
    }

    let (opponent, myself) = input_line.split_at(1);
    let opponent = map_to_enum(opponent);
    let myself = map_to_enum(myself.trim());

    let action_score = myself as i32;
    let victory_score = calculate_victory_score(opponent, myself);

    println!("{} === {}, {}", input_line, action_score, victory_score);

    return action_score + victory_score;
}

fn map_to_enum(encoded: &str) -> Action {
    match encoded {
        "A" => Action::Rock,
        "B" => Action::Paper,
        "C" => Action::Scissors,
        "X" => Action::Rock,
        "Y" => Action::Paper,
        "Z" => Action::Scissors,
        &_ => panic!("Failed to decode"),
    }
}

fn calculate_victory_score(opponent: Action, myself: Action) -> i32 {
    if opponent == myself {
        return 3;
    }
    if opponent as i32 == myself as i32 + 1
        || (opponent == Action::Rock && myself == Action::Scissors)
    {
        return 0;
    }
    6
}
