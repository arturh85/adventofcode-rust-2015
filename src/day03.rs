//! # [Day 3: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3)
//!
//! Santa is delivering presents to an infinite two-dimensional grid of houses.
//!
//! He begins by delivering a present to the house at his starting location, and
//! then an elf at the North Pole calls him via radio and tells him where to move
//! next. Moves are always exactly one house to the north (`^`), south (`v`),
//! east (`>`), or west (`<`). After each move, he delivers another present to the
//! house at his new location.
//!
//! However, the elf back at the north pole has had a little too much eggnog, and
//! so his directions are a little off, and Santa ends up visiting some houses more
//! than once. **How many houses receive at least one present?**
//!
//! For example:
//!
//! -   `>` delivers presents to `2` houses: one at the starting location, and one
//!     to the east.
//! -   `^>v<` delivers presents to `4` houses in a square, including twice to the
//!     house at his starting/ending location.
//! -   `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at
//!     only `2` houses.
//! --- Part Two ---
//!
//! The next year, to speed up the process, Santa creates a robot version of himself,
//! Robo-Santa, to deliver presents with him.
//!
//! Santa and Robo-Santa start at the same location (delivering two presents to the same
//! starting house), then take turns moving based on instructions from the elf, who is eggnoggedly
//! reading from the same script as the previous year.
//!
//! This year, how many houses receive at least one present?
//!
//! For example:
//!
//! -   `^v` delivers presents to `3` houses, because Santa goes north, and then Robo-Santa goes south.
//! -   `^>v<` now delivers presents to `3` houses, and Santa and Robo-Santa end up back where
//! they started.
//! -   `^v^v^v^v^v` now delivers presents to `11` houses, with Santa going one direction
//! and Robo-Santa going the other.

use itertools::Itertools;
use std::collections::HashMap;

/// Part 1
#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    steps(input).len()
}

/// Part 2
#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    split_merge_steps(input)
}

type Point = (i32, i32);

fn steps(s: &str) -> HashMap<Point, u32> {
    let mut history: HashMap<Point, u32> = HashMap::new();
    let mut position: Point = (0, 0);
    history.insert(position, 1);
    for c in s.chars() {
        position = match c {
            '^' => (position.0, position.1 - 1),
            'v' => (position.0, position.1 + 1),
            '<' => (position.0 - 1, position.1),
            '>' => (position.0 + 1, position.1),
            _ => unreachable!(),
        };
        *history.entry(position).or_insert(0) += 1;
    }
    history
}

fn split_merge_steps(s: &str) -> usize {
    let (even, odd): (String, String) = s.chars().enumerate().partition_map(|(idx, c)| {
        if idx % 2 == 0 {
            itertools::Either::Left(c)
        } else {
            itertools::Either::Right(c)
        }
    });
    let mut even = steps(&even);
    let odd = steps(&odd);
    even.extend(odd.into_iter());
    return even.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        /* `>` delivers presents to `2` houses: one at the starting location, and one
        to the east. */
        assert_eq!(steps(">").len(), 2);

        /* `^>v<` delivers presents to `4` houses in a square, including twice to the
        house at his starting/ending location. */
        assert_eq!(steps("^>v<").len(), 4);

        /* `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at
        only `2` houses. */
        assert_eq!(steps("^v^v^v^v^v").len(), 2);
    }

    #[test]
    fn part2_example() {
        // `^v` delivers presents to `3` houses, because Santa goes north, and then Robo-Santa
        // goes south.
        assert_eq!(split_merge_steps(">"), 2);
        // `^>v<` now delivers presents to `3` houses, and Santa and Robo-Santa end up back where
        // they started.
        assert_eq!(split_merge_steps("^>v<"), 3);
        // `^v^v^v^v^v` now delivers presents to `11` houses, with Santa going one direction
        // and Robo-Santa going the other.
        assert_eq!(split_merge_steps("^v^v^v^v^v"), 11);
    }
}
