//! # [Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1)
//!
//! Santa was hoping for a white Christmas, but his weather machine's "snow"
//! function is powered by stars, and he's fresh out! To save Christmas,
//! he needs you to collect fifty stars by December 25th.
//!
//! Collect stars by helping Santa solve puzzles. Two puzzles will be made
//! available on each day in the Advent calendar; the second puzzle is unlocked
//! when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Here's an easy puzzle to warm you up.
//!
//! Santa is trying to deliver presents in a large apartment building, but he
//! can't find the right floor - the directions he got are a little confusing.
//! He starts on the ground floor (floor `0`) and then follows the instructions
//! one character at a time.
//!
//! An opening parenthesis, `(`, means he should go up one floor, and a closing
//! parenthesis, `)`, means he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep; he will
//! never find the top or bottom floors.
//!
//! For example:
//!
//! -   `(())` and `()()` both result in floor `0`.
//! -   `(((` and `(()(()(` both result in floor `3`.
//! -   `))(((((` also results in floor `3`.
//! -   `())` and `))(` both result in floor `-1` (the first basement level).
//! -   `)))` and `)())())` both result in floor `-3`.
//!
//! **To what floor do the instructions take Santa?**
//!
//! # Part Two
//! Now, given the same instructions, find the position of the first character that
//! causes him to enter the basement (floor  `-1`). The first character in the
//! instructions has position  `1`, the second character has
//! position  `2`, and so on.
//!
//! For example:
//!
//! -   `)`  causes him to enter the basement at character position  `1`.
//! -   `()())`  causes him to enter the basement at character position  `5`.
//!
//! **What is the position of the character that causes Santa to first
//! enter the basement?**

/// Part 1, procedural style
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for char in input.chars() {
        match char {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => unreachable!(),
        }
    }
    sum
}

/// Part 1, functional style
#[aoc(day1, part1, alt1)]
pub fn part1_alt1(input: &str) -> i32 {
    input
        .chars()
        .map(|char| match char {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}

/// Part 2
#[aoc(day1, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut sum: i32 = 0;
    for (idx, char) in input.chars().enumerate() {
        match char {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => {}
        }
        if sum < 0 {
            return Some(idx + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // `(())` and `()()` both result in floor `0`.
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        // `(((` and `(()(()(` both result in floor `3`.
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        // `))(((((` also results in floor `3`.
        assert_eq!(part1("))((((("), 3);
        // `())` and `))(` both result in floor `-1` (the first basement level).
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        // `)))` and `)())())` both result in floor `-3`.
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }
    #[test]
    fn part2_example() {
        // `)` causes him to enter the basement at character position `1`.
        assert_eq!(part2(")"), Some(1));
        // `()())` causes him to enter the basement at character position `5`.
        assert_eq!(part2("()())"), Some(5));
    }
}
