use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0;
        let mut lines = reader.lines().flatten();
        let mut ranges = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let (start, end) = line
                .split_once('-')
                .ok_or_else(|| anyhow!("Invalid range line: {}", line))?;
            let start: usize = start.parse()?;
            let end: usize = end.parse()?;
            ranges.push((start, end));
        }
        for line in lines {
            let point: usize = line.parse()?;
            for (start, end) in &ranges {
                if point >= *start && point <= *end {
                    count += 1;
                    break;
                }
            }
        }
        Ok(count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().flatten();
        let mut ranges = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let (start, end) = line
                .split_once('-')
                .ok_or_else(|| anyhow!("Invalid range line: {}", line))?;
            let start: usize = start.parse()?;
            let end: usize = end.parse()?;
            ranges.push((start, end));
        }

        fn merge_overlapping_ranges(ranges: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
            if ranges.is_empty() {
                return Vec::new();
            }
            ranges.sort_by_key(|r| r.0);
            let mut merged = Vec::new();
            let mut current_start = ranges[0].0;
            let mut current_end = ranges[0].1;
            for &(start, end) in &ranges[1..] {
                if start <= current_end {
                    current_end = current_end.max(end);
                } else {
                    merged.push((current_start, current_end));
                    current_start = start;
                    current_end = end;
                }
            }
            merged.push((current_start, current_end));
            merged
        }

        let ranges = merge_overlapping_ranges(&mut ranges);
        let mut count = 0;
        for (start, end) in &ranges {
            count += end - start + 1;
        }

        Ok(count)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
