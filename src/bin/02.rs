use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().next().context("No input")??;
        let numbers = line
            .split(',')
            .map(|n| {
                let splits = n.split('-').collect::<Vec<&str>>();
                // if splits.len() != 2 {
                //     return Err(anyhow!("Invalid range: {}", n));
                // }
                // let start: usize = splits[0].parse()?;
                // let end: usize = splits[1].parse()?;
                Ok((splits[0], splits[1]))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut answer = 0;
        for &(start, end) in &numbers {
            let n_start = start.len();
            let start_i = start.parse::<usize>()?;
            let end_i = end.parse::<usize>()?;
            let mut current;
            if n_start % 2 == 0 {
                let first_half = &start[..n_start / 2];
                let mut first_half_i: usize = first_half.parse()?;
                let next_num: usize = format!("{first_half_i}{first_half_i}").parse()?;
                if next_num < n_start {
                    first_half_i = first_half_i + 1;
                }
                current = first_half_i;
            } else {
                let first_half = 10usize.pow(n_start as u32 / 2);
                current = first_half;
            }
            loop {
                let num = format!("{current}{current}");
                let num_i: usize = num.parse()?;
                if num_i > end_i {
                    break;
                }
                if num_i >= start_i {
                    answer += num_i;
                }
                current += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let line = reader.lines().next().context("No input")??;
        let numbers = line
            .split(',')
            .map(|n| {
                let splits = n.split('-').collect::<Vec<&str>>();
                // if splits.len() != 2 {
                //     return Err(anyhow!("Invalid range: {}", n));
                // }
                // let start: usize = splits[0].parse()?;
                // let end: usize = splits[1].parse()?;
                Ok((splits[0], splits[1]))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut answer = 0;
        let mut seen = HashSet::new();
        for &(start, end) in &numbers {
            let n_start = start.len();
            let n_end = end.len();
            let start_i = start.parse::<usize>()?;
            let end_i = end.parse::<usize>()?;
            for d in 2..=n_end {
                let mut current;
                if n_start % d == 0 {
                    let first_half = &start[..n_start / d];
                    let mut first_half_i: usize = first_half.parse()?;
                    let next_num: usize = format!("{first_half_i}").repeat(d).parse()?;
                    if next_num < n_start {
                        first_half_i = first_half_i + 1;
                    }
                    current = first_half_i;
                } else {
                    let first_half = 10usize.pow(n_start as u32 / d as u32);
                    current = first_half;
                }
                loop {
                    let num = format!("{current}").repeat(d);
                    let num_i: usize = num.parse()?;
                    if num_i > end_i {
                        break;
                    }
                    if num_i >= start_i && !seen.contains(&num_i) {
                        answer += num_i;
                        seen.insert(num_i);
                    }
                    current += 1;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
