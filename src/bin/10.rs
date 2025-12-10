use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use z3::ast::Int;
use z3::Optimize;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct BitMask(usize);

    impl BitMask {
        fn from_str(s: &str) -> Self {
            let mut value = 0;
            for (i, c) in s.chars().enumerate() {
                if c == '#' {
                    value |= 1 << i;
                }
            }
            BitMask(value)
        }

        fn toggle(&mut self, i: usize) {
            self.0 ^= 1 << i;
        }

        fn button_press(&self, other: &BitMask) -> BitMask {
            BitMask(self.0 ^ other.0)
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines().flatten() {
            let mut parts = line.split_whitespace();
            let final_conf = parts
                .next()
                .ok_or_else(|| anyhow!("Missing configuration in line: {}", line))?
                .trim_matches(&['[', ']'] as &[_]);
            let final_mask = BitMask::from_str(final_conf);
            let start_mask = BitMask(0);
            let buttons = parts
                .filter_map(|p| {
                    if !p.starts_with('(') || !p.ends_with(')') {
                        return None;
                    }
                    let p = p.trim_matches(&['(', ')']);
                    let indices = p.split(',').filter_map(|s| s.parse::<usize>().ok());
                    let mut mask = BitMask(0);
                    for i in indices {
                        mask.toggle(i);
                    }
                    Some(mask)
                })
                .collect::<Vec<_>>();

            let mut presses = None;
            // BFS to find minimum button presses
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back((start_mask, 0));
            visited.insert(start_mask);

            while let Some((current, steps)) = queue.pop_front() {
                if current == final_mask {
                    presses = Some(steps);
                    break;
                }

                for button in &buttons {
                    let next = current.button_press(button);
                    if !visited.contains(&next) {
                        visited.insert(next);
                        queue.push_back((next, steps + 1));
                    }
                }
            }
            answer += presses.unwrap_or(0);
        }
        Ok(answer)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines().flatten() {
            let mut parts = line.split_whitespace();
            let _ = parts.next().unwrap();
            let mut buttons = parts
                .map(|p| {
                    let p = p.trim_matches(&['(', ')', '{', '}']);
                    p.split(',')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>();
            let final_values = buttons.pop().unwrap();

            let opt = Optimize::new();

            // Create variables: x[i] = number of times button i is pressed
            let vars: Vec<Int> = (0..buttons.len())
                .map(|i| Int::new_const(format!("x{}", i)))
                .collect();

            // Constraint: each variable >= 0
            for var in &vars {
                opt.assert(&var.ge(&Int::from_i64(0)));
            }

            // Constraint: for each position, sum of button presses = target value
            for pos in 0..final_values.len() {
                let mut sum = Int::from_i64(0);
                for (button_idx, button) in buttons.iter().enumerate() {
                    if button.contains(&pos) {
                        sum = sum + &vars[button_idx];
                    }
                }
                opt.assert(&sum.eq(&Int::from_i64(final_values[pos] as i64)));
            }

            // Objective: minimize total button presses
            let mut total = Int::from_i64(0);
            for var in &vars {
                total = total + var;
            }
            opt.minimize(&total);

            // Solve
            match opt.check(&[]) {
                z3::SatResult::Sat => {
                    let model = opt.get_model().unwrap();
                    let mut min_presses = 0;
                    for var in &vars {
                        if let Some(val) = model.eval(var, true) {
                            min_presses += val.as_i64().unwrap() as usize;
                        }
                    }
                    answer += min_presses;
                }
                _ => {
                    answer += 0;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);
    println!("Test passed.");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
