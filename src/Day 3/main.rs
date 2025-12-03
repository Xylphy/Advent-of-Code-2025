use std::fs::File;
use std::io::Read;
use std::vec::Vec;

fn find_largest_joltage_from_k(line: &String, k: usize) -> usize {
    let bytes: &[u8] = line.as_bytes();
    let mut to_drop: usize = bytes.len() - k;
    let mut stack: Vec<u8> = Vec::with_capacity(bytes.len());

    for &b in bytes {
        let digit: u8 = b - b'0';

        while to_drop > 0 && !stack.is_empty() && stack[stack.len() - 1] < digit {
            stack.pop();
            to_drop -= 1;
        }

        stack.push(digit);
    }

    // keep only first k digits
    stack.truncate(k);

    // turn digits into a u64
    stack
        .into_iter()
        .fold(0usize, |acc: usize, d: u8| acc * 10 + d as usize)
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line: &String| find_largest_joltage_from_k(line, 2) as usize)
        .sum()
}

fn part2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line: &String| find_largest_joltage_from_k(line, 12) as usize)
        .sum()
}

fn main() {
    let lines: Vec<String> = match File::open("src/Day 3/input.txt") {
        Ok(mut file) => {
            let mut contents: String = String::new();
            file.read_to_string(&mut contents)
                .expect("Error reading file");

            contents
                .lines()
                .map(|s: &str| s.to_string())
                .collect::<Vec<String>>()
        }
        Err(e) => {
            println!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "Sum of max joltage from each bank (part 1): {}",
        part1(&lines)
    );
    print!(
        "Sum of max joltage from each bank (part 2): {}",
        part2(&lines)
    );
}
