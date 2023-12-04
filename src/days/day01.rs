use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day1.txt").expect("");
    // Your solution here...
    let mut sol1: u64 = 0;
    let lines = input.split("\n");

    let mut f1 : char = '0';
    let mut l1 : char = '0';
    for line in lines {
        for c in line.chars() {
            if c.is_ascii_digit() {
                f1 = c;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                l1 = c;
                break;
            }
        }

        sol1 += format!("{}{}", f1, l1).parse::<u64>().unwrap()
    }


    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
