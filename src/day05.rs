//! # [Day 5: Doesn't He Have Intern-Elves For This?](https://adventofcode.com/2015/day/5)
//! Santa needs help figuring out which strings in his text file are naughty or nice.
//!
//! A nice string is one with all of the following properties:
//!
//! -   It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
//! -   It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`),
//!     or `aabbccdd` (`aa`, `bb`, `cc`, or `dd`).
//! -   It does not contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of
//!     the other requirements.
//!
//! For example:
//!
//! -   `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`),
//!     a double letter (`...dd...`), and none of the disallowed substrings.
//! -   `aaa` is nice because it has at least three vowels and a double letter, even though
//!     the letters used by different rules overlap.
//! -   `jchzalrnumimnmhp` is naughty because it has no double letter.
//! -   `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
//! -   `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
//!
//! **How many strings are nice?**
//!
//! # Part Two
//!
//! Realizing the error of his ways, Santa has switched to a better model of determining whether a
//! string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.
//!
//! Now, a nice string is one with all of the following properties:
//!
//! -   It contains a pair of any two letters that appears at least twice in the string without
//!     overlapping, like `xyxy` (`xy`) or `aabcdefgaa` (`aa`),
//!     but not like `aaa` (`aa`, but it overlaps).
//! -   It contains at least one letter which repeats with exactly one letter between them,
//!     like `xyx`, `abcdefeghi` (`efe`), or even `aaa`.
//!
//! For example:
//!
//! -   `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter
//!     that repeats with exactly one letter between them (`zxz`).
//! -   `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with one
//!     between, even though the letters used by each rule overlap.
//! -   `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single
//!     letter between them.
//! -   `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`),
//!     but no pair that appears twice.
//!
//! How many strings are nice under these new rules?

/// Part 1
#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|line| is_nice1(line)).count()
}

/// Part 2
#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|line| is_nice2(line)).count()
}

fn is_nice1(input: &str) -> bool {
    has_three_vowels(input) && has_double_letter(input) && !has_invalid(input)
}

fn is_nice2(input: &str) -> bool {
    has_pair_twice_without_overlapping(input) && has_repeat_with_gap(input)
}

fn has_three_vowels(s: &str) -> bool {
    s.chars()
        .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
        .count()
        >= 3
}
fn has_invalid(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn has_double_letter(s: &str) -> bool {
    let g = unicode_segmentation::UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    for i in 1..g.len() {
        if g[i - 1] == g[i] {
            return true;
        }
    }
    false
}

fn has_pair_twice_without_overlapping(input: &str) -> bool {
    let g = unicode_segmentation::UnicodeSegmentation::graphemes(input, true);
    let v = g.collect::<Vec<&str>>();
    for idx1 in 0..v.len() - 3 {
        for idx2 in (idx1 + 2)..v.len() - 1 {
            let first: String = v[idx1..idx1 + 2].iter().map(|s| s.to_string()).collect();
            let second: String = v[idx2..idx2 + 2].iter().map(|s| s.to_string()).collect();
            if first.eq(&second) {
                return true;
            }
        }
    }
    false
}

fn has_repeat_with_gap(input: &str) -> bool {
    let g =
        unicode_segmentation::UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    for idx in 1..g.len() - 2 {
        if g[idx] == g[idx + 2] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        // `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`),
        //     a double letter (`...dd...`), and none of the disallowed substrings.
        assert_eq!(is_nice1("ugknbfddgicrmopn"), true);

        // `aaa` is nice because it has at least three vowels and a double letter, even though
        //     the letters used by different rules overlap.
        assert_eq!(is_nice1("aaa"), true);

        // `jchzalrnumimnmhp` is naughty because it has no double letter.
        assert_eq!(is_nice1("jchzalrnumimnmhp"), false);

        // `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
        assert_eq!(is_nice1("haegwjzuvuyypxyu"), false);

        // `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
        assert_eq!(is_nice1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn part2_examples() {
        // `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter
        // that repeats with exactly one letter between them (`zxz`).
        assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true, "a");

        // `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with one
        // between, even though the letters used by each rule overlap.
        assert_eq!(is_nice2("xxyxx"), true, "b");

        // `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single
        //  letter between them.
        assert_eq!(is_nice2("uurcxstgmygtbstg"), false, "c");

        // `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`),
        //     but no pair that appears twice.
        assert_eq!(is_nice2("ieodomkazucvgmuy"), false, "f");
    }

    #[test]
    fn test_emojis() {
        // `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter
        // that repeats with exactly one letter between them (`zxz`).
        assert_eq!(has_double_letter("🐂🐂"), true);
    }
}
