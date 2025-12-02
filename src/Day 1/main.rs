use std::fs::File;
use std::io::prelude::Read;

fn count_zero_during_pass(lines: &Vec<String>) -> usize {
    let mut count: usize = 0;
    let mut start: isize = 50;

    for line in lines {
        let direction: char = line.chars().next().unwrap();
        let mut step: usize = line[1..].trim().parse().unwrap();
        let initial_start = start;

        count += step / 100;
        step %= 100;

        if direction == 'L' {
            start += step as isize;
            if start > 100 && initial_start != 0 {
                count += 1;
            }
        } else {
            start -= step as isize;

            if start < 0 && initial_start != 0 {
                count += 1;
            }

            start += 100;
        }

        start %= 100;

        if start == 0 {
            count += 1;
        }
    }

    count
}

fn count_start_at_zero(lines: &Vec<String>) -> usize {
    let mut count: usize = 0;
    let mut start: isize = 50;

    for line in lines {
        let direction: char = line.chars().next().unwrap();
        let step: usize = line[1..].trim().parse().unwrap();

        if direction == 'L' {
            start = (start + step as isize).rem_euclid(100);
        } else {
            start = (start - step as isize).rem_euclid(100);
        }

        if start == 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut contents: String = String::new();
    let mut lines = Vec::new();

    match File::open("src/Day 1/input.txt") {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => {
                lines = contents
                    .lines()
                    .map(|l: &str| l.trim().to_string())
                    .filter(|l: &String| !l.is_empty())
                    .collect();
            }
            Err(e) => {
                println!("Error reading file: {}", e);
            }
        },

        Err(e) => {
            println!("Error opening file: {}", e);
        }
    }

    println!("Rotations that stop at 0: {}", count_start_at_zero(&lines));
    println!(
        "Rotations that pass through 0: {}",
        count_zero_during_pass(&lines)
    );
}
