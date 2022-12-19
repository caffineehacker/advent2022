use clap::Parser;
use rayon::prelude::*;
use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
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
        .map(|(index, blueprint)| (index + 1) * best_geode_count(&blueprint, 24))
        .sum();

    println!("Part 1 total: {}", total_quality);

    let best: Vec<usize> = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| best_geode_count(&blueprint, 32))
        .collect();

    println!("Best 3: {}, {}, {}", best[0], best[1], best[2]);
    println!("Part 1 total: {}", best[0] * best[1] * best[2]);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SearchState {
    geodes: u32,
    geode_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    clay: u32,
    clay_robots: u32,
    ore: u32,
    ore_robots: u32,
    remaining_time: u32,
}

impl SearchState {
    fn geode_potential(&self) -> u32 {
        // The potential is the numbe of geodes we already have
        // plus the number of geode robots times the time remaing
        // plus the number of geodes that could be produced by making a new geode robot each minute.
        let geodes = self.geodes
            + (0..self.remaining_time)
                .rev()
                .map(|t| self.geode_robots + t)
                .sum::<u32>();

        geodes
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // First order comparison is the geode potential of the state
        match self.geode_potential().partial_cmp(&other.geode_potential()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        match self.geodes.partial_cmp(&other.geodes) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.geode_robots.partial_cmp(&other.geode_robots) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.obsidian.partial_cmp(&other.obsidian) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.obsidian_robots.partial_cmp(&other.obsidian_robots) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.clay.partial_cmp(&other.clay) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.clay_robots.partial_cmp(&other.clay_robots) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.ore.partial_cmp(&other.ore) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.ore_robots.partial_cmp(&other.ore_robots) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.remaining_time.partial_cmp(&other.remaining_time)
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn best_geode_count(blueprint: &Blueprint, total_time: u32) -> usize {
    let mut search_states = BinaryHeap::new();
    search_states.push(SearchState {
        ore: 0,
        ore_robots: 1,
        clay: 0,
        clay_robots: 0,
        obsidian: 0,
        obsidian_robots: 0,
        geodes: 0,
        geode_robots: 0,
        remaining_time: total_time,
    });

    let mut seen_states = HashSet::new();
    let mut terminal_states = Vec::new();
    while !search_states.is_empty() {
        let state = search_states.pop().unwrap();

        if state.remaining_time == 0 {
            //println!("{} / {}", state.geodes, search_states.len());
            terminal_states.push(state);
            break;
        }

        //println!("{}: {}", state.time, search_states.len());

        let mut new_state = state.clone();
        new_state.clay += new_state.clay_robots;
        new_state.ore += new_state.ore_robots;
        new_state.obsidian += new_state.obsidian_robots;
        new_state.geodes += new_state.geode_robots;
        new_state.remaining_time -= 1;
        let new_state = new_state;

        if state.ore >= blueprint.geode_robot_cost.0
            && state.obsidian >= blueprint.geode_robot_cost.1
        {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.geode_robot_cost.0;
            new_state.obsidian -= blueprint.geode_robot_cost.1;
            new_state.geode_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push(new_state);
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
                search_states.push(new_state);
            }
        }

        if state.ore >= blueprint.clay_robot_cost {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.clay_robot_cost;
            new_state.clay_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push(new_state);
            }
        }

        if state.ore >= blueprint.ore_robot_cost {
            let mut new_state = new_state.clone();
            new_state.ore -= blueprint.ore_robot_cost;
            new_state.ore_robots += 1;

            if seen_states.insert(new_state.clone()) {
                search_states.push(new_state);
            }
        }

        if seen_states.insert(new_state.clone()) {
            search_states.push(new_state);
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
