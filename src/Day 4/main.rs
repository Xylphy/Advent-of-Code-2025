use std::fs::File;
use std::io::Read;
use std::ops::{Deref, DerefMut};

struct Grid(Vec<Vec<u8>>);

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            writeln!(f, "{}", String::from_utf8_lossy(row))?;
        }
        Ok(())
    }
}

impl FromIterator<Vec<u8>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<u8>>>(iter: T) -> Self {
        Grid(iter.into_iter().collect())
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<u8>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn total_accessed(lines: &Vec<String>) -> usize {
    let mut count: usize = 0;
    let height: usize = lines.len();
    let width: usize = lines.get(0).expect("Lines must not be empty").len();

    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '@' {
                continue;
            }

            let mut neighbor_count: i32 = 0;

            for &(dx, dy) in &DIRECTIONS {
                let ny: isize = y as isize + dy;
                let nx: isize = x as isize + dx;

                if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
                    continue;
                }

                if lines[ny as usize].as_bytes()[nx as usize] as char == '@' {
                    neighbor_count += 1;
                }
            }

            if neighbor_count < 4 {
                count += 1;
            }
        }
    }

    count
}

fn dfs_remove(grid: &mut Grid, x: isize, y: isize, height: usize, width: usize) -> usize {
    let mut removed_count: usize = 0;
    let mut neighbor_count: usize = 0;

    for &(dx, dy) in &DIRECTIONS {
        let ny: isize = y as isize + dy;
        let nx: isize = x as isize + dx;

        if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
            continue;
        }

        if grid[ny as usize][nx as usize] as char == '@' {
            neighbor_count += 1;
        }
    }

    if neighbor_count < 4 {
        grid[y as usize][x as usize] = b'.';
        removed_count += 1;
        for &(dx, dy) in &DIRECTIONS {
            if x + dx < 0
                || x + dx >= width as isize
                || y + dy < 0
                || y + dy >= height as isize
                || grid[(y + dy) as usize][(x + dx) as usize] != b'@'
            {
                continue;
            }

            removed_count += dfs_remove(grid, x + dx, y + dy, height, width);
        }
    }

    removed_count
}

fn total_removed(lines: &Vec<String>) -> usize {
    let mut count: usize = 0;
    let height: usize = lines.len();
    let width: usize = lines.get(0).expect("Lines must not be empty").len();

    let mut grid: Grid = lines
        .iter()
        .map(|line: &String| line.as_bytes().to_vec())
        .collect();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut neighbor_count: i32 = 0;

            for &(dx, dy) in &DIRECTIONS {
                let ny: isize = y as isize + dy;
                let nx: isize = x as isize + dx;

                if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
                    continue;
                }

                if grid[ny as usize][nx as usize] as char == '@' {
                    neighbor_count += 1;
                }
            }

            if neighbor_count < 4 {
                grid[y][x] = b'.';
                count += 1;

                for &(dx, dy) in &DIRECTIONS {
                    let ny: isize = y as isize + dy;
                    let nx: isize = x as isize + dx;
                    if nx < 0
                        || nx >= width as isize
                        || ny < 0
                        || ny >= height as isize
                        || grid[ny as usize][nx as usize] != b'@'
                    {
                        continue;
                    }

                    count += dfs_remove(&mut grid, nx, ny, height, width);
                }
            }
        }
    }

    count
}

fn main() {
    let lines: Vec<String> = match File::open("src/Day 4/input.txt") {
        Ok(mut file) => {
            let mut contents: String = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
                .lines()
                .map(|line: &str| line.to_string())
                .collect()
        }
        Err(_) => {
            println!("Error reading the file.");
            std::process::exit(1);
        }
    };

    println!("Part 1: {}", total_accessed(&lines));
    println!("Part 2: {}", total_removed(&lines));
}
