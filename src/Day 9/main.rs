use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

mod point;

use point::Point;

fn largest_area_part1(points: &[Point]) -> u64 {
    (1..points.len())
        .flat_map(|x: usize| (0..x).map(move |y: usize| points[x].get_area(&points[y])))
        .max()
        .unwrap_or(0u64)
}

fn merge_intervals(mut intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if intervals.is_empty() {
        return intervals;
    }
    intervals.sort_by_key(|(a, b): &(i64, i64)| (*a, *b));

    let mut out: Vec<(i64, i64)> = Vec::with_capacity(intervals.len());
    let mut cur: (i64, i64) = intervals[0];

    for (a, b) in intervals.into_iter().skip(1) {
        if a <= cur.1 + 1 {
            cur.1 = cur.1.max(b);
        } else {
            out.push(cur);
            cur = (a, b);
        }
    }
    out.push(cur);
    out
}

fn row_covers(intervals: &[(i64, i64)], x1: i64, x2: i64) -> bool {
    // intervals are merged + sorted
    for &(a, b) in intervals {
        if a <= x1 && b >= x2 {
            return true;
        }
        if a > x1 {
            break;
        }
    }
    false
}

fn largest_area_part2(points: &[Point]) -> u64 {
    if points.is_empty() {
        return 0;
    }

    // Red tiles = given points
    let red: HashSet<(i64, i64)> = points.iter().map(|p| (p.x, p.y)).collect();

    // Build edges with wrap-around (IMPORTANT: last -> first)
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let a: &Point = &points[i];
        let b: &Point = &points[(i + 1) % points.len()];
        edges.push(((a.x, a.y), (b.x, b.y)));
    }

    // For each row y:
    // - crossings[y] = x positions where a vertical edge crosses that row (scanline parity)
    // - boundary[y]  = explicit boundary coverage on that row (horizontal runs + vertical points)
    let mut crossings: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut boundary: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    let mut min_y: i64 = i64::MAX;
    let mut max_y: i64 = i64::MIN;

    for &((x1, y1), (x2, y2)) in &edges {
        min_y = min_y.min(y1).min(y2);
        max_y = max_y.max(y1).max(y2);

        if x1 == x2 {
            // Vertical edge at x = x1 from ylo..yhi (inclusive boundary tiles)
            let ylo = y1.min(y2);
            let yhi = y1.max(y2);

            // Boundary tiles on this vertical line
            for y in ylo..=yhi {
                boundary.entry(y).or_default().push((x1, x1));
            }

            // Scanline crossings: use half-open [ylo, yhi) to avoid double-counting vertices
            for y in ylo..yhi {
                crossings.entry(y).or_default().push(x1);
            }
        } else {
            // Horizontal edge on row y = y1 from xlo..xhi (inclusive boundary tiles)
            boundary
                .entry(y1)
                .or_default()
                .push((x1.min(x2), x1.max(x2)));
        }
    }

    // Build merged valid intervals per row: (interior by parity) U (boundary)
    let mut valid_by_y: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    for y in min_y..=max_y {
        let mut intervals: Vec<(i64, i64)> = Vec::new();

        if let Some(xs) = crossings.get_mut(&y) {
            xs.sort_unstable();
            // Pair up crossings: [x0,x1], [x2,x3], ... are inside (including boundary)
            for pair in xs.chunks_exact(2) {
                intervals.push((pair[0].min(pair[1]), pair[0].max(pair[1])));
            }
        }

        if let Some(b) = boundary.get(&y) {
            intervals.extend_from_slice(b);
        }

        let merged: Vec<(i64, i64)> = merge_intervals(intervals);
        if !merged.is_empty() {
            valid_by_y.insert(y, merged);
        }
    }

    // Try all pairs of red tiles as opposite corners; validate rectangle rows via intervals
    let reds: Vec<(i64, i64)> = red.iter().copied().collect();
    let mut best: u64 = 0;

    for i in 0..reds.len() {
        let (ax, ay) = reds[i];
        for &(bx, by) in &reds[(i + 1)..] {
            let x1: i64 = ax.min(bx);
            let x2: i64 = ax.max(bx);
            let y1: i64 = ay.min(by);
            let y2: i64 = ay.max(by);

            // Quick pruning: if any row is missing entirely, it's invalid
            let mut ok: bool = true;
            for y in y1..=y2 {
                match valid_by_y.get(&y) {
                    Some(intervals) if row_covers(intervals, x1, x2) => {}
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok {
                continue;
            }

            best = best.max(((x2 - x1 + 1) as u64) * ((y2 - y1 + 1) as u64));
        }
    }

    best
}

fn main() {
    let points: Vec<Point> =
        BufReader::new(File::open("src/Day 9/input.txt").expect("Failed to open file"))
            .lines()
            .map(|line| {
                let line_str: String = line.expect("Failed to read line");
                let (x, y) = line_str.split_once(',').expect("Error splitting string");
                Point::new(
                    x.trim().parse::<i64>().expect("Error parsing x"),
                    y.trim().parse::<i64>().expect("Error parsing y"),
                )
            })
            .collect::<Vec<Point>>();

    println!("Largest area (Part 1): {}", largest_area_part1(&points));
    println!("Largest area (Part 2): {}", largest_area_part2(&points));
}
