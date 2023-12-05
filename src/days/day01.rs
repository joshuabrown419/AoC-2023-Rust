use crate::{Solution, SolutionPair};
use std::{fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

const DIGITS : [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day1.txt").expect("");
    // Your solution here...
    let mut sol1: usize = 0;

    let mut f1 : usize = 0;
    let mut l1 : usize = 0;
    for line in input.split("\n") {
        for c in line.chars() {
            if c.is_ascii_digit() {
                f1 = c as usize - 0x30;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                l1 = c as usize - 0x30;
                break;
            }
        }

        sol1 += f1 * 10 + l1
    }

    let mut sol2: usize = 0;
    let mut f2 : usize = 0;
    let mut l2 : usize = 0;
    for line in input.split("\n") {
        let chars : Vec<char> = line.chars().collect();
        for i in 0..line.len() {
            if chars[i].is_ascii_digit() {
                f2 = chars[i] as usize - 0x30;
                break;
            }

            if i > 1 && DIGITS.contains(&String::from_iter(&chars[i-2..i+1]).as_str()){
                f2 = slice_to_usize(&chars[i-2..i+1]);
                break;
            } else if i > 2 && DIGITS.contains(&String::from_iter(&chars[i-3..i+1]).as_str()) {
                f2 = slice_to_usize(&chars[i-3..i+1]);
                break;
            } else if i > 3 && DIGITS.contains(&String::from_iter(&chars[i-4..i+1]).as_str()) {
                f2 = slice_to_usize(&chars[i-4..i+1]);
                break;
            }
        }

        for j in (0..line.len()).rev() {
            if chars[j].is_ascii_digit() {
                l2 = chars[j] as usize - 0x30;
                break;
            }

            if j > 1 && DIGITS.contains(&String::from_iter(&chars[j-2..j+1]).as_str()){
                l2 = slice_to_usize(&chars[j-2..j+1]);
                break;
            } else if j > 2 && DIGITS.contains(&String::from_iter(&chars[j-3..j+1]).as_str()) {
                l2 = slice_to_usize(&chars[j-3..j+1]);
                break;
            } else if j > 3 && DIGITS.contains(&String::from_iter(&chars[j-4..j+1]).as_str()) {
                l2 = slice_to_usize(&chars[j-4..j+1]);
                break;
            }
        }
        sol2 += f2 * 10 + l2
    }

    (Solution::from(sol1), Solution::from(sol2))
}

pub fn slice_to_usize(input: &[char]) -> usize {
    if input == ['o', 'n', 'e'] {
        return 1;
    } else if input == ['t', 'w', 'o'] {
        return 2;
    } else if input == ['t', 'h', 'r', 'e', 'e'] {
        return 3;
    } else if input == ['f', 'o', 'u', 'r'] {
        return 4;
    } else if input == ['f', 'i', 'v', 'e'] {
        return 5;
    } else if input == ['s', 'i', 'x'] {
        return 6;
    } else if input == ['s', 'e', 'v', 'e', 'n'] {
        return 7;
    } else if input == ['e', 'i', 'g', 'h', 't'] {
        return 8;
    } else if input == ['n', 'i', 'n', 'e'] {
        return 9;
    }
    return 0;
}
