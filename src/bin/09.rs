use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let points = reader
            .lines()
            .map(|line| {
                let line = line?;
                let (x_str, y_str) = line
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Invalid point line: {}", line))?;
                let x: usize = x_str.parse()?;
                let y: usize = y_str.parse()?;
                Ok((x, y))
            })
            .collect::<Result<Vec<(usize, usize)>>>()?;
        let answer = points
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| {
                let dx = (p1.0 as isize - p2.0 as isize).abs() as usize + 1;
                let dy = (p1.1 as isize - p2.1 as isize).abs() as usize + 1;
                dx * dy
            })
            .max()
            .unwrap_or(0);
        Ok(answer)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let points = reader
            .lines()
            .map(|line| {
                let line = line?;
                let (x_str, y_str) = line
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Invalid point line: {}", line))?;
                let x: usize = x_str.parse()?;
                let y: usize = y_str.parse()?;
                Ok((x, y))
            })
            .collect::<Result<Vec<(usize, usize)>>>()?;

        let mut is_inside_memo = HashMap::new();

        let mut is_inside = |p: (usize, usize)| {
            let (px, py) = p;
            if let Some(&res) = is_inside_memo.get(&p) {
                return res;
            }
            let n = points.len();
            for i in 0..n {
                let (x1, y1) = points[i];
                let (x2, y2) = points[(i + 1) % n];

                // Check if on vertical edge
                if x1 == x2 && px == x1 {
                    if y1.min(y2) <= py && py <= y1.max(y2) {
                        is_inside_memo.insert(p, true);
                        return true;
                    }
                }

                // Check if on horizontal edge
                if y1 == y2 && py == y1 {
                    if x1.min(x2) <= px && px <= x1.max(x2) {
                        is_inside_memo.insert(p, true);
                        return true;
                    }
                }
            }

            // use ray-casting algorithm
            let mut inside = false;
            for i in 0..n {
                let (x1, y1) = points[i];
                let (x2, y2) = points[(i + 1) % n];

                if x1 == x2 {
                    let x_edge = x1;
                    let y_min = y1.min(y2);
                    let y_max = y1.max(y2);

                    if x_edge > px && y_min <= py && py < y_max {
                        inside = !inside;
                    }
                }
            }
            is_inside_memo.insert(p, inside);
            inside
        };

        let mut current_max = 0;

        for (p1, p2) in points.iter().tuple_combinations() {
            let (x1, y1) = *p1;
            let (x2, y2) = *p2;
            let dx = (p1.0 as isize - p2.0 as isize).abs() as usize + 1;
            let dy = (p1.1 as isize - p2.1 as isize).abs() as usize + 1;
            if dx * dy <= current_max {
                continue;
            }
            let mut points_on_edges = vec![];
            for i in x1.min(x2)..=x1.max(x2) {
                points_on_edges.push((i, y1));
                points_on_edges.push((i, y2));
            }
            for j in y1.min(y2)..=y1.max(y2) {
                points_on_edges.push((x1, j));
                points_on_edges.push((x2, j));
            }
            points_on_edges.push((x1, y2));
            points_on_edges.push((x2, y1));
            if !points_on_edges.iter().all(|&p| is_inside(p)) {
                continue;
            }

            let dx = (p1.0 as isize - p2.0 as isize).abs() as usize + 1;
            let dy = (p1.1 as isize - p2.1 as isize).abs() as usize + 1;
            let p = dx * dy;
            if p > current_max {
                current_max = p;
            }
        }
        Ok(current_max)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
