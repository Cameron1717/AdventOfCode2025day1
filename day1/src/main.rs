use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    println!("Advent of code Day 1 script");
    //    let mut combination = Vec::new();
    let mut position = 50;
    let mut final_combo = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            //combination.push(line);
            position = ouroboros(&line, position);
            if position == 0 {
                final_combo += 1;
            }
        }
    }
    println!("final combination: {}", final_combo);
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn ouroboros(input: &String, mut position: i32) -> i32 {
    let (direction, rotations) = input.split_at(1);
    let i_rotations = rotations.parse::<i32>().unwrap_or_default();
    if direction == "L" {
        position -= i_rotations;
    } else {
        position += i_rotations;
    }
    while position < 0 {
        position += 100;
    }
    while position > 99 {
        position -= 100
    }
    position
}
