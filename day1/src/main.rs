use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    println!("Advent of code Day 1 script");
    let mut position = 50;
    let mut final_combo = 0;
    let mut zero_counter = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            (position, zero_counter) = ouroboros(&line, position, zero_counter);
            if position == 0 {
                final_combo += 1;
            }
        }
    };
    println!("final combination: {}", final_combo);
    println!("Part 2 answer {}", zero_counter)
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn ouroboros(input: &String, mut position: i32, mut zero_counter: i32) -> (i32, i32) {
    let (direction, rotations) = input.split_at(1);
    let val = rotations.parse::<i32>().unwrap_or_default();

    let full = val / 100;
    let partial = val % 100;
    zero_counter += full;

    let delta = if direction == "L" { -partial } else { partial };
    let next_position = position + delta;

    if position != 0 {
        if direction == "L" && next_position <= 0 {
            zero_counter += 1;
        } else if direction == "R" && next_position >= 100 {
            zero_counter += 1;
        }
    }

    position = next_position % 100;
    if position < 0 {
        position += 100;
    }

    (position, zero_counter)
}
