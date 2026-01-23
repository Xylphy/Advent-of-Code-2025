use std::fs::File;
use std::io::{BufRead, BufReader};

struct Summary {
    x: u8,
    y: u8,
    values: Vec<u8>,
}

fn count_fits(summaries: &[Summary]) -> usize {
    let mut that_fits: usize = 0;

    for summary in summaries {
        let box_count: usize = summary.values.iter().map(|v: &u8| usize::from(*v)).sum();
        let count_box_x: usize = usize::from(summary.x) / 3;
        let count_box_y: usize = usize::from(summary.y) / 3;

        if box_count <= count_box_x * count_box_y {
            that_fits += 1;
        }
    }

    that_fits
}

fn main() -> std::io::Result<()> {
    let mut summaries: Vec<Summary> = Vec::new();

    for line in BufReader::new(File::open("src/Day 12/input.txt")?).lines() {
        let line: String = line?.trim().to_string();

        if line.contains('x')
            && let Some((dimension, quantities)) = line.split_once(':')
            && let Some((x, y)) = dimension.split_once('x')
        {
            summaries.push(Summary {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                values: quantities
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            });
        }
    }

    println!("That fits: {}", count_fits(&summaries));

    Ok(())
}
