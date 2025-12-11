use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut graph = HashMap::new();
        for line in reader.lines().flatten() {
            let mut parts = line.split(':');
            let node = parts
                .next()
                .ok_or_else(|| anyhow!("Missing node in line: {}", line))?
                .trim();
            let edges = parts
                .next()
                .ok_or_else(|| anyhow!("Missing edges in line: {}", line))?
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            graph.insert(node.to_string(), edges);
        }
        let start = "you";
        let target = "out";
        let mut no_of_paths = 0;

        fn dfs(
            graph: &HashMap<String, Vec<String>>,
            current: &str,
            target: &str,
            visited: &mut HashSet<String>,
            no_of_paths: &mut usize,
        ) {
            if current == target {
                *no_of_paths += 1;
                return;
            }
            if let Some(neighbors) = graph.get(current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        dfs(graph, neighbor, target, visited, no_of_paths);
                        visited.remove(neighbor);
                    }
                }
            }
        }

        let mut visited = HashSet::new();
        visited.insert(start.to_string());
        dfs(&graph, start, target, &mut visited, &mut no_of_paths);
        Ok(no_of_paths)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut graph = HashMap::new();
        for line in reader.lines().flatten() {
            let mut parts = line.split(':');
            let node = parts
                .next()
                .ok_or_else(|| anyhow!("Missing node in line: {}", line))?
                .trim();
            let edges = parts
                .next()
                .ok_or_else(|| anyhow!("Missing edges in line: {}", line))?
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            graph.insert(node.to_string(), edges);
        }
        let start = "svr";
        let target = "out";

        fn dfs(
            graph: &HashMap<String, Vec<String>>,
            current: &str,
            target: &str,
            visited: &mut HashSet<String>,
            visited_dac: bool,
            visited_fft: bool,
            memo: &mut HashMap<(String, bool, bool), usize>,
        ) -> usize {
            if current == target {
                return if visited_dac && visited_fft { 1 } else { 0 };
            }

            let key = (current.to_string(), visited_dac, visited_fft);
            if let Some(&cached) = memo.get(&key) {
                return cached;
            }

            let mut count = 0;
            if let Some(neighbors) = graph.get(current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());

                        let new_visited_dac = visited_dac || neighbor == "dac";
                        let new_visited_fft = visited_fft || neighbor == "fft";

                        count += dfs(
                            graph,
                            neighbor,
                            target,
                            visited,
                            new_visited_dac,
                            new_visited_fft,
                            memo,
                        );

                        visited.remove(neighbor);
                    }
                }
            }

            memo.insert(key, count);
            count
        }

        let mut visited = HashSet::new();
        visited.insert(start.to_string());
        let mut memo = HashMap::new();
        let no_of_paths = dfs(&graph, start, target, &mut visited, false, false, &mut memo);

        Ok(no_of_paths)
    }

    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
