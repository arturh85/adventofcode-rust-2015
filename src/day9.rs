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

/// What is the distance of the shortest route?
#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let connections = parse_connections(input);
    shortest_connection(connections)
}

/// What is the distance of the longest route?
#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    let connections = parse_connections(input);
    longest_connection(connections)
}

fn shortest_connection(connections: HashMap<String, HashMap<String, u64>>) -> u64 {
    let mut shortest = u64::MAX;
    for perm in connections.keys().permutations(connections.keys().len()) {
        let mut len = 0;
        for flight in perm.windows(2) {
            let source = flight[0];
            let target = flight[1];
            len += connections[source][target];
        }
        if len < shortest {
            shortest = len;
        }
    }
    shortest
}

fn longest_connection(connections: HashMap<String, HashMap<String, u64>>) -> u64 {
    let mut longest = 0;
    for perm in connections.keys().permutations(connections.keys().len()) {
        let mut len = 0;
        for flight in perm.windows(2) {
            let source = flight[0];
            let target = flight[1];
            len += connections[source][target];
        }
        if len > longest {
            longest = len;
        }
    }
    longest
}

fn parse_connections(input: &str) -> HashMap<String, HashMap<String, u64>> {
    let mut m = HashMap::new();
    for line in input.lines() {
        let vec: Vec<&str> = line.split(" ").collect();
        let source = vec[0];
        let target = vec[2];
        let distance = vec[4];
        if !m.contains_key(source) {
            m.insert(source.to_string(), HashMap::new());
        }
        if !m.contains_key(target) {
            m.insert(target.to_string(), HashMap::new());
        }
        m.get_mut(source)
            .unwrap()
            .insert(target.to_string(), distance.parse().unwrap());
        m.get_mut(target)
            .unwrap()
            .insert(source.to_string(), distance.parse().unwrap());
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let map = parse_connections(input);
        assert_eq!(shortest_connection(map), 605);
    }

    #[test]
    fn part2_examples() {}
}
