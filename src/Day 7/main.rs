use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

fn count_splitted(grid: &[Vec<u8>], starting_point: usize) -> usize {
    let mut count: usize = 0;
    let mut stack: Vec<(usize, usize)> = vec![(0, starting_point)];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let height: usize = grid.len();

    while let Some((mut current_depth, current_index)) = stack.pop() {
        while let Some(&character) = grid[current_depth].get(current_index) {
            if character == b'^' {
                if seen.contains(&(current_depth, current_index)) {
                    break;
                }
                stack.push((current_depth, current_index + 1));
                stack.push((current_depth, current_index - 1));
                seen.insert((current_depth, current_index));
                count += 1;
                break;
            }

            if current_depth + 1 < height {
                current_depth += 1;
            } else {
                break;
            }
        }
    }

    count
}

fn count_timelines(grid: &[Vec<u8>], starting_point: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut curr: Vec<usize> = vec![0usize; width];
    curr[starting_point] = 1;

    for row in 0..height - 1 {
        let mut next: Vec<usize> = vec![0usize; width];

        for col in 0..width {
            let count: usize = curr[col];
            if count == 0 {
                continue;
            }

            let below = grid[row + 1][col];
            if below == b'^' {
                if col > 0 {
                    next[col - 1] += count;
                }
                if col + 1 < width {
                    next[col + 1] += count;
                }
            } else {
                next[col] += count;
            }
        }

        curr = next;
    }

    curr.into_iter().sum()
}

fn main() {
    let lines: Vec<Vec<u8>> =
        BufReader::new(File::open("src/Day 7/input.txt").expect("Can't open file"))
            .lines()
            .map(|line: Result<String, std::io::Error>| {
                line.expect("Error reading a line").as_bytes().to_vec()
            })
            .collect::<Vec<Vec<u8>>>();

    let starting_point = lines[0]
        .iter()
        .position(|&character| character == b'S')
        .expect("Starting point not found");

    println!(
        "The number of splitted paths is: {}",
        count_splitted(&lines, starting_point)
    );

    println!(
        "The number of timelines is: {}",
        count_timelines(&lines, starting_point)
    );
}
