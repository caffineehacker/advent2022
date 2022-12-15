use clap::Parser;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    enable_graphics: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let rocks: HashSet<(i32, i32)> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(get_rock_squares)
        .flatten()
        .collect();

    let last_rock_y = rocks.iter().map(|s| s.1).max().unwrap();
    let mut occupied_squares = rocks.clone();

    // We can be smart because the next piece of sand will always follow the same path as the previous one
    let mut sand_path = vec![(500, 0)];
    let mut settled_sand_count = 0;
    let mut part1_done = false;
    if args.enable_graphics {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Falling Sand", 800, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut viewport = Rect::new(0, 0, 800, 600);
        let mut scale = 100;
        'running: loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        viewport.x -= 1;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        viewport.x += 1;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        viewport.y -= 1;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => {
                        viewport.y += 1;
                    }
                    Event::MouseWheel { y, .. } => {
                        scale += y * 2;
                    }
                    Event::MouseMotion {
                        mousestate,
                        xrel,
                        yrel,
                        ..
                    } => {
                        if mousestate.left() {
                            viewport.x += (xrel as f32 / (scale as f32 / 100.0)) as i32;
                            viewport.y += (yrel as f32 / (scale as f32 / 100.0)) as i32;
                        }
                    }
                    _ => {}
                }
            }

            canvas.set_viewport(viewport);
            canvas
                .set_scale(scale as f32 / 100.0, scale as f32 / 100.0)
                .expect("Failed to set scale");

            if !sand_path.is_empty() {
                sand_drop(
                    &mut sand_path,
                    &mut occupied_squares,
                    last_rock_y,
                    &mut part1_done,
                    &mut settled_sand_count,
                );
            }

            // Draw rocks
            canvas.set_draw_color(Color::RED);
            canvas
                .draw_points(
                    rocks
                        .iter()
                        .map(|r| Point::new(r.0, r.1))
                        .collect::<Vec<Point>>()
                        .as_slice(),
                )
                .expect("Drawing rocks failed");

            // Draw sand
            canvas.set_draw_color(Color::YELLOW);
            canvas
                .draw_points(
                    occupied_squares
                        .difference(&rocks)
                        .map(|s| Point::new(s.0, s.1))
                        .collect::<Vec<Point>>()
                        .as_slice(),
                )
                .expect("Failed to render sand");

            canvas.present();
        }
    } else {
        while !sand_path.is_empty() {
            sand_drop(
                &mut sand_path,
                &mut occupied_squares,
                last_rock_y,
                &mut part1_done,
                &mut settled_sand_count,
            );
        }

        println!("Part 2 sand count: {}", settled_sand_count);
    }
}

fn sand_drop(
    sand_path: &mut Vec<(i32, i32)>,
    occupied_squares: &mut HashSet<(i32, i32)>,
    last_rock_y: i32,
    part1_done: &mut bool,
    settled_sand_count: &mut i32,
) {
    let test_point = sand_path.last().unwrap();
    if test_point.1 >= last_rock_y {
        if !*part1_done {
            println!("Settled sand count: {}", settled_sand_count);
        }
        *part1_done = true;
    }

    if test_point.1 != last_rock_y + 1
        && !occupied_squares.contains(&(test_point.0, test_point.1 + 1))
    {
        sand_path.push((test_point.0, test_point.1 + 1));
    } else if test_point.1 != last_rock_y + 1
        && !occupied_squares.contains(&(test_point.0 - 1, test_point.1 + 1))
    {
        sand_path.push((test_point.0 - 1, test_point.1 + 1));
    } else if test_point.1 != last_rock_y + 1
        && !occupied_squares.contains(&(test_point.0 + 1, test_point.1 + 1))
    {
        sand_path.push((test_point.0 + 1, test_point.1 + 1));
    } else {
        // Nowhere else to go, occupy this square, pop it off the test point and continue
        occupied_squares.insert(*test_point);
        sand_path.pop();
        *settled_sand_count += 1;
    }
}

fn get_rock_squares(line: String) -> Vec<(i32, i32)> {
    let endpoints: Vec<&str> = line.split(" -> ").collect();
    let mut occupied_squares = Vec::new();
    let (startx, starty) = endpoints[0].split_once(",").unwrap();
    let startx: i32 = startx.parse().unwrap();
    let starty: i32 = starty.parse().unwrap();
    occupied_squares.push((startx, starty));

    for endpoint in endpoints {
        let (x, y) = endpoint.split_once(",").unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        let last = *occupied_squares.last().unwrap();
        let diff = (
            (x - last.0) / (x - last.0).abs().max(1),
            (y - last.1) / (y - last.1).abs().max(1),
        );
        while *occupied_squares.last().unwrap() != (x, y) {
            let last = *occupied_squares.last().unwrap();
            occupied_squares.push((last.0 + diff.0, last.1 + diff.1));
        }
    }

    occupied_squares
}
