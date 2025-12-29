use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    num::ParseIntError,
};

struct Line {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    requirements: Vec<usize>,
}

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|num_str: &str| {
            let trimmed: &str = num_str.trim();
            trimmed
                .parse::<usize>()
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

    let goal: Vec<bool> = goal_str
        .chars()
        .map(|c: char| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid character in goal string"),
        })
        .collect();

    let mut buttons: Vec<Vec<usize>> = Vec::new();

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

fn bools_to_mask(bits: &[bool]) -> u64 {
    let mut mask = 0u64;
    for (i, &b) in bits.iter().enumerate() {
        if b {
            mask |= 1u64 << i;
        }
    }
    mask
}

fn button_to_mask(button: &[usize]) -> u64 {
    let mut mask = 0u64;
    for &idx in button {
        mask |= 1u64 << idx;
    }
    mask
}

fn find_fewest_presses_goal_mitm(line: &Line) -> usize {
    let goal_mask: u64 = bools_to_mask(&line.goal);
    let masks: Vec<u64> = line
        .buttons
        .iter()
        .map(|b: &Vec<usize>| button_to_mask(b))
        .collect();

    let m: usize = masks.len();
    let mid: usize = m / 2;
    let left: &[u64] = &masks[..mid];
    let right: &[u64] = &masks[mid..];

    let mut left_best: HashMap<u64, u8> = HashMap::new();

    for subset in 0..(1usize << left.len()) {
        let mut x = 0u64;
        let mut cnt = 0u8;
        for (i, &mask) in left.iter().enumerate() {
            if (subset >> i) & 1 == 1 {
                x ^= mask;
                cnt += 1;
            }
        }
        left_best
            .entry(x)
            .and_modify(|best| *best = (*best).min(cnt))
            .or_insert(cnt);
    }

    let mut answer: usize = usize::MAX;
    for subset in 0..(1usize << right.len()) {
        let mut x: u64 = 0u64;
        let mut cnt: usize = 0usize;
        for (i, &mask) in right.iter().enumerate() {
            if (subset >> i) & 1 == 1 {
                x ^= mask;
                cnt += 1;
            }
        }

        let needed: u64 = goal_mask ^ x;
        if let Some(&lcnt) = left_best.get(&needed) {
            answer = answer.min(cnt + lcnt as usize);
        }
    }

    answer
}

fn solve_machine_min_presses(req: &[usize], buttons: &[Vec<usize>]) -> usize {
    let n = req.len();
    let mut rem: Vec<i32> = req.iter().map(|&x: &usize| x as i32).collect();

    let mut btn_masks: Vec<u64> = buttons
        .iter()
        .map(|b: &Vec<usize>| button_to_mask(b))
        .collect();

    // Sort buttons by decreasing popcount (helps pruning)
    btn_masks.sort_by_key(|&m| std::cmp::Reverse(m.count_ones()));

    let m = btn_masks.len();

    let mut affects: Vec<Vec<bool>> = vec![vec![false; m + 1]; n];
    for i in 0..n {
        affects[i][m] = false;
        for k in (0..m).rev() {
            let bit = ((btn_masks[k] >> i) & 1) == 1;
            affects[i][k] = affects[i][k + 1] || bit;
        }
    }

    let mut best: usize = greedy_upper_bound(&rem, &btn_masks);

    let popcounts: Vec<i32> = btn_masks
        .iter()
        .map(|m| m.count_ones().cast_signed())
        .collect();

    dfs(0, &mut rem, &btn_masks, &popcounts, &affects, 0, &mut best);
    best
}

fn dfs(
    k: usize,
    rem: &mut [i32],
    btn_masks: &[u64],
    popcounts: &[i32],
    affects: &[Vec<bool>],
    current: usize,
    best: &mut usize,
) {
    let m: usize = btn_masks.len();
    let n: usize = rem.len();

    if current >= *best {
        return;
    }

    // If all satisfied
    if rem.iter().all(|&x: &i32| x == 0) {
        *best = current;
        return;
    }

    // No buttons left
    if k == m {
        return;
    }

    // Feasibility prune: if any rem[i] > 0 but no remaining button touches i
    for i in 0..n {
        if rem[i] > 0 && !affects[i][k] {
            return;
        }
    }

    // Lower bound prune
    let max_need: usize = rem.iter().copied().max().unwrap_or(0).max(0) as usize;

    let sum_need: i32 = rem.iter().filter(|&&x| x > 0).sum();
    let lb2: usize = ((sum_need + popcounts[k..].iter().copied().max().unwrap_or(1).max(1) - 1)
        / popcounts[k..].iter().copied().max().unwrap_or(1).max(1)) as usize;

    let lb: usize = max_need.max(lb2);

    if current + lb >= *best {
        return;
    }

    // Compute max_x for this button
    let mask: u64 = btn_masks[k];
    let mut max_x: i32 = i32::MAX;
    for i in 0..n {
        if ((mask >> i) & 1) == 1 {
            max_x = max_x.min(rem[i]);
        }
    }
    if max_x < 0 {
        // This button only hits already-satisfied counters, skip it
        dfs(k + 1, rem, btn_masks, popcounts, affects, current, best);
        return;
    }

    // Try max_x down to 0 (tends to find good solutions fast)
    for x in (0..=max_x).rev() {
        // Apply x presses
        if x > 0 {
            for i in 0..n {
                if ((mask >> i) & 1) == 1 {
                    rem[i] -= x;
                }
            }
        }

        // Overshoot check
        if rem.iter().all(|&v: &i32| v >= 0) {
            dfs(
                k + 1,
                rem,
                btn_masks,
                popcounts,
                affects,
                current + x as usize,
                best,
            );
        }

        // Undo
        if x > 0 {
            for i in 0..n {
                if ((mask >> i) & 1) == 1 {
                    rem[i] += x;
                }
            }
        }
    }
}

// Greedy UB: repeatedly press the best button as much as possible
fn greedy_upper_bound(rem0: &[i32], btn_masks: &[u64]) -> usize {
    let n: usize = rem0.len();
    let mut rem: Vec<i32> = rem0.to_vec();
    let mut presses: usize = 0usize;

    loop {
        if rem.iter().all(|&x: &i32| x == 0) {
            return presses;
        }

        // pick best button
        let mut best_j: Option<usize> = None;
        let mut best_gain: i32 = -1i32;

        for (j, &mask) in btn_masks.iter().enumerate() {
            let mut gain: i32 = 0i32;
            for i in 0..n {
                if rem[i] > 0 && ((mask >> i) & 1) == 1 {
                    gain += 1;
                }
            }
            if gain > best_gain {
                best_gain = gain;
                best_j = Some(j);
            }
        }

        let j = match best_j {
            Some(j) if best_gain > 0 => j,
            _ => return usize::MAX / 4, // unreachable
        };

        // press as many times as allowed
        let mut max_x = i32::MAX;
        for i in 0..n {
            if ((btn_masks[j] >> i) & 1) == 1 {
                max_x = max_x.min(rem[i]);
            }
        }
        if max_x <= 0 {
            return usize::MAX / 4;
        }

        presses += max_x as usize;
        for i in 0..n {
            if ((btn_masks[j] >> i) & 1) == 1 {
                rem[i] -= max_x;
            }
        }
    }
}

fn part_1(lines: &[Line]) -> usize {
    lines.iter().map(find_fewest_presses_goal_mitm).sum()
}

fn part_2(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|line: &Line| solve_machine_min_presses(&line.requirements, &line.buttons))
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
        part_1(&lines)
    );
    println!(
        "Fewest button presses against requirements(Part 2): {}",
        part_2(&lines)
    );
}
