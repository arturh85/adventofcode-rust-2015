//! # [Day 9: All in a Single Night](https://adventofcode.com/2015/day/9) ---
//!
//! Every year, Santa manages to deliver all of his presents in a single night.
//!
//! This year, however, he has some new locations to visit; his elves have provided him the
//! distances between every pair of locations. He can start and end at any two (different)
//! locations he wants, but he must visit each location exactly once. What is the shortest distance
//! he can travel to achieve this?
//!
//! For example, given the following distances:
//!
//! ```plain
//! London to Dublin = 464
//! London to Belfast = 518
//! Dublin to Belfast = 141
//! ```
//!
//! The possible routes are therefore:
//!
//! ```plain
//! Dublin -> London -> Belfast = 982
//! London -> Dublin -> Belfast = 605
//! London -> Belfast -> Dublin = 659
//! Dublin -> Belfast -> London = 659
//! Belfast -> Dublin -> London = 605
//! Belfast -> London -> Dublin = 982
//! ```
//!
//! The shortest of these is `London -> Dublin -> Belfast = 605`, and so
//! the answer is `605` in this example.
//!
//! **What is the distance of the shortest route?**
//!
//! # Part Two
//!
//! The next year, just to show off, Santa decides to take the route with
//! the longest distance instead.
//!
//! He can still start and end at any two (different) locations he wants, and he still must
//! visit each location exactly once.
//!
//! For example, given the distances above, the longest route would be `982`
//! via (for example) `Dublin -> London -> Belfast`.
//!
//! **What is the distance of the longest route?**

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> anyhow::Result<HashMap<String, HashMap<String, u64>>> {
    let mut routes = HashMap::new();
    // London to Dublin = 464
    let re = Regex::new(r"^(?P<source>\w+) to (?P<target>\w+) = (?P<distance>\d+)$")?;
    for line in input.lines() {
        if let Some(matches) = re.captures(line) {
            let source = matches.name("source").unwrap().as_str();
            let target = matches.name("target").unwrap().as_str();
            let distance = matches.name("distance").unwrap().as_str().parse()?;
            if !routes.contains_key(source) {
                routes.insert(source.to_string(), HashMap::new());
            }
            if !routes.contains_key(target) {
                routes.insert(target.to_string(), HashMap::new());
            }
            routes
                .get_mut(source)
                .unwrap()
                .insert(target.to_string(), distance);
            routes
                .get_mut(target)
                .unwrap()
                .insert(source.to_string(), distance);
        } else {
            panic!("failed to parse: {}", line)
        }
    }
    Ok(routes)
}

type RoutingMap = HashMap<String, HashMap<String, u64>>;

/// Part 1: What is the distance of the shortest route?
#[aoc(day9, part1)]
fn part1(routes: &RoutingMap) -> u64 {
    *lengths(routes)
        .iter()
        .min()
        .expect("routes should not be empty")
}

/// Part 2: What is the distance of the longest route?
#[aoc(day9, part2)]
fn part2(routes: &RoutingMap) -> u64 {
    *lengths(routes)
        .iter()
        .max()
        .expect("routes should not be empty")
}

fn lengths(routes: &RoutingMap) -> Vec<u64> {
    routes
        .keys()
        .permutations(routes.keys().len())
        .map(|perm| {
            let mut len = 0;
            for flight in perm.windows(2) {
                let source = flight[0];
                let target = flight[1];
                len += routes[source][target];
            }
            len
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn part1_examples() {
        // The shortest of these is `London -> Dublin -> Belfast = 605`, and so
        // the answer is `605` in this example.
        assert_eq!(605, part1(&parse_input(EXAMPLE).unwrap()));
    }

    #[test]
    fn part2_examples() {
        // For example, given the distances above, the longest route would be `982`
        // via (for example) `Dublin -> London -> Belfast`.
        assert_eq!(982, part2(&parse_input(EXAMPLE).unwrap()));
    }
}
