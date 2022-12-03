use clap::Parser;
use num_traits::FromPrimitive;
use std::fs;

#[macro_use]
extern crate num_derive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(FromPrimitive, Clone, Copy, PartialEq)]
#[repr(i32)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
enum VictoryState {
    Lose = 0,
    Tie = 3,
    Win = 6,
}

fn main() {
    let args = Args::parse();

    let data = fs::read_to_string(args.data_file).expect("Failed to read file");

    let total_score_part1: i32 = data.split('\n').map(score_part1).sum();

    println!("Total score, part 1: {}", total_score_part1);

    let total_score_part2: i32 = data.split('\n').map(score_part2).sum();
    println!("Total score, part 2: {}", total_score_part2);
}

fn score_part2(input_line: &str) -> i32 {
    if input_line.is_empty() {
        return 0;
    }

    let (opponent, victory_state) = input_line.split_at(1);
    let opponent = map_to_enum(opponent);
    let victory_state = decode_victory_state(victory_state.trim());

    let myself = pick_action(opponent, victory_state);
    let action_score = myself as i32;
    let victory_score = victory_state as i32;

    println!("{} === {}, {}", input_line, action_score, victory_score);

    return action_score + victory_score;
}

fn pick_action(opponent: Action, victory_state: VictoryState) -> Action {
    let mut action_id = opponent as i32
        + match victory_state {
            VictoryState::Lose => -1,
            VictoryState::Tie => 0,
            VictoryState::Win => 1,
        };

    if action_id == 0 {
        action_id = 3;
    } else if action_id == 4 {
        action_id = 1;
    }

    FromPrimitive::from_i32(action_id).expect("Failed to convert")
}

fn score_part1(input_line: &str) -> i32 {
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

fn decode_victory_state(encoded: &str) -> VictoryState {
    match encoded {
        "X" => VictoryState::Lose,
        "Y" => VictoryState::Tie,
        "Z" => VictoryState::Win,
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
