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
use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> anyhow::Result<HashMap<String, HashMap<String, u64>>> {
    parse_routes(input)
}

type RoutingMap = HashMap<String, HashMap<String, u64>>;

/// What is the distance of the shortest route?
#[aoc(day9, part1)]
fn part1(routes: &RoutingMap) -> u64 {
    *lengths(routes)
        .iter()
        .min()
        .expect("routes should not be empty")
}

/// What is the distance of the longest route?
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

fn parse_routes(input: &str) -> anyhow::Result<RoutingMap> {
    let mut routes = HashMap::new();
    for line in input.lines() {
        let vec: Vec<&str> = line.split(" ").collect();
        if vec.len() != 5 {
            return Err(anyhow!("invalid format, expected 4 words per line"));
        }
        // London to Dublin = 464
        // 0      1  2      3 4
        let source = vec[0];
        let target = vec[2];
        let distance = vec[4];
        if !routes.contains_key(source) {
            routes.insert(source.to_string(), HashMap::new());
        }
        if !routes.contains_key(target) {
            routes.insert(target.to_string(), HashMap::new());
        }
        routes
            .get_mut(source)
            .unwrap()
            .insert(target.to_string(), distance.parse().unwrap());
        routes
            .get_mut(target)
            .unwrap()
            .insert(source.to_string(), distance.parse().unwrap());
    }
    Ok(routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let routes = parse_routes(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        )
        .expect("failed to parse example");

        // The shortest of these is `London -> Dublin -> Belfast = 605`, and so
        // the answer is `605` in this example.
        assert_eq!(part1(&routes), 605);
    }

    #[test]
    fn part2_examples() {
        let routes = parse_routes(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        )
        .expect("failed to parse example");

        // For example, given the distances above, the longest route would be `982`
        // via (for example) `Dublin -> London -> Belfast`.
        assert_eq!(part2(&routes), 982);
    }
}
