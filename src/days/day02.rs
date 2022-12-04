use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01").unwrap();

    let rows = input.split("\n");

    enum Chosen {
        X = 1,
        Y = 2,
        Z = 3,
    };

    enum State {
        LOSS = 0,
        DRAW = 3,
        WIN = 6,
    };

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
