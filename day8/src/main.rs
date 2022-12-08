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
    let data: Vec<Vec<i16>> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i16>().expect("Failed to parse value"))
                .collect()
        })
        .collect();

    // Part 1 is how many trees are visible from the edge
    let left_visible_table = get_left_visible(&data);
    let right_visible_table = get_right_visible(&data);
    let top_visible_table = transpose(&get_left_visible(&transpose(&data)));
    let bottom_visible_table = transpose(&get_right_visible(&transpose(&data)));

    let tables = vec![
        &left_visible_table,
        &right_visible_table,
        &top_visible_table,
        &bottom_visible_table,
    ];

    // Now zip the above tables to get the total number of visible trees
    let number_of_visibile_trees: usize = (0..left_visible_table.len())
        .map(|x| {
            let x = x;
            (0..left_visible_table[0].len())
                .map(|y| tables.iter().any(|t| t[x][y]))
                .filter(|v| *v)
                .count()
        })
        .sum();

    println!("Number of visible trees: {}", number_of_visibile_trees);

    // Part 2 is finding the highest scenic score
    let left_scenic_table = get_left_scenic(&data);
    let right_scenic_table = get_left_scenic(
        &data
            .iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect(),
    )
    .iter()
    .map(|row| row.iter().rev().cloned().collect())
    .collect();
    let top_scenic_table = transpose(&get_left_scenic(&transpose(&data)));
    let bottom_scenic_table = transpose(
        &get_left_scenic(
            &transpose(&data)
                .iter()
                .map(|row| row.iter().rev().cloned().collect())
                .collect(),
        )
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect(),
    );
    let scenic_tables = vec![
        &left_scenic_table,
        &right_scenic_table,
        &top_scenic_table,
        &bottom_scenic_table,
    ];
    let best_scenary_score = (0..left_scenic_table.len())
        .flat_map(|x| {
            let x = &x;
            (0..left_scenic_table[0].len())
                .map(|y| {
                    let x = *x;
                    scenic_tables
                        .iter()
                        .map(|t| t[x][y])
                        .reduce(|acc, score| acc * score)
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();

    println!("Best scenic score: {}", best_scenary_score);
}

fn get_left_scenic(data: &Vec<Vec<i16>>) -> Vec<Vec<usize>> {
    data.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(index, tree)| {
                    if index == 0 {
                        return 0;
                    }

                    let visible_count = row[0..index]
                        .iter()
                        .rev()
                        .take_while(|t| **t < *tree)
                        .count();
                    if visible_count != index {
                        return visible_count + 1;
                    }
                    visible_count
                })
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn get_left_visible(data: &Vec<Vec<i16>>) -> Vec<Vec<bool>> {
    data.iter()
        .map(|row| {
            row.iter()
                .scan(-1, |tallest, tree| {
                    if *tallest < *tree {
                        *tallest = *tree;
                        Some(true)
                    } else {
                        Some(false)
                    }
                })
                .collect()
        })
        .collect()
}

fn get_right_visible(data: &Vec<Vec<i16>>) -> Vec<Vec<bool>> {
    data.iter()
        .map(|row| {
            row.iter()
                .rev()
                .scan(-1, |tallest, tree| {
                    if *tallest < *tree {
                        *tallest = *tree;
                        Some(true)
                    } else {
                        Some(false)
                    }
                })
                .collect::<Vec<bool>>()
                .iter()
                .rev()
                .cloned()
                .collect()
        })
        .collect()
}

fn transpose<T>(data: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..data[0].len())
        .map(|i| {
            data.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<T>>()
        })
        .collect()
}
