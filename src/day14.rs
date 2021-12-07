//! # [Day 14: Reindeer Olympics](https://adventofcode.com/2015/day/14)
//!
//! This year is the Reindeer Olympics!
//! Reindeer can fly at high speeds, but must rest occasionally to recover their energy. Santa
//! would like to know which of his reindeer is fastest, and so he has them race.
//!
//! Reindeer can only either be flying (always at their top speed) or resting (not moving at all),
//! and always spend whole seconds in either state.
//!
//! For example, suppose you have the following Reindeer:
//!
//! -   Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
//! -   Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
//!
//! After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds,
//! Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins
//! resting (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the
//! 12th second, both reindeer are resting. They continue to rest until the 138th second, when Comet
//! flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.
//!
//! In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead
//! at `1120` km (poor Dancer has only gotten `1056` km by that point). So, in this situation,
//! Comet would win (if the race ended at 1000 seconds).
//!
//! **Given the descriptions of each reindeer (in your puzzle input), after exactly `2503` seconds,
//! what distance has the winning reindeer traveled?**
//!
//! # Part Two
//!
//! Seeing how reindeer move in bursts, Santa decides he's not pleased with the old scoring system.
//!
//! Instead, at the end of each second, he awards one point to the reindeer currently in the lead.
//! (If there are multiple reindeer tied for the lead, they each get one point.)
//! He keeps the traditional 2503 second time limit, of course, as doing otherwise would be
//! entirely ridiculous.
//!
//! Given the example reindeer from above, after the first second, Dancer is in the lead and gets
//! one point. He stays in the lead until several seconds into Comet's second burst: after the 140th
//! second, Comet pulls into the lead and gets his first point. Of course, since Dancer had been in
//! the lead for the 139 seconds before that, he has accumulated 139 points by the 140th second.
//!
//! After the 1000th second, Dancer has accumulated `689` points, while poor Comet, our old
//! champion, only has `312`. So, with the new scoring system, Dancer would win (if the race ended
//! at 1000 seconds).
//!
//! **Again given the descriptions of each reindeer (in your puzzle input), after exactly `2503`
//! seconds, how many points does the winning reindeer have?**

use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> anyhow::Result<Vec<Reindeer>> {
    let mut reindeers: Vec<Reindeer> = Vec::new();
    // Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    let re = Regex::new(
        r"^(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<run_duration>\d+) seconds, but then must rest for (?P<rest_duration>\d+) seconds.$",
    )?;
    for line in input.lines() {
        if let Some(matches) = re.captures(line) {
            let name = matches.name("name").unwrap().as_str().to_string();
            let speed = matches.name("speed").unwrap().as_str().parse()?;
            let run_duration = matches.name("run_duration").unwrap().as_str().parse()?;
            let rest_duration = matches.name("rest_duration").unwrap().as_str().parse()?;
            reindeers.push(Reindeer {
                name,
                speed,
                run_duration,
                rest_duration,
            })
        } else {
            panic!("failed to parse: {}", line)
        }
    }

    Ok(reindeers)
}

/// Part 1: After exactly `2503` seconds, what distance has the winning reindeer traveled?
#[aoc(day14, part1)]
fn part1(input: &Vec<Reindeer>) -> u64 {
    best_distance_after(input, 2503)
}

/// Part 2: After exactly `2503` seconds, how many points does the winning reindeer have?
#[aoc(day14, part2)]
fn part2(input: &Vec<Reindeer>) -> u64 {
    best_points_after(input, 2503)
}

struct Reindeer {
    name: String,
    speed: u64,
    run_duration: u64,
    rest_duration: u64,
}

impl Reindeer {
    fn distance_after(&self, secs: u64) -> u64 {
        let cycle_distance = self.speed * self.run_duration;
        let cycle_duration = self.run_duration + self.rest_duration;
        let cycle_count = secs / cycle_duration;
        let prev_cycles_distance = cycle_distance * cycle_count;
        let remaining_secs = std::cmp::min(secs % cycle_duration, self.run_duration);
        prev_cycles_distance + remaining_secs * self.speed
    }
}

fn best_distance_after(reindeers: &Vec<Reindeer>, time: u64) -> u64 {
    reindeers
        .iter()
        .map(|reindeer| reindeer.distance_after(time))
        .max()
        .unwrap()
}

fn best_points_after(reindeers: &Vec<Reindeer>, time: u64) -> u64 {
    let mut points: HashMap<String, u64> = HashMap::new();
    for reindeer in reindeers {
        points.insert(reindeer.name.clone(), 0);
    }
    for t in 1..=time {
        let best_distance = best_distance_after(reindeers, t);
        for reindeer in reindeers {
            if reindeer.distance_after(t) == best_distance {
                *points.get_mut(&reindeer.name).unwrap() += 1;
            }
        }
    }
    *points.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn part1_examples() {
        let reindeers = parse_input(EXAMPLE).expect("failed to parse");
        assert_eq!(1120, best_distance_after(&reindeers, 1000));
    }

    #[test]
    fn part2_examples() {
        let reindeers = parse_input(EXAMPLE).expect("failed to parse");
        assert_eq!(689, best_points_after(&reindeers, 1000));
    }
}
