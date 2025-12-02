use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

struct InvalidIDRange {
    start: String,
    end: String,
}

fn sum_invalid_ids(ranges: &Vec<InvalidIDRange>) -> usize {
    let mut sum: usize = 0;

    for range in ranges {
        let max_len: usize = range.end.len();
        let low: usize = range
            .start
            .parse::<usize>()
            .expect("Failed to parse start integer");
        let high: usize = range
            .end
            .parse::<usize>()
            .expect("Failed to parse end integer");

        for i in 2..=max_len {
            if i % 2 == 1 {
                continue;
            }

            let half: usize = i / 2;
            let multiplier: usize = 10_usize.pow(half as u32) + 1;

            // smallest s so that s * (10^half + 1) >= low
            let s_min: usize = (low + (multiplier - 1)) / multiplier;

            // largest s so that s * (10^half + 1) <= high
            let s_max: usize = high / multiplier;
            let lower_bound: usize = 10_usize.pow((half - 1) as u32); // ensure s has the correct number of digits

            for s in std::cmp::max(s_min, lower_bound)..=s_max {
                let s_str: String = s.to_string();
                if s_str.len() != half {
                    continue;
                }
                let candidate: usize = format!("{}{}", s_str, s_str)
                    .parse::<usize>()
                    .expect("Failed to parse candidate integer");

                if candidate >= low && candidate <= high {
                    sum += candidate;
                }
            }
        }
    }

    sum
}

fn sum_modified_invalid_id(ranges: &Vec<InvalidIDRange>) -> usize {
    let mut sum: usize = 0;

    for range in ranges {
        let low: usize = range
            .start
            .parse::<usize>()
            .expect("Failed to parse start integer");
        let high: usize = range
            .end
            .parse::<usize>()
            .expect("Failed to parse end integer");
        let max_len: usize = range.end.len();
        let mut seen: HashSet<usize> = HashSet::new();

        for total_len in 2..=max_len {
            for base_len in 1..=(total_len / 2) {
                let repeat_count: usize = total_len / base_len;
                if total_len % base_len != 0 || repeat_count < 2 {
                    continue;
                }

                let multiplier: usize = (10_usize.pow((base_len * repeat_count) as u32) - 1)
                    / (10_usize.pow(base_len as u32) - 1);

                // smallest s so that s * multiplier >= low
                let s_min: usize = (low + (multiplier - 1)) / multiplier;

                // largest s so that s * multiplier <= high
                let s_max: usize = high / multiplier;
                let lower_bound: usize = 10_usize.pow((base_len - 1) as u32); // ensure s has the correct number of digits

                for s in std::cmp::max(s_min, lower_bound)..=s_max {
                    let s_str: String = s.to_string();
                    if s_str.len() != base_len {
                        continue;
                    }
                    let candidate_str: String = s_str.repeat(repeat_count);
                    let candidate: usize = candidate_str
                        .parse::<usize>()
                        .expect("Failed to parse candidate integer");

                    if candidate >= low && candidate <= high && !seen.contains(&candidate) {
                        sum += candidate;
                        seen.insert(candidate);
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let mut ranges: Vec<InvalidIDRange> = Vec::new();

    match File::open("src/Day 2/input.txt") {
        Ok(mut file) => {
            let mut contents: String = String::new();

            file.read_to_string(&mut contents)
                .expect("Failed to read the file");

            ranges = contents
                .lines()
                .flat_map(|line: &str| line.split(','))
                .filter_map(|r: &str| {
                    let r: &str = r.trim();
                    if r.is_empty() { None } else { Some(r) }
                })
                .map(|range: &str| {
                    let (start, end) = range.split_once('-').unwrap();
                    InvalidIDRange {
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })
                .collect::<Vec<InvalidIDRange>>();
        }
        Err(e) => {
            println!("Error opening file: {}", e);
        }
    }

    println!("Sum of invalid IDs(part 1): {}", sum_invalid_ids(&ranges));
    println!(
        "Sum of invalid IDs(part 2): {}",
        sum_modified_invalid_id(&ranges)
    );
}
