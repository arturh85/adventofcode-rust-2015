//! # [Day 19: Medicine for Rudolph](https://adventofcode.com/2015/day/19)
//!
//! Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly,
//! and he needs medicine.
//!
//! Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph is going to
//! need custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry isn't similar to regular
//! reindeer chemistry, either.
//!
//! The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant, capable of
//! constructing any Red-Nosed Reindeer molecule you need. It works by starting with some input
//! molecule and then doing a series of replacements, one per step, until it has the right molecule.
//!
//! However, the machine has to be calibrated before it can be used. Calibration involves
//! determining the number of molecules that can be generated in one step from a given
//! starting point.
//!
//! For example, imagine a simpler machine that supports only the following replacements:
//!
//! ```plain
//! H => HO
//! H => OH
//! O => HH
//!
//! ```
//!
//! Given the replacements above and starting with `HOH`, the following molecules
//! could be generated:
//!
//! - `HOOH` (via `H => HO` on the first `H`).
//! - `HOHO` (via `H => HO` on the second `H`).
//! - `OHOH` (via `H => OH` on the first `H`).
//! - `HOOH` (via `H => OH` on the second `H`).
//! - `HHHH` (via `O => HH`).
//!
//! So, in the example above, there are `4` distinct molecules (not five, because `HOOH`
//! appears twice) after one replacement from `HOH`. Santa's favorite molecule, `HOHOHO`,
//! can become `7` distinct molecules (over nine replacements: six from `H`, and three from `O`).
//!
//! The machine replaces without regard for the surrounding characters. For example,
//! given the string `H2O`, the transition `H => OO` would result in `OO2O`.
//!
//! Your puzzle input describes all of the possible replacements and, at the bottom, the medicine
//! molecule for which you need to calibrate the machine. **How many distinct molecules can be
//! created after all the different ways you can do one replacement on the medicine molecule?**
//!
//! # Part Two
//!
//! Now that the machine is calibrated, you're ready to begin molecule fabrication.
//!
//! Molecule fabrication always begins with just a single electron, `e`, and applying
//! replacements one at a time, just like the ones during calibration.
//!
//! For example, suppose you have the following replacements:
//!
//! ```plain
//! e => H
//! e => O
//! H => HO
//! H => OH
//! O => HH
//! ```
//!
//! If you'd like to make `HOH`, you start with `e`, and then make the following replacements:
//!
//! - `e => O` to get `O`
//! - `O => HH` to get `HH`
//! - `H => OH` (on the second `H`) to get `HOH`
//!
//! So, you could make `HOH` after `3` steps. Santa's favorite molecule, `HOHOHO`,
//! can be made in `6` steps.
//!
//! **How long will it take to make the medicine?** Given the available replacements and the medicine
//! molecule in your puzzle input, what is the fewest number of steps to go from `e` to the
//! medicine molecule?

use itertools::Itertools;
use std::collections::HashSet;

type Replacements = Vec<(String, String)>;

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (Replacements, String) {
    let mut puzzle = String::new();
    let delimiter = " => ";
    let mut entries: Replacements = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains(delimiter) {
            let parts: Vec<&str> = line.split(delimiter).collect();
            entries.push((parts[0].to_owned(), parts[1].to_owned()));
        } else {
            puzzle = line.into();
        }
    }
    entries.sort_by(|&(_, ref a), &(_, ref b)| a.len().cmp(&b.len()));
    (entries, puzzle)
}

/// Part 1: How many distinct molecules can be
/// created after all the different ways you can do one replacement on the medicine molecule?
#[aoc(day19, part1)]
fn part1(input: &(Replacements, String)) -> usize {
    let (replacements, puzzle) = input;
    let vals = apply(replacements, puzzle);
    vals.len()
}

fn apply(replacements: &Replacements, puzzle: &str) -> HashSet<String> {
    let mut mols: HashSet<String> = HashSet::new();
    for (from, to) in replacements {
        for res in variants(puzzle, from, to) {
            mols.insert(res);
        }
    }
    mols
}

fn variants(puzzle: &str, from: &str, to: &str) -> HashSet<String> {
    HashSet::from_iter(
        puzzle
            .single_replacements(from, to)
            .iter()
            .map(|s| s.to_owned()),
    )
}

/// Part 2: How long will it take to make the medicine? Given the available replacements and the
/// medicine molecule in your puzzle input, what is the fewest number of steps to go from `e` to
/// the medicine molecule?
#[aoc(day19, part2)]
fn part2(input: &(Replacements, String)) -> usize {
    let (replacements, puzzle) = input;
    find_production(replacements, puzzle, 0).unwrap()
}

fn find_production(replacements: &Replacements, input: &str, depth: usize) -> Option<usize> {
    if input == "e" {
        return Some(depth);
    }
    for next_step in replacements
        .iter()
        .flat_map(|&(ref from, ref to)| input.single_replacements(to, from).into_iter())
        .unique()
    {
        if let Some(count) = find_production(replacements, &next_step, depth + 1) {
            return Some(count);
        }
    }
    None
}

trait SingleReplacements {
    // like std::str::replace but returning a Vec<String> with one replacement in each
    fn single_replacements(&self, from: &str, to: &str) -> Vec<String>;
}

impl<'a> SingleReplacements for &'a str {
    fn single_replacements(&self, from: &str, to: &str) -> Vec<String> {
        let mut results = Vec::new();
        for (start, part) in self.match_indices(from) {
            let mut string = String::new();
            string.push_str(self.get(0..start).unwrap());
            string.push_str(to);
            string.push_str(self.get(start + part.len()..self.len()).unwrap());
            results.push(string);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "H => HO
H => OH
O => HH";

    const EXAMPLE_2: &str = "e => H
e => O
H => HO
H => OH
O => HH";

    #[test]
    fn part1_examples() {
        let (replacements, _) = parse_input(EXAMPLE_1);
        // So, in the example above, there are `4` distinct molecules
        let molecules = apply(&replacements, "HOH");
        assert_eq!(4, molecules.len());
        assert_eq!(
            HashSet::from_iter(vec![
                "HOOH".to_string(),
                "HOHO".to_string(),
                "OHOH".to_string(),
                "HHHH".to_string()
            ]),
            molecules
        );
    }

    #[test]
    fn part2_examples() {
        let (replacements, _) = parse_input(EXAMPLE_2);
        // So, you could make `HOH` after `3` steps.
        assert_eq!(3, find_production(&replacements, "HOH", 0).unwrap());
        // Santa's favorite molecule, `HOHOHO`, can be made in `6` steps.
        assert_eq!(6, find_production(&replacements, "HOHOHO", 0).unwrap());
    }
}
