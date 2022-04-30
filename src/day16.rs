//! # [Day 16: Aunt Sue](https://adventofcode.com/2015/day/16)
//!
//! Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card.
//! However, there's a small problem: she signed it "From, Aunt Sue".
//!
//! You have 500 Aunts named "Sue".
//!
//! So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue
//! (which you conveniently number 1 to 500, for sanity) gave you the gift.
//! You open the present and, as luck would have it, good ol' Aunt Sue got you a
//! My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.
//!
//! The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific compounds
//! in a given sample, as well as how many distinct kinds of those compounds there are.
//! According to the instructions, these are what the MFCSAM can detect:
//!
//! -   `children`, by human DNA age analysis.
//! -   `cats`. It doesn't differentiate individual breeds.
//! -   Several seemingly random breeds of dog:
//!     `[samoyeds](https://en.wikipedia.org/wiki/Samoyed_%28dog%29)`,
//!     `[pomeranians](https://en.wikipedia.org/wiki/Pomeranian_%28dog%29)`,    
//!     `[akitas](https://en.wikipedia.org/wiki/Akita_%28dog%29)`, and
//!     `[vizslas](https://en.wikipedia.org/wiki/Vizsla)`.
//! -   `goldfish`. No other kinds of fish.
//! -   `trees`, all in one group.
//! -   `cars`, presumably by exhaust or gasoline or something.
//! -   `perfumes`, which is handy, since many of your Aunts Sue wear a few kinds.
//!
//! In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift
//! into the MFCSAM. It beeps inquisitively at you a few times and then prints out a
//! message on [ticker tape](https://en.wikipedia.org/wiki/Ticker_tape):
//!
//! ```plain
//! children: 3
//! cats: 7
//! samoyeds: 2
//! pomeranians: 3
//! akitas: 0
//! vizslas: 0
//! goldfish: 5
//! trees: 3
//! cars: 2
//! perfumes: 1
//! ```
//!
//! You make a list of the things you can remember about each Aunt Sue.
//! Things missing from your list aren't zero - you simply don't remember the value.
//!
//! **What is the number of the Sue that got you the gift?**
//!
//! # Part Two
//!
//! As you're about to send the thank you note, something in the MFCSAM's instructions catches your
//! eye. Apparently, it has an outdated
//! [retroencabulator](https://www.youtube.com/watch?v=RXJKdh1KZ0w), and so the output from the
//! machine isn't exact values - some of them indicate ranges.
//!
//! In particular, the `cats` and `trees` readings indicates that there are greater than that many
//! (due to the unpredictable nuclear decay of cat dander and tree pollen), while the
//! `pomeranians` and `goldfish` readings indicate that there are fewer than that many
//! (due to the modial interaction of magnetoreluctance).
//!
//! **What is the number of the real Aunt Sue?**

use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day16)]
fn parse_input(input: &str) -> anyhow::Result<Vec<Sue>> {
    let mut list: Vec<Sue> = Vec::new();
    //Sue 1: children: 1, cars: 8, vizslas: 7
    let re = Regex::new(r"^Sue (?P<nr>\d+): (?P<data>.+)$")?;
    for line in input.lines() {
        if let Some(matches) = re.captures(line) {
            let nr = matches.name("nr").unwrap().as_str().parse()?;
            let data = matches.name("data").unwrap().as_str().to_string();
            let mut map = HashMap::new();
            for datum in data.split(", ") {
                let parts: Vec<&str> = datum.split(": ").collect();
                map.insert(parts[0].to_string(), parts[1].parse()?);
            }
            list.push(Sue { nr, data: map })
        } else {
            return Err(anyhow!("failed to parse: {}", line));
        }
    }

    Ok(list)
}

/// Part 1: What is the number of the Sue that got you the gift?
#[aoc(day16, part1)]
fn part1(input: &[Sue]) -> u16 {
    find_sue1(input, &get_search()).unwrap()
}

fn find_sue1(sues: &[Sue], search: &HashMap<String, i32>) -> Option<u16> {
    for sue in sues {
        let mut possible = true;
        for key in search.keys() {
            if sue.data.contains_key(key) && sue.data.get(key) != search.get(key) {
                possible = false;
                break;
            }
        }
        if possible {
            return Some(sue.nr);
        }
    }
    None
}

/// Part 2: What is the number of the real Aunt Sue?
#[aoc(day16, part2)]
fn part2(input: &[Sue]) -> u16 {
    find_sue2(input, &get_search()).unwrap()
}

fn find_sue2(sues: &[Sue], search: &HashMap<String, i32>) -> Option<u16> {
    for sue in sues {
        let mut possible = true;
        for key in search.keys() {
            if sue.data.contains_key(key)
                && match key.as_str() {
                    "cats" | "trees" => sue.data.get(key) <= search.get(key),
                    "pomeranians" | "goldfish" => sue.data.get(key) >= search.get(key),
                    _ => sue.data.get(key) != search.get(key),
                }
            {
                possible = false;
                break;
            }
        }
        if possible {
            return Some(sue.nr);
        }
    }
    None
}

#[derive(Debug)]
struct Sue {
    nr: u16,
    data: HashMap<String, i32>,
}

fn get_search() -> HashMap<String, i32> {
    HashMap::from([
        ("children".into(), 3),
        ("cats".into(), 7),
        ("samoyeds".into(), 2),
        ("pomeranians".into(), 3),
        ("akitas".into(), 0),
        ("vizslas".into(), 0),
        ("goldfish".into(), 5),
        ("trees".into(), 3),
        ("cars".into(), 2),
        ("perfumes".into(), 1),
    ])
}
