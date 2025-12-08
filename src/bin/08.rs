use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    struct Point(usize, usize, usize);

    impl Point {
        fn dist_sq(&self, other: &Point) -> usize {
            let dx = if self.0 > other.0 {
                self.0 - other.0
            } else {
                other.0 - self.0
            };
            let dy = if self.1 > other.1 {
                self.1 - other.1
            } else {
                other.1 - self.1
            };
            let dz = if self.2 > other.2 {
                self.2 - other.2
            } else {
                other.2 - self.2
            };
            dx * dx + dy * dy + dz * dz
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let points = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let coords = line
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                Point(coords[0], coords[1], coords[2])
            })
            .collect::<Vec<Point>>();
        let mut pairs = points
            .iter()
            .enumerate()
            .flat_map(|(i, &p1)| {
                points
                    .iter()
                    .skip(i + 1)
                    .map(move |&p2| (p1, p2, p1.dist_sq(&p2)))
            })
            .collect::<Vec<(Point, Point, usize)>>();
        pairs.sort_by_key(|&(_, _, dist_sq)| dist_sq);
        let n = if points.len() == 20 { 10 } else { 1000 };
        let mut set_maps = HashMap::new();
        let mut set_id = 0;
        for &(p1, p2, _) in pairs.iter().take(n) {
            let id1 = *set_maps.entry(p1).or_insert_with(|| {
                let id = set_id;
                set_id += 1;
                id
            });
            let id2 = *set_maps.entry(p2).or_insert_with(|| {
                let id = set_id;
                set_id += 1;
                id
            });
            if id1 != id2 {
                let old_id = id2;
                for (_, v) in set_maps.iter_mut() {
                    if *v == old_id {
                        *v = id1;
                    }
                }
            }
        }
        let mut groups = HashMap::new();
        for (&point, &id) in set_maps.iter() {
            groups.entry(id).or_insert_with(Vec::new).push(point);
        }
        let top_3 = groups
            .values()
            .map(|v| v.len())
            .collect::<Vec<usize>>()
            .into_iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .product();
        Ok(top_3)
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);

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
                let line = line.unwrap();
                let coords = line
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                Point(coords[0], coords[1], coords[2])
            })
            .collect::<Vec<Point>>();
        let mut pairs = points
            .iter()
            .enumerate()
            .flat_map(|(i, &p1)| {
                points
                    .iter()
                    .skip(i + 1)
                    .map(move |&p2| (p1, p2, p1.dist_sq(&p2)))
            })
            .collect::<Vec<(Point, Point, usize)>>();
        pairs.sort_by_key(|&(_, _, dist_sq)| dist_sq);
        let mut set_maps = HashMap::new();
        let mut set_id = 0;
        for &(p1, p2, _) in pairs.iter() {
            let id1 = set_maps.get(&p1).copied();
            let id2 = set_maps.get(&p2).copied();
            match (id1, id2) {
                (Some(id1), Some(id2)) => {
                    if id1 != id2 {
                        let old_id = id2;
                        for (_, v) in set_maps.iter_mut() {
                            if *v == old_id {
                                *v = id1;
                            }
                        }
                    }
                }
                (Some(id1), None) => {
                    set_maps.insert(p2, id1);
                }
                (None, Some(id2)) => {
                    set_maps.insert(p1, id2);
                }
                (None, None) => {
                    let id = set_id;
                    set_id += 1;
                    set_maps.insert(p1, id);
                    set_maps.insert(p2, id);
                }
            }
            if set_maps.len() == points.len() {
                let first_id = *set_maps.values().next().unwrap();
                if set_maps.values().all(|&id| id == first_id) {
                    return Ok(p1.0 * p2.0);
                }
            }
        }
        Ok(0)
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
