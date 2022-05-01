//! # [Day 17: No Such Thing as Too Much](https://adventofcode.com/2015/day/17)
//!
//! The elves bought too much eggnog again - `150` liters this time.
//! To fit it all into your refrigerator, you'll need to move it into smaller containers.
//! You take an inventory of the capacities of the available containers.
//!
//! For example, suppose you have containers of size `20`, `15`, `10`, `5`, and `5` liters.
//! If you need to store `25` liters, there are four ways to do it:
//!
//! - `15` and `10`
//! - `20` and `5` (the first `5`)
//! - `20` and `5` (the second `5`)
//! - `15`, `5`, and `5`
//!
//! Filling all containers entirely, how many different combinations of containers can
//! exactly fit all `150` liters of eggnog?
//!
//! # Part Two
//!
//! While playing with all the containers in the kitchen, another load of eggnog arrives!
//! The shipping and receiving department is requesting as many containers as you can spare.
//!
//! Find the minimum number of containers that can exactly fit all `150` liters of eggnog.
//! How many different ways can you fill that number of containers and
//! still hold exactly `150` litres?
//!
//! In the example above, the minimum number of containers was two.
//! There were three ways to use that many containers, and so the answer there would be `3`.

use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Part 1: Filling all containers entirely, how many different combinations of containers can
/// exactly fit all `150` liters of eggnog?
#[aoc(day17, part1)]
fn part1(input: &[u64]) -> usize {
    build_combinations(input, 150).len()
}

fn build_combinations(input: &[u64], target: u64) -> Vec<Vec<u64>> {
    let mut combinations = Vec::new();
    for k in 1..=input.len() {
        for comb in input.iter().combinations(k) {
            if comb.iter().map(|n| **n).sum::<u64>() == target {
                combinations.push(comb.iter().map(|n| **n).collect());
            }
        }
    }
    combinations
}

/// Part 2: Find the minimum number of containers that can exactly fit all `150` liters of eggnog.
/// How many different ways can you fill that number of containers and
/// still hold exactly `150` litres?
#[aoc(day17, part2)]
fn part2(input: &[u64]) -> usize {
    get_min_count(input, 150)
}

fn get_min_count(input: &[u64], target: u64) -> usize {
    let combinations = build_combinations(input, target);
    let count_map = build_count_map(&combinations);
    let min = count_map.keys().min().unwrap();
    *count_map.get(min).unwrap()
}

fn build_count_map(combinations: &Vec<Vec<u64>>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for entry in combinations {
        let len = entry.len();
        map.entry(len).or_insert(0);
        *map.get_mut(&len).unwrap() += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "20
15
10
5
5";

    #[test]
    fn part1_examples() {
        // For example, suppose you have containers of size `20`, `15`, `10`, `5`, and `5` liters.
        // If you need to store `25` liters, there are four ways to do it:
        assert_eq!(4, build_combinations(&parse_input(EXAMPLE), 25).len());
    }

    #[test]
    fn part2_examples() {
        // In the example above, the minimum number of containers was two.
        // There were three ways to use that many containers, and so the answer there would be `3`.
        assert_eq!(3, get_min_count(&parse_input(EXAMPLE), 25));
    }
}
