//! # [Day 13: Knights of the Dinner Table](https://adventofcode.com/2015/day/13)
//!
//! In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along!
//! This year, you resolve, will be different. You're going to find the optimal seating arrangement
//! and avoid all those awkward conversations.
//!
//! You start by writing up a list of everyone invited and the amount their happiness would increase
//! or decrease if they were to find themselves sitting next to each other person.
//! You have a circular table that will be just big enough to fit everyone comfortably,
//! and so each person will have exactly two neighbors.
//!
//! For example, suppose you have only four attendees planned, and you calculate their potential
//! happiness as follows:
//!
//! ```plain
//! Alice would gain 54 happiness units by sitting next to Bob.
//! Alice would lose 79 happiness units by sitting next to Carol.
//! Alice would lose 2 happiness units by sitting next to David.
//! Bob would gain 83 happiness units by sitting next to Alice.
//! Bob would lose 7 happiness units by sitting next to Carol.
//! Bob would lose 63 happiness units by sitting next to David.
//! Carol would lose 62 happiness units by sitting next to Alice.
//! Carol would gain 60 happiness units by sitting next to Bob.
//! Carol would gain 55 happiness units by sitting next to David.
//! David would gain 46 happiness units by sitting next to Alice.
//! David would lose 7 happiness units by sitting next to Bob.
//! David would gain 41 happiness units by sitting next to Carol.
//! ```
//!
//! Then, if you seat Alice next to David, Alice would lose `2` happiness units
//! (because David talks so much), but David would gain `46` happiness units (because Alice is
//! such a good listener), for a total change of `44`.
//!
//! If you continue around the table, you could then seat Bob next to Alice (Bob gains `83`, Alice
//! gains `54`). Finally, seat Carol, who sits next to Bob (Carol gains `60`, Bob loses `7`) and
//! David (Carol gains `55`, David gains `41`). The arrangement looks like this:
//!
//! ```plain
//!      +41 +46
//! +55   David    -2
//! Carol       Alice
//! +60    Bob    +54
//!      -7  +83
//! ```
//!
//! After trying every other seating arrangement in this hypothetical scenario,
//! you find that this one is the most optimal, with a total change in happiness of `330`.
//!
//! **What is the total change in happiness for the optimal seating arrangement of the
//! actual guest list?**
//!
//! # Part Two
//!
//! In all the commotion, you realize that you forgot to seat yourself. At this point, you're
//! pretty apathetic toward the whole thing, and your happiness wouldn't really go up or down
//! regardless of who you sit next to. You assume everyone else would be just as ambivalent about
//! sitting next to you, too.
//!
//! So, add yourself to the list, and give all happiness relationships that involve you a
//! score of `0`.
//!
//! **What is the total change in happiness for the optimal seating arrangement that actually
//! includes yourself?**

use itertools::Itertools;
use std::collections::HashMap;

type Rules = HashMap<String, HashMap<String, i64>>;

struct TableRuleset {
    rules: Rules,
    names: Vec<String>,
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> anyhow::Result<TableRuleset> {
    let mut names: Vec<String> = Vec::new();
    let mut rules: Rules = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        // Alice would gain 54 happiness units by sitting next to Bob.
        // 0     1     2    3  4         5     6  7       8    9  10
        let left = parts[0].to_string();
        let mut right = parts[10].to_string();
        right = right[0..right.len() - 1].to_string(); // remove last char
        let mut delta: i64 = parts[3].parse()?;
        if parts[2] == "lose" {
            delta *= -1;
        }
        if !names.contains(&left) {
            names.push(left.clone());
        }
        if !names.contains(&right) {
            names.push(right.clone());
        }

        if !rules.contains_key(&left) {
            rules.insert(left.clone(), HashMap::new());
        }
        rules.get_mut(&left).unwrap().insert(right.clone(), delta);
    }

    Ok(TableRuleset { names, rules })
}

/// What is the total change in happiness for the optimal seating arrangement of the
/// actual guest list?
#[aoc(day13, part1)]
fn part1(input: &TableRuleset) -> i64 {
    input.best_happiness()
}

#[aoc(day13, part2)]
fn part2(input: &TableRuleset) -> i64 {
    add_yourself(input).best_happiness()
}

impl TableRuleset {
    fn best_happiness(&self) -> i64 {
        let mut best = i64::MIN;
        for perm in self.names.iter().permutations(self.names.len()) {
            let happiness = calc_happiness(&perm, &self.rules);
            if happiness > best {
                best = happiness;
            }
        }
        best
    }
}

fn add_yourself(rules: &TableRuleset) -> TableRuleset {
    let mut names = rules.names.clone();
    let mut rules = rules.rules.clone();
    names.push("me".into());
    rules.insert("me".into(), HashMap::new());

    TableRuleset { names, rules }
}

fn get_delta(rules: &Rules, left: &str, right: &str) -> i64 {
    *rules.get(left).unwrap().get(right).unwrap_or(&0)
}

fn calc_happiness(seating: &Vec<&String>, rules: &Rules) -> i64 {
    let mut by_name: HashMap<String, i64> = HashMap::new();
    for name in seating {
        by_name.insert((*name).clone(), 0);
    }
    for pair in seating.windows(2) {
        *by_name.get_mut(pair[0]).unwrap() += get_delta(rules, pair[0], pair[1]);
        *by_name.get_mut(pair[1]).unwrap() += get_delta(rules, pair[1], pair[0]);
    }
    let last_idx = seating.len() - 1;
    *by_name.get_mut(seating[0]).unwrap() += get_delta(rules, seating[0], seating[last_idx]);
    *by_name.get_mut(seating[last_idx]).unwrap() += get_delta(rules, seating[last_idx], seating[0]);
    by_name.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let rules = parse_input(
            "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        )
        .expect("failed to parse");
        assert_eq!(330, rules.best_happiness());
    }
}
