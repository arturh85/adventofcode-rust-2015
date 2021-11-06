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

struct TableRule {
    left: String,
    right: String,
    delta: i64,
}

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
        let parts: Vec<&str> = line.split(' ').to_vec();
        // Alice would gain 54 happiness units by sitting next to Bob.
        // 0     1     2    3  4         5     6  7       8    9  10
        let left = parts[0].to_string();
        let right = parts[10].to_string();
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
        if !rules.contains_key(&right) {
            rules.insert(left.clone(), HashMap::new());
        }
        rules.get_mut(&left).unwrap().insert(right.clone(), delta);
        rules.get_mut(&right).unwrap().insert(left.clone(), delta);
    }

    Ok(TableRuleset { names, rules })
}

/// What is the total change in happiness for the optimal seating arrangement of the
/// actual guest list?
#[aoc(day13, part1)]
fn part1(input: &TableRuleset) -> usize {
    let mut best = i64::MIN;
    for perm in input.names.iter().permutations(input.names.len()) {}
    0
}

fn calc_happiness(seating: &Vec<&String>, rules: &Rules) -> i64 {
    let mut by_name: HashMap<String, i64> = HashMap::new();
    for name in seating {
        by_name.insert((*name).clone(), 0);
    }

    for pair in seating.windows(2) {
        by_name[pair[0]] += rules.get(pair[0]).unwrap().get(pair[1]).unwrap();
        by_name[pair[1]] += rules.get(pair[1]).unwrap().get(pair[1]).unwrap();
    }

    // table2 = vcat(table[2:end], table[1])
    //
    // for (a,b) in zip(table, table2) {
    //     for change in filter(r -> r.personA == a && r.personB == b, rules) {
    //         by_name[a] += change.value
    //     }
    //     for change in filter(r -> r.personA == b && r.personB == a, rules) {
    //         by_name[b] += change.value
    //     }
    // }

    // sum(values(by_name))
    todo!();
}

/// What is the total change in happiness for the optimal seating arrangement that actually
/// includes yourself?
// #[aoc(day12, part2)]
// fn part2(input: &str) -> usize {
//     todo!();
// }

// calc_happiness(["Alice", "Bob", "Carol", "David"], map(parseline, split(example_input, "\n")))

/*

fn parseline(line) {
    m = match(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)", line)
    (
        personA=m[1],
        value=m[2] == "gain" ? parse(Int, m[3]) : -parse(Int, m[3]),
        personB=m[4]
    )
}

# map(parseline, split(example_input, "\n"))


calc_happiness(["Alice", "Bob", "Carol", "David"], map(parseline, split(example_input, "\n")))

fn mosthappy1(rules) {
    people = collect(Set(map(rule -> rule.personA, rules)))

    maxhappiness = 0
    for perm in permutations(people) {
        maxhappiness = max(maxhappiness, calc_happiness(perm, rules))
    }
    maxhappiness
}

@assert mosthappy1(map(parseline, split(example_input, "\n"))) == 330

part1 = mosthappy1(map(parseline, split(puzzle_input, "\n")))


fn mosthappy2(rules) {
    people = collect(Set(map(rule -> rule.personA, rules)))
    me_rules = copy(rules)

    for person in people {
        push!(rules, (personA="me", value=0, personB=person))
        push!(rules, (personA=person, value=0, personB="me"))
    }

    push!(people, "me")
    maxhappiness = 0
    for perm in permutations(people) {
        maxhappiness = max(maxhappiness, calc_happiness(perm, rules))
    }
    maxhappiness
}

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
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
David would gain 41 happiness units by sitting next to Carol.";
    }

    #[test]
    fn part2_examples() {}
}
