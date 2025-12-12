use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().flatten();
        let mut shapes = vec![];
        let mut count = 0;
        for _ in 0..6 {
            lines.next().unwrap();
            let mut shape = [['.', '.', '.']; 3];
            for i in 0..3 {
                let line = lines.next().unwrap();
                let mut chars = line.chars();
                shape[i][0] = chars.next().unwrap();
                shape[i][1] = chars.next().unwrap();
                shape[i][2] = chars.next().unwrap();
            }
            shapes.push(shape);
            lines.next().unwrap();
        }
        let shape_sizes: Vec<usize> = shapes
            .into_iter()
            .map(|s| {
                let mut count = 0;
                for row in s {
                    for c in row {
                        if c == '#' {
                            count += 1;
                        }
                    }
                }
                count
            })
            .collect();
        for line in lines {
            let parts: Vec<&str> = line.split(':').collect();
            let (x, y) = parts[0].trim().split_once('x').unwrap();
            let (x, y) = (x.parse::<usize>()?, y.parse::<usize>()?);
            let shape_counts = parts[1]
                .trim()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>();
            let total = shape_counts
                .iter()
                .zip(shape_sizes.iter())
                .map(|(a, b)| a * b)
                .sum::<usize>();
            if total <= x * y {
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
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
