use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
            let mut max_i = 0;
            for i in 1..(digits.len() - 1) {
                if digits[i] > digits[max_i] {
                    max_i = i;
                }
            }
            let mut max_i_1 = max_i + 1;
            for i in (max_i + 2)..digits.len() {
                if digits[i] > digits[max_i_1] {
                    max_i_1 = i;
                }
            }
            let val = digits[max_i] * 10 + digits[max_i_1];
            answer += val;
        }
        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
            let mut max_i = 0;
            let mut val = 0;
            for d in (0..12).rev() {
                let current_i = digits.len() - d;
                max_i = if d == 11 { 0 } else { max_i + 1 };
                for i in max_i + 1..current_i {
                    if digits[i] > digits[max_i] {
                        max_i = i;
                    }
                }
                val = val * 10 + digits[max_i];
            }
            answer += val;
        }
        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
