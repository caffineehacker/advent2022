use clap::Parser;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
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
                }
                "ls" => {
                    // All lines until a $ line are file listings
                    let current_directory = breadcrumbs.last().unwrap();
                    while index + 1 < lines.len() && !lines.get(index + 1).unwrap().starts_with("$")
                    {
                        index += 1;
                        let (size, name) = lines.get(index).unwrap().split_once(" ").unwrap();
                        if size == "dir" {
                            get_or_create_subdir(current_directory.clone(), name);
                        } else {
                            current_directory.borrow_mut().files.push(SantaFile {
                                name: name.to_string(),
                                size: size.parse().unwrap(),
                            });
                        }
                    }
                }
                &_ => panic!("Unexpected operation"),
            }
        }

        index += 1;
    }

    print_directory(&root.borrow(), 0);
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
