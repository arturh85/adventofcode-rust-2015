//! # [Day 10: Elves Look, Elves Say](https://adventofcode.com/2015/day/10)
//!
//! Today, the Elves are playing a game called [look-and-say](https://en.wikipedia.org/wiki/Look-and-say_sequence).
//! They take turns making sequences by reading aloud the previous sequence and using that reading
//! as the next sequence. For example, `211` is read as "one two, two ones", which
//! becomes `1221` (`1` `2`, `2` `1`s).
//!
//! Look-and-say sequences are generated iteratively, using the previous value as input for the
//! next step. For each step, take the previous value, and replace each run of digits (like `111`)
//! with the number of digits (`3`) followed by the digit itself (`1`).
//!
//! For example:
//!
//! -   `1` becomes `11` (`1` copy of digit `1`).
//! -   `11` becomes `21` (`2` copies of digit `1`).
//! -   `21` becomes `1211` (one `2` followed by one `1`).
//! -   `1211` becomes `111221` (one `1`, one `2`, and two `1`s).
//! -   `111221` becomes `312211` (three `1`s, two `2`s, and one `1`).
//!
//! Starting with the digits in your puzzle input, apply this process 40 times.
//! **What is the length of the result?**
//!
//! # Part Two
//!
//! Neat, right? You might also enjoy hearing [John Conway talking about this sequence](https://www.youtube.com/watch?v=ea7lJkEhytA)
//! (that's Conway of Conway's Game of Life fame).
//!
//! Now, starting again with the digits in your puzzle input, apply this process 50 times.
//! **What is the length of the new result?**

use itertools::Itertools;

/// Part 1: Starting with the digits in your puzzle input, apply this process 40 times.
/// What is the length of the result?
#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..40 {
        s = lookandsay(&s);
    }
    s.len()
}

/// Part 2: Now, starting again with the digits in your puzzle input, apply this process 50 times.
/// What is the length of the new result?
#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..50 {
        s = lookandsay(&s);
    }
    s.len()
}

fn group_chars(input: &str) -> Vec<String> {
    input
        .chars()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect()
}

fn lookandsay(input: &str) -> String {
    let mut s = String::new();
    for group in group_chars(input) {
        let count = group.len().to_string();
        let char = group.chars().next().unwrap().to_string();
        s += count.as_str();
        s += char.as_str();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        // `1` becomes `11` (`1` copy of digit `1`).
        assert_eq!(lookandsay("1"), "11");
        // `11` becomes `21` (`2` copies of digit `1`).
        assert_eq!(lookandsay("11"), "21");
        // `21` becomes `1211` (one `2` followed by one `1`).
        assert_eq!(lookandsay("21"), "1211");
        // `1211` becomes `111221` (one `1`, one `2`, and two `1`s).
        assert_eq!(lookandsay("1211"), "111221");
        // `111221` becomes `312211` (three `1`s, two `2`s, and one `1`).
        assert_eq!(lookandsay("111221"), "312211");
    }
}
