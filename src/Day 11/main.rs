use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn dp1(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == "out" {
        return 1;
    }

    if let Some(&ans) = memo.get(node) {
        return ans;
    }

    let mut total = 0usize;

    if let Some(neighbors) = graph.get(node) {
        for n in neighbors {
            total = total.saturating_add(dp1(n, graph, memo));
        }
    }

    memo.insert(node.to_string(), total);
    total
}

fn part_1(graph: &HashMap<String, Vec<String>>) -> usize {
    dp1("you", graph, &mut HashMap::new())
}

fn flag(node: &str) -> u8 {
    match node {
        "dac" => 1,
        "fft" => 2,
        _ => 0,
    }
}

fn dp(
    node: &str,
    mask: u8,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, u8), usize>,
) -> usize {
    if node == "out" {
        return usize::from(mask == 3);
    }

    if let Some(&ans) = memo.get(&(node.to_string(), mask)) {
        return ans;
    }

    let mut total = 0usize;

    if let Some(neighbors) = graph.get(node) {
        for n in neighbors {
            total = total.saturating_add(dp(n, mask | flag(n.as_str()), graph, memo));
        }
    }

    memo.insert((node.to_string(), mask), total);
    total
}

fn part_2(graph: &HashMap<String, Vec<String>>) -> usize {
    dp("svr", flag("svr"), graph, &mut HashMap::new())
}

fn main() {
    let graph: HashMap<String, Vec<String>> =
        BufReader::new(File::open("src/Day 11/input.txt").expect("Failed to open file"))
            .lines()
            .map_while(Result::ok)
            .map(|line: String| {
                let end: usize = line.find(':').expect("No colon found in line");

                (
                    line[0..end].to_string(),
                    line[end + 1..]
                        .trim()
                        .split(' ')
                        .map(|s: &str| s.trim().to_string())
                        .collect(),
                )
            })
            .collect();

    println!("Number of paths from 'you' to 'out': {}", part_1(&graph));
    println!(
        "Number of paths from 'svr' to 'out' visiting 'fft' and 'dac': {}",
        part_2(&graph)
    );
}
