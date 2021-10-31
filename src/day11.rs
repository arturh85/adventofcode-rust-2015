//! # [Day 11: Corporate Policy](https://adventofcode.com/2015/day/11)
//!
//! Santa's previous password expired, and he needs help choosing a new one.
//!
//! To help him remember his new password after the old one expires, Santa has devised a method of
//! coming up with a password based on the previous one. Corporate policy dictates that passwords
//! must be exactly eight lowercase letters (for security reasons), so he finds his new password by
//! incrementing his old password string repeatedly until it is valid.
//!
//! Incrementing is just like counting with numbers: `xx`, `xy`, `xz`, `ya`, `yb`, and so on.
//! Increase the rightmost letter one step; if it was `z`, it wraps around to `a`, and repeat
//! with the next letter to the left until one doesn't wrap around.
//!
//! Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional
//! password requirements:
//!
//! -   Passwords must include one increasing straight of at least three letters,
//!     like `abc`, `bcd`, `cde`, and so on, up to `xyz`.
//!     They cannot skip letters; `abd` doesn't count.
//! -   Passwords may not contain the letters `i`, `o`, or `l`, as these letters can be mistaken
//!     for other characters and are therefore confusing.
//! -   Passwords must contain at least two different, non-overlapping pairs of letters,
//!     like `aa`, `bb`, or `zz`.
//!
//! For example:
//!
//! -   `hijklmmn` meets the first requirement (because it contains the straight `hij`) but
//!     fails the second requirement requirement (because it contains `i` and `l`).
//! -   `abbceffg` meets the third requirement (because it repeats `bb` and `ff`) but fails
//!     the first requirement.
//! -   `abbcegjk` fails the third requirement, because it only has one double letter (`bb`).
//! -   The next password after `abcdefgh` is `abcdffaa`.
//! -   The next password after `ghijklmn` is `ghjaabcc`, because you eventually skip all the
//!     passwords that start with `ghi...`, since `i` is not allowed.
//!
//! **Given Santa's current password (your puzzle input), what should his next password be?**
//!
//! # Part Two
//!
//! **Santa's password expired again. What's the next one?**

/// Given Santa's current password (your puzzle input), what should his next password be?
#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    next_valid(input)
}

/// Santa's password expired again. What's the next one?
#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    let next = next_valid(input);
    next_valid(&next)
}

fn is_valid(s: &str) -> bool {
    has_straight(s) && has_no_confusing(s) && has_two_nonoverlapping_pair(s)
}

fn next_valid(s: &str) -> String {
    let mut s = s.to_owned();
    loop {
        s = next(&s);
        if is_valid(&s) {
            return s;
        }
    }
}

fn next(input: &str) -> String {
    let mut bytes: Vec<u8> = input.as_bytes().to_vec();
    let first = 'a' as u8;
    let last = 'z' as u8;
    let len = input.len();
    for idx in (0..len).rev() {
        let mut v = bytes[idx] + 1;
        if v > last {
            v -= (last - first) + 1;
            bytes[idx] = v;
        } else {
            bytes[idx] = v;
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn has_straight(s: &str) -> bool {
    let bytes = s.as_bytes();
    for idx in 0..bytes.len() - 2 {
        let a = bytes[idx] as u8;
        let b = bytes[idx + 1] as u8;
        let c = bytes[idx + 2] as u8;

        if b == a + 1 && c == b + 1 {
            return true;
        }
    }
    false
}

fn has_no_confusing(input: &str) -> bool {
    !input.contains('i') && !input.contains('o') && !input.contains('l')
}

// Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
fn has_two_nonoverlapping_pair(s: &str) -> bool {
    let b = s.as_bytes();
    for idx1 in 1..b.len() - 1 {
        if b[idx1] == b[idx1 + 1] {
            for idx2 in idx1 + 2..b.len() - 1 {
                if b[idx2] == b[idx2 + 1] {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        //! -   `hijklmmn` meets the first requirement (because it contains the straight `hij`) but
        //!     fails the second requirement requirement (because it contains `i` and `l`).
        //! -   `abbceffg` meets the third requirement (because it repeats `bb` and `ff`) but fails
        //!     the first requirement.
        //! -   `abbcegjk` fails the third requirement, because it only has one double letter (`bb`).
        //! -   The next password after `abcdefgh` is `abcdffaa`.
        //! -   The next password after `ghijklmn` is `ghjaabcc`, because you eventually skip all the
        //!     passwords that start with `ghi...`, since `i` is not allowed.
        assert_eq!(next("xx"), "xy");
        assert_eq!(next("xy"), "xz");
        assert_eq!(next("xz"), "ya");
        assert_eq!(next("ya"), "yb");

        assert_eq!(has_straight("hijklmmn"), true);
        assert_eq!(has_straight("abbceffg"), false);

        assert_eq!(has_no_confusing("hijklmmn"), false);
        assert_eq!(has_no_confusing("abbceffg"), true);

        assert_eq!(has_two_nonoverlapping_pair("abbceffg"), true);
    }
}
