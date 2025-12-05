use std::{fs::read_to_string, str::Split};

fn count_fresh_ingredients(
    processed_ranges: &Vec<(usize, usize)>,
    ingredients: &Vec<usize>,
) -> usize {
    ingredients
        .iter()
        .filter(|&&ingredient| {
            processed_ranges
                .iter()
                .any(|&(start, end)| ingredient >= start && ingredient <= end)
        })
        .count()
}

fn count_fresh_ranges(processed_ranges: &Vec<(usize, usize)>) -> usize {
    processed_ranges
        .iter()
        .fold(0_usize, |acc, &(start, end)| acc + (end - start + 1))
}

fn main() {
    let contents: String = match read_to_string("src/Day 5/input.txt") {
        Ok(c) => c,
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    };

    let mut sections: Split<'_, &str> = contents.split("\n\n");

    let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();

    let range_section: &str = sections.next().expect("No range section");
    let ingredients_section: &str = sections.next().expect("No ingredient section");

    for line in range_section.lines() {
        if let Some((start, end)) = line.split_once('-') {
            fresh_ranges.push((
                start.trim().parse().expect("Invalid start number"),
                end.trim().parse().expect("Invalid end number"),
            ));
        } else {
            panic!("Invalid range line: {}", line);
        }
    }

    for line in ingredients_section.lines() {
        ingredients.push(
            line.trim()
                .parse()
                .expect(&format!("Invalid ingredient number {}", line)),
        );
    }

    // Preprocess
    fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut processed_ranges: Vec<(usize, usize)> = vec![(fresh_ranges[0])]; // Initialize with the first range

    // Merge overlapping ranges
    for i in 1..fresh_ranges.len() {
        if fresh_ranges[i].0
            <= processed_ranges
                .last()
                .expect("There's no last element in processed ranges")
                .1
                + 1
        {
            let (old_start, old_end) = processed_ranges
                .pop()
                .expect("There's no last element in processed ranges");

            processed_ranges.push((old_start, fresh_ranges[i].1.max(old_end)));
        } else {
            processed_ranges.push(fresh_ranges[i]);
        }
    }

    println!(
        "Number of fresh ingredients: {}",
        count_fresh_ingredients(&processed_ranges, &ingredients)
    );

    println!(
        "Total number of fresh ingredient IDs in ranges: {}",
        count_fresh_ranges(&processed_ranges)
    );
}
