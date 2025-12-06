use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut input = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<_>>>();
        let operators = input.pop().context("No operators line")?;
        let n = operators.len();
        for col in 0..n {
            let mut col_values = Vec::new();
            for row in 0..input.len() {
                let val: usize = input[row][col].parse()?;
                col_values.push(val);
            }
            let col_answer: usize = match operators[col].as_str() {
                "+" => col_values.iter().sum(),
                "*" => col_values.iter().product(),
                _ => return Err(anyhow!("Invalid operator: {}", operators[col])),
            };
            answer += col_answer;
        }
        Ok(answer)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut input = reader
            .lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let n = input.iter().map(|row| row.len()).max().unwrap_or(0);
        let operators = input.pop().context("No operators line")?;
        let mut current_values = vec![];
        for col in (0..n).rev() {
            let mut val = String::new();
            for row in 0..input.len() {
                if col < input[row].len() {
                    val.push(input[row][col]);
                }
            }
            if val.trim().is_empty() {
                continue;
            }
            let col_value: usize = val.trim().parse()?;
            current_values.push(col_value);
            let op = operators.get(col).cloned().unwrap_or(' ');
            match op {
                '+' => {
                    answer += current_values.iter().sum::<usize>();
                    current_values.clear();
                }
                '*' => {
                    answer += current_values.iter().product::<usize>();
                    current_values.clear();
                }
                _ => {}
            }
        }
        Ok(answer)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
