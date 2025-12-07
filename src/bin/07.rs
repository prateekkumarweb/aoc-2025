use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    struct Grid {
        cells: Vec<Vec<char>>,
    }

    impl Grid {
        fn left(&self, row: usize, col: usize) -> Option<(usize, usize)> {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        }

        fn right(&self, row: usize, col: usize) -> Option<(usize, usize)> {
            if col + 1 >= self.cells[0].len() {
                None
            } else {
                Some((row, col + 1))
            }
        }

        fn down(&self, row: usize, col: usize) -> Option<(usize, usize)> {
            if row + 1 >= self.cells.len() {
                None
            } else {
                Some((row + 1, col))
            }
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let grid = Grid { cells: grid };
        let s_pos = grid.cells[0]
            .iter()
            .position(|&c| c == 'S')
            .ok_or_else(|| anyhow!("No starting position found"))?;
        let mut answer = 0;
        let mut beams = vec![s_pos];
        for row in 2..grid.cells.len() {
            let mut new_beams = HashSet::new();
            for col in beams {
                let down = grid.down(row - 1, col).unwrap();
                match grid.cells[down.0][down.1] {
                    '^' => {
                        if let Some(left) = grid.left(down.0, down.1) {
                            new_beams.insert(left.1);
                        }
                        if let Some(right) = grid.right(down.0, down.1) {
                            new_beams.insert(right.1);
                        }
                        answer += 1;
                    }
                    _ => {
                        new_beams.insert(col);
                    }
                }
            }
            beams = new_beams.iter().sorted().copied().collect();
        }
        Ok(answer)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let grid = Grid { cells: grid };
        let s_pos = grid.cells[0]
            .iter()
            .position(|&c| c == 'S')
            .ok_or_else(|| anyhow!("No starting position found"))?;
        let mut beams = HashMap::new();
        beams.insert(s_pos, 1);
        for row in 2..grid.cells.len() {
            let mut new_beams = HashMap::new();
            for (col, c) in beams {
                let down = grid.down(row - 1, col).unwrap();
                match grid.cells[down.0][down.1] {
                    '^' => {
                        if let Some(left) = grid.left(down.0, down.1) {
                            *new_beams.entry(left.1).or_insert(0) += c;
                        }
                        if let Some(right) = grid.right(down.0, down.1) {
                            *new_beams.entry(right.1).or_insert(0) += c;
                        }
                    }
                    _ => {
                        *new_beams.entry(col).or_insert(0) += c;
                    }
                }
            }
            beams = new_beams;
        }
        Ok(beams.values().sum())
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
