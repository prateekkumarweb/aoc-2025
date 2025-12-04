use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    struct Grid {
        cells: Vec<Vec<char>>,
    }

    impl Grid {
        fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
            let deltas = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ];
            let mut result = Vec::new();
            for (dr, dc) in deltas.iter() {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row >= 0
                    && new_row < self.cells.len() as isize
                    && new_col >= 0
                    && new_col < self.cells[0].len() as isize
                {
                    result.push((new_row as usize, new_col as usize));
                }
            }
            result
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid {
            cells: reader
                .lines()
                .flatten()
                .map(|line| line.chars().collect())
                .collect(),
        };
        let mut answer = 0;
        for row in 0..grid.cells.len() {
            for col in 0..grid.cells[0].len() {
                let cell = grid.cells[row][col];
                if cell != '@' {
                    continue;
                }
                let neighbors = grid.neighbors(row, col);
                let count = neighbors
                    .iter()
                    .filter(|(r, c)| grid.cells[*r][*c] == '@')
                    .count();
                if count < 4 {
                    answer += 1;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = Grid {
            cells: reader
                .lines()
                .flatten()
                .map(|line| line.chars().collect())
                .collect(),
        };
        let mut grid_new = Grid {
            cells: grid.cells.clone(),
        };
        let mut answer = 0;
        loop {
            let mut current = 0;
            for row in 0..grid.cells.len() {
                for col in 0..grid.cells[0].len() {
                    let cell = grid.cells[row][col];
                    if cell != '@' {
                        continue;
                    }
                    let neighbors = grid.neighbors(row, col);
                    let count = neighbors
                        .iter()
                        .filter(|(r, c)| grid.cells[*r][*c] == '@')
                        .count();
                    if count < 4 {
                        grid_new.cells[row][col] = '.';
                        current += 1;
                    }
                }
            }
            if current == 0 {
                break;
            }
            grid = Grid {
                cells: grid_new.cells.clone(),
            };
            answer += current;
        }

        Ok(answer)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
