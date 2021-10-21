//! # [Day 4: The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4)
//!
//! Santa needs help [mining](https://en.wikipedia.org/wiki/Bitcoin#Mining) some AdventCoins
//! (very similar to [bitcoins](https://en.wikipedia.org/wiki/Bitcoin)) to use as gifts for all
//! the economically forward-thinking little girls and boys.
//!
//! To do this, he needs to find [MD5](https://en.wikipedia.org/wiki/MD5) hashes which,
//! in [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal), start with at least five zeroes.
//! The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a
//! number in decimal. To mine AdventCoins, **you must find Santa the lowest positive
//! number (no leading zeroes: `1`, `2`, `3`, ...) that produces such a hash.**
//!
//! For example:
//!
//! -   If your secret key is `abcdef`, the answer is `609043`, because the MD5
//!     hash of `abcdef609043` starts with five zeroes (`000001dbbfa...`), and it is the
//!     lowest such number to do so.
//! -   If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash
//!     starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970`
//!     looks like `000006136ef...`
//!
//! # Part Two
//! Now find one that starts with **six zeroes**.

use crypto::digest::Digest;

/// increments a counter starting at 0 which is appended to `input` until `test` returns
/// true for the md5 hash buffer, then returns the counter
fn md5_suffix_increment_until(input: &str, test: fn(&[u8; 16]) -> bool) -> u64 {
    let mut hasher = crypto::md5::Md5::new();
    let mut output = [0; 16]; // An MD5 is 16 bytes
    for i in 0..u64::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut output);
        if test(&output) {
            return i;
        }
        hasher.reset();
    }
    0
}

/// Part 1
#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    md5_suffix_increment_until(input, |output| {
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        first_five == 0
    })
}

/// Part 2
#[aoc(day4, part2)]
fn part2(input: &str) -> u64 {
    md5_suffix_increment_until(input, |output| {
        let first_six = output[0] as i32 + output[1] as i32 + output[2] as i32;
        first_six == 0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        /* If your secret key is `abcdef`, the answer is `609043`, because the MD5
        hash of `abcdef609043` starts with five zeroes (`000001dbbfa...`), and it is the
        lowest such number to do so. */
        assert_eq!(part1("abcdef"), 609043);

        /* If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash
        starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970`
        looks like `000006136ef...`. */
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
