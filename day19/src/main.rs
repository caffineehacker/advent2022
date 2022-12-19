use clap::Parser;
use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

struct Blueprint {
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    /// Cost is (ore, clay)
    obsidian_robot_cost: (u32, u32),
    /// Cost is (ore, obsidian)
    geode_robot_cost: (u32, u32),
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let blueprints: Vec<Blueprint> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(parse_blueprint)
        .collect();

    let total_quality: usize = blueprints
        .par_iter()
        .enumerate()
        .map(|(index, blueprint)| (index + 1) * best_geode_count(&blueprint))
        .sum();

    println!("Part 1 total: {}", total_quality);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SearchState {
    geodes: u32,
    geode_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    clay: u32,
    clay_robots: u32,
    ore: u32,
    ore_robots: u32,
}

fn best_geode_count(blueprint: &Blueprint) -> usize {
    let mut search_states = VecDeque::new();
    search_states.push_back((
        SearchState {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
        },
        0,
    ));

    let mut seen_states = HashSet::new();
    let mut terminal_states = Vec::new();
    while !search_states.is_empty() {
        let (state, time) = search_states.pop_front().unwrap();

        if time == 24 {
            //println!("{} / {}", state.geodes, search_states.len());
            terminal_states.push(state);
            continue;
        }

        //println!("{}: {}", state.time, search_states.len());

        let mut new_state = state.clone();
        new_state.clay += new_state.clay_robots;
        new_state.ore += new_state.ore_robots;
        new_state.obsidian += new_state.obsidian_robots;
        new_state.geodes += new_state.geode_robots;
        let new_state = new_state;

        if state.ore >= blueprint.geode_robot_cost.0
            && state.obsidian >= blueprint.geode_robot_cost.1
        {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.geode_robot_cost.0;
            new_state.obsidian -= blueprint.geode_robot_cost.1;
            new_state.geode_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push_back((new_state, time + 1));
            }
        }

        if state.ore >= blueprint.obsidian_robot_cost.0
            && state.clay >= blueprint.obsidian_robot_cost.1
        {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.obsidian_robot_cost.0;
            new_state.clay -= blueprint.obsidian_robot_cost.1;
            new_state.obsidian_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push_back((new_state, time + 1));
            }
        }

        if state.ore >= blueprint.clay_robot_cost {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.clay_robot_cost;
            new_state.clay_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push_back((new_state, time + 1));
            }
        }

        if state.ore >= blueprint.ore_robot_cost {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.ore_robot_cost;
            new_state.ore_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push_back((new_state, time + 1));
            }
        }

        if seen_states.insert(new_state.clone()) {
            search_states.push_back((new_state, time + 1));
        }
    }

    let best_state = terminal_states.iter().max_by_key(|s| s.geodes).unwrap();
    println!(
        "O/C/OB/G: {}/{}/{}/{}",
        best_state.ore, best_state.clay, best_state.obsidian, best_state.geodes
    );
    println!(
        "O/C/OB/G: {}/{}/{}/{}",
        best_state.ore_robots,
        best_state.clay_robots,
        best_state.obsidian_robots,
        best_state.geode_robots
    );
    terminal_states.iter().map(|s| s.geodes).max().unwrap() as usize
}

fn parse_blueprint(input: String) -> Blueprint {
    let components: Vec<&str> = input.split_whitespace().collect();

    Blueprint {
        ore_robot_cost: components[6].parse().unwrap(),
        clay_robot_cost: components[12].parse().unwrap(),
        obsidian_robot_cost: (
            components[18].parse().unwrap(),
            components[21].parse().unwrap(),
        ),
        geode_robot_cost: (
            components[27].parse().unwrap(),
            components[30].parse().unwrap(),
        ),
    }
}
