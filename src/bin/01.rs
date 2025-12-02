use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut pointer: i32 = 50;
        let mut count = 0;
        for line in reader.lines() {
            let line = line?;
            let (turn, dist) = line.split_at(1);
            let dist: i32 = dist.parse()?;
            match turn {
                "L" => {
                    pointer = (pointer + 100 - dist) % 100;
                }
                "R" => {
                    pointer = (pointer + dist) % 100;
                }
                _ => {
                    return Err(anyhow!("Invalid turn direction: {}", turn));
                }
            }
            if pointer == 0 {
                count += 1;
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
        let mut pointer: i32 = 50;
        let mut count = 0;
        for line in reader.lines() {
            let line = line?;
            let (turn, dist) = line.split_at(1);
            let dist: i32 = dist.parse()?;
            let div = dist / 100;
            let rem = dist % 100;
            count += div as usize;
            if rem == 0 {
                continue;
            }
            match turn {
                "L" => {
                    if pointer == 0 {
                        pointer = 100;
                    }
                    pointer = pointer + 100 - rem;
                    if pointer <= 100 {
                        count += 1;
                    }
                    pointer = pointer % 100;
                }
                "R" => {
                    pointer = pointer + rem;
                    if pointer >= 100 {
                        count += 1;
                    }
                    pointer = pointer % 100;
                }
                _ => {
                    return Err(anyhow!("Invalid turn direction: {}", turn));
                }
            }
        }
        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
