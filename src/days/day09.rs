use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn figure_dir(dir: &str) -> (i32, i32) {
    match dir {
        "R" => (1, 0),
        "U" => (0, 1),
        "L" => (-1, 0),
        "D" => (0, -1),
        _ => panic!("fuck"),
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09").unwrap();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let mut line_as_iter = line.split_ascii_whitespace();
        let dir = figure_dir(line_as_iter.next().unwrap());
        let num = str::parse::<i32>(line_as_iter.next().unwrap()).unwrap();

        for _n in 0..num {
            head = (head.0 + dir.0, head.1 + dir.1);

            match (head.0 - tail.0, head.1 - tail.1) {
                (2, 0) => {
                    tail.0 += 1;
                }
                (-2, 0) => {
                    tail.0 -= 1;
                }
                (0, 2) => {
                    tail.1 += 1;
                }
                (0, -2) => {
                    tail.1 -= 1;
                }
                (2, 1) | (1, 2) => {
                    tail.0 += 1;
                    tail.1 += 1;
                }
                (2, -1) | (1, -2) => {
                    tail.0 += 1;
                    tail.1 -= 1;
                }
                (-2, 1) | (-1, 2) => {
                    tail.0 -= 1;
                    tail.1 += 1;
                }
                (-2, -1) | (-1, -2) => {
                    tail.0 -= 1;
                    tail.1 -= 1;
                }
                _ => {
                    // do nothing case
                    // panic!("ruh roh shaggy")
                }
            }
            seen.insert(tail);
        }
    }

    // Your solution here...
    let sol1 = seen.len();
    let sol2 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
