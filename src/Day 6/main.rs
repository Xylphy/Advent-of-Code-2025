use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn evaluate(inputs: &[String]) -> usize {
    let processed_inputs: Vec<Vec<&str>> = inputs
        .iter()
        .map(|line: &String| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let height: usize = processed_inputs.len();
    let width: usize = processed_inputs[0].len();
    let ops_row: &Vec<&str> = processed_inputs.last().expect("No operator row found");

    (0..width)
        .map(|column| {
            let numbers_iter = (0..height - 1).map(|row: usize| {
                processed_inputs[row][column]
                    .parse::<usize>()
                    .expect("Failed to parse a number")
            });

            match ops_row[column] {
                "+" => numbers_iter.sum::<usize>(),
                "*" => numbers_iter.product::<usize>(),
                operator => panic!("Unknown operator: {}", operator),
            }
        })
        .sum::<usize>()
}

fn column_wise_evaluate(inputs: &[String]) -> usize {
    let (rows, op_row) = inputs.split_at(inputs.len() - 1);
    let op_line: &String = &op_row[0];

    let operators: Vec<&str> = op_line.split_whitespace().collect();

    let height = rows.len();
    let width = rows[0].len();

    let mut sum: usize = 0;
    let mut op_idx: usize = 0;

    fn init_semi_total(op: &str) -> usize {
        match op {
            "+" => 0,
            "*" => 1,
            other => panic!("Unknown operator: {other}"),
        }
    }

    let mut semi_total = init_semi_total(operators[0]);

    for x in 0..width {
        let mut has_digit: bool = false;
        let mut number: usize = 0;

        for y in 0..height {
            let b: u8 = rows[y].as_bytes()[x];
            let c: char = b as char;

            if c.is_ascii_digit() {
                has_digit = true;
                number = number * 10 + (b - b'0') as usize;
            }
        }

        if has_digit {
            match operators[op_idx] {
                "+" => semi_total += number,
                "*" => semi_total *= number,
                other => panic!("Unknown operator: {other}"),
            }
        } else {
            sum += semi_total;
            op_idx += 1;

            if op_idx >= operators.len() {
                panic!("Insufficient operators for the number of columns");
            }

            semi_total = init_semi_total(operators[op_idx]);
        }
    }

    sum + semi_total
}

fn main() {
    let file: File = File::open("src/Day 6/input.txt").expect("Failed to open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line: Result<String, std::io::Error>| line.expect("Failed to read line"))
        .collect::<Vec<String>>();

    println!("Evaluated Result (Part 1): {}", evaluate(&lines));
    println!(
        "Column-wise Evaluated Result (Part 2): {}",
        column_wise_evaluate(&lines)
    );
}
