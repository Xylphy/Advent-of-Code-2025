use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
    num::ParseIntError,
};

struct Line {
    goal: u16,
    buttons: Vec<Vec<u16>>,
    requirements: Vec<u16>,
}

const INF: usize = usize::MAX;

fn parse_numbers(s: &str) -> Vec<u16> {
    s.split(',')
        .map(|num_str: &str| {
            let trimmed: &str = num_str.trim();
            trimmed
                .parse::<u16>()
                .unwrap_or_else(|e: ParseIntError| panic!("Invalid number '{trimmed}' ({e})"))
        })
        .collect()
}

fn parse_line(s: &str) -> Line {
    let string: &str = s.trim();
    assert!(!string.is_empty(), "Empty line encountered");

    let mut start: usize = string.find('[').expect("No [ found");
    let mut end: usize = string.find(']').expect("No ] found");

    let goal_str: &str = &string[start + 1..end];
    let mut goal: u16 = 0;

    for (i, c) in goal_str.chars().enumerate() {
        goal |= u16::from(c == '#') << i;
    }

    let mut buttons: Vec<Vec<u16>> = Vec::new();
    while let Some(next_start) = string[end..].find('(') {
        start = end + next_start + 1;
        end = start
            + string[start..]
                .find(')')
                .expect("No closing ) found for button");

        buttons.push(parse_numbers(&string[start..end]));
    }

    start = string[end..]
        .find('{')
        .expect("No { found for requirements")
        + end;
    end = string[end..]
        .find('}')
        .expect("No closing } found for requirements")
        + end;

    Line {
        goal,
        buttons,
        requirements: parse_numbers(&string[start + 1..end]),
    }
}

fn toggle(goal: &mut u16, button: &[u16]) {
    for &pos in button {
        *goal ^= 1 << pos;
    }
}

fn find_fewest_presses(goal: u16, buttons: &[Vec<u16>]) -> Option<usize> {
    let mut visited: HashSet<u16> = HashSet::new();
    // BFS queue storing (current_state, depth)
    let mut queue: VecDeque<(u16, usize)> = VecDeque::new();

    queue.push_back((0, 0));
    visited.insert(0);

    while let Some((current_state, depth)) = queue.pop_front() {
        if current_state == goal {
            return Some(depth);
        }

        for button in buttons {
            let mut next_state = current_state;
            toggle(&mut next_state, button);

            if visited.insert(next_state) {
                queue.push_back((next_state, depth + 1));
            }
        }
    }

    None
}

fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .filter_map(|line: &Line| find_fewest_presses(line.goal, &line.buttons))
        .sum()
}

fn solve_part2_min_presses(requirements: &[u16], buttons: &[Vec<u16>]) -> usize {
    dfs(requirements.to_vec(), buttons, &mut HashMap::new())
}

fn dfs(target: Vec<u16>, buttons: &[Vec<u16>], memo: &mut HashMap<Vec<u16>, usize>) -> usize {
    if target.iter().all(|x: &u16| *x == 0) {
        return 0;
    }
    if let Some(&ans) = memo.get(&target) {
        return ans;
    }

    let n: usize = target.len();

    let mut best: usize = INF;

    for mask in 0..(1 << buttons.len()) {
        let mut used: usize = 0;
        let mut effect: Vec<u16> = vec![0; n];

        // Build Phase-1 subset effect
        for (j, button) in buttons.iter().enumerate() {
            // Check if button j is included in the subset
            if (mask >> j) & 1 == 1 {
                used += 1;
                for &pos in button {
                    effect[usize::from(pos)] += 1;
                }
            }
        }

        // Check: target - effect must be >= 0 and all even
        let mut next: Vec<u16> = vec![0; n];
        let mut ok: bool = true;

        for i in 0..n {
            if effect[i] > target[i] {
                ok = false;
                break;
            }

            let rem: u16 = target[i] - effect[i];
            if (rem & 1) == 1 {
                ok = false;
                break;
            }
            next[i] = rem / 2;
        }

        if !ok {
            continue;
        }

        let sub: usize = dfs(next, buttons, memo);
        if sub == INF {
            continue;
        }

        best = best.min(used + 2 * sub);
    }

    memo.insert(target, best);
    best
}

fn part2(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|line: &Line| solve_part2_min_presses(&line.requirements, &line.buttons))
        .sum()
}

fn main() {
    let lines: Vec<Line> =
        BufReader::new(File::open("src/Day 10/input.txt").expect("Can't open file"))
            .lines()
            .map(|line: Result<String, Error>| match line {
                Ok(l) => parse_line(&l),
                Err(e) => {
                    panic!("Error reading line: {e}");
                }
            })
            .collect();

    println!(
        "Fewest button presses against goal(Part 1): {}",
        part1(&lines)
    );
    println!(
        "Fewest button presses against requirements(Part 2): {}",
        part2(&lines)
    );
}
