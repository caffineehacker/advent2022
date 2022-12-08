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

struct SantaFile {
    name: String,
    size: u32,
}

#[derive(Default)]
struct Directory {
    name: String,
    sub_directories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<SantaFile>,
    size: u32,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let root = Rc::new(RefCell::new(Directory {
        name: "/".to_string(),
        ..Default::default()
    }));
    let mut breadcrumbs = vec![root.clone()];

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut index = 0;

    while index < lines.len() {
        let line = lines.get(index).unwrap();
        if line.starts_with("$") {
            let components: Vec<&str> = line.split_whitespace().collect();
            match *components.get(1).unwrap() {
                "cd" => {
                    let directory_name = components.get(2).unwrap();
                    if *directory_name == "/" {
                        breadcrumbs.clear();
                        breadcrumbs.push(root.clone());
                    } else if *directory_name == ".." {
                        breadcrumbs.pop();
                    } else {
                        let current_directory = breadcrumbs.last().unwrap();
                        breadcrumbs.push(get_or_create_subdir(
                            current_directory.clone(),
                            directory_name,
                        ));
                    }

                    // println!(
                    //     "Entering {}",
                    //     breadcrumbs
                    //         .iter()
                    //         .map(|d| d.borrow().name.clone())
                    //         .reduce(|acc, d| acc + "/" + &d)
                    //         .unwrap()
                    // );
                }
                "ls" => {
                    // All lines until a $ line are file listings
                    let current_directory = breadcrumbs.last().unwrap().clone();
                    while index + 1 < lines.len() && !lines.get(index + 1).unwrap().starts_with("$")
                    {
                        index += 1;
                        let (size, name) = lines.get(index).unwrap().split_once(" ").unwrap();
                        if size == "dir" {
                            get_or_create_subdir(current_directory.clone(), name);
                        } else {
                            let file_size: u32 = size.parse().unwrap();
                            current_directory.borrow_mut().files.push(SantaFile {
                                name: name.to_string(),
                                size: file_size,
                            });
                            breadcrumbs
                                .iter_mut()
                                .for_each(|b| b.borrow_mut().size += file_size);
                        }
                    }
                }
                &_ => panic!("Unexpected operation"),
            }
        }

        index += 1;
    }

    print_directory(&root.borrow(), 0);
    let total_under_limit: u32 = get_all_directories(root)
        .iter()
        .map(|d| d.borrow().size)
        .filter(|s| *s <= 100000)
        .sum();
    println!("Sum of directories <= 100000: {}", total_under_limit);
}

fn get_all_directories(root: Rc<RefCell<Directory>>) -> Vec<Rc<RefCell<Directory>>> {
    let mut results = Vec::new();
    let mut to_process = vec![root];

    while !to_process.is_empty() {
        let directory = to_process.pop().unwrap();
        to_process.append(&mut directory.borrow().sub_directories.clone());
        results.push(directory.clone());
    }

    results
}

fn print_directory(directory: &Directory, space_depth: u32) {
    for _ in 0..space_depth {
        print!(" ");
    }
    print!("- {} (dir)\n", directory.name);
    for subdir in directory.sub_directories.iter() {
        print_directory(&subdir.borrow(), space_depth + 2);
    }

    for file in directory.files.iter() {
        for _ in 0..(space_depth + 2) {
            print!(" ");
        }
        print!("- {} (file, size={})\n", file.name, file.size);
    }
}

fn get_or_create_subdir(
    parent_directory: Rc<RefCell<Directory>>,
    subdir_name: &str,
) -> Rc<RefCell<Directory>> {
    let subdir = parent_directory
        .borrow()
        .sub_directories
        .iter()
        .find(|dir| dir.borrow().name == subdir_name)
        .cloned();

    if let Some(subdir) = subdir {
        return subdir.clone();
    }

    let new_dir = Rc::new(RefCell::new(Directory {
        name: subdir_name.to_string(),
        ..Default::default()
    }));
    parent_directory
        .borrow_mut()
        .sub_directories
        .push(new_dir.clone());

    new_dir
}
