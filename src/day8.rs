//! # [Day 8: Matchsticks](https://adventofcode.com/2015/day/8)
//!
//! Space on the sleigh is limited this year, and so Santa will be bringing his list as a digital copy.
//! He needs to know how much space it will take up when stored.
//!
//! It is common in many programming languages to provide a way to escape special characters in
//! strings. For example, [C](https://en.wikipedia.org/wiki/Escape_sequences_in_C),
//! [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String),
//! [Perl](http://perldoc.perl.org/perlop.html#Quote-and-Quote-like-Operators),
//! [Python](https://docs.python.org/2.0/ref/strings.html), and even
//! [PHP](http://php.net/manual/en/language.types.string.php#language.types.string.syntax.double)
//! handle special characters in very similar ways.
//!
//! However, it is important to realize the difference between the number of characters in the code
//! representation of the string literal and the number of characters in the in-memory string itself.
//!
//! For example:
//!
//! -   `""` is `2` characters of code (the two double quotes), but the string contains zero characters.
//! -   `"abc"` is `5` characters of code, but `3` characters in the string data.
//! -   `"aaa\\"aaa"` is `10` characters of code, but the string itself contains six "a" characters and
//!     a single, escaped quote character, for a total of `7` characters in the string data.
//! -   `"\x27"` is `6` characters of code, but the string itself contains just one - an
//!     apostrophe (`'`), escaped using hexadecimal notation.
//!
//! Santa's list is a file that contains many double-quoted string literals, one on each line.
//! The only escape sequences used are `\\` (which represents a single backslash), `\"` (which
//! represents a lone double-quote character), and `\x` plus two hexadecimal characters (which
//! represents a single character with that ASCII code).
//!
//! Disregarding the whitespace in the file, **what is the number of characters of code for string
//! literals minus the number of characters in memory for the values of the strings in total for the
//! entire file?**
//!
//! For example, given the four strings above, the total number of characters of string
//! code (`2 + 5 + 10 + 6 = 23`) minus the total number of characters in memory for string
//! values (`0 + 3 + 7 + 1 = 11`) is `23 - 11 = 12`.
//!
//! # Part Two
//!
//! Now, let's go the other way. In addition to finding the number of characters of code,
//! you should now encode each code representation as a new string and find the number of characters
//! of the new encoded representation, including the surrounding double quotes.
//!
//! For example:
//!
//! -   `""` encodes to `"\"\""`, an increase from `2` characters to `6`.
//! -   `"abc"` encodes to `"\"abc\""`, an increase from `5` characters to `9`.
//! -   `"aaa\"aaa"` encodes to `"\"aaa\\\"aaa\""`, an increase from `10` characters to `16`.
//! -   `"\x27"` encodes to `"\"\\x27\""`, an increase from `6` characters to `11`.
//!
//! **Your task is to find the total number of characters to represent the newly encoded strings
//! minus the number of characters of code in each original string literal.** For example, for the
//! strings above, the total encoded length (`6 + 9 + 16 + 11 = 42`) minus the characters in the
//! original code representation (`23`, just like in the first part of this puzzle)
//! is `42 - 23 = 19`.

use nom::branch::alt;

/// Part 1: what is the number of characters of code for string literals minus the number of characters in
/// memory for the values of the strings in total for the entire file?
#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let mut total_string = 0;
    let mut total_code = 0;

    for line in input.lines() {
        let (string, code) = count(line);
        total_string += string;
        total_code += code;
    }

    total_string - total_code
}

/// Part 2: find the total number of characters to represent the newly encoded strings
/// minus the number of characters of code in each original string literal
#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let mut total_encoded = 0;
    let mut total_code = 0;

    for line in input.lines() {
        let encoded = encode(line);
        let (string, _) = count(line);
        total_encoded += encoded.len();
        total_code += string;
    }

    total_encoded - total_code
}

fn _hex_to_char(s: &str) -> Result<char, std::num::ParseIntError> {
    u8::from_str_radix(s, 16).map(|n| n as char)
}

fn _parse_hex(i: &str) -> nom::IResult<&str, String, ()> {
    Ok((
        "",
        String::from(u8::from_str_radix(i, 16).map(|n| n as char).unwrap()),
    ))
}

fn escaped_backslash(i: &str) -> nom::IResult<&str, &str, ()> {
    nom::combinator::recognize(nom::character::complete::char('\\'))(i)
}
fn escaped_quote(i: &str) -> nom::IResult<&str, &str, ()> {
    nom::combinator::value("\"", nom::bytes::complete::tag("\""))(i)
}
fn escaped_hex(i: &str) -> nom::IResult<&str, &str, ()> {
    let (tail, _) = nom::bytes::complete::tag("x")(i)?;
    let tail = &tail[2..tail.len()];
    Ok((tail, ".")) // correct char doesn't matter so we just use a dot for everything
}

fn parse(input: &str) -> nom::IResult<&str, String, ()> {
    let (tail, content) = nom::bytes::complete::escaped_transform(
        nom::bytes::complete::is_not("\\"),
        '\\',
        alt((escaped_backslash, escaped_quote, escaped_hex)),
    )(&input[1..input.len() - 1])?;
    Ok((tail, content))
}

fn count(input: &str) -> (usize, usize) {
    let (_, content) = parse(input).unwrap();
    (input.len(), content.len())
}

fn encode(i: &str) -> String {
    let mut s = i.to_string();
    s = s.replace("\\", "\\\\");
    s = "\"".to_string() + &*s.replace("\"", "\\\"") + "\"";
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        // `""` is `2` characters of code (the two double quotes), but the string contains zero characters.
        assert_eq!(count("\"\""), (2, 0));

        // `"abc"` is `5` characters of code, but `3` characters in the string data.
        assert_eq!(count("\"abc\""), (5, 3));

        // `"aaa\\"aaa"` is `10` characters of code, but the string itself contains six "a" characters and
        // a single, escaped quote character, for a total of `7` characters in the string data.
        assert_eq!(count("\"aaa\\\"aaa\""), (10, 7));

        // `"\x27"` is `6` characters of code, but the string itself contains just one - an
        // apostrophe (`'`), escaped using hexadecimal notation.
        assert_eq!(count("\"\\x27\""), (6, 1));
    }

    #[test]
    fn part2_examples() {
        // `""` encodes to `"\"\""`, an increase from `2` characters to `6`.
        assert_eq!(encode("\"\""), "\"\\\"\\\"\"");
        assert_eq!(encode("\"\"").len(), 6);
        // `"abc"` encodes to `"\"abc\""`, an increase from `5` characters to `9`.
        assert_eq!(encode("\"abc\""), "\"\\\"abc\\\"\"");
        assert_eq!(encode("\"abc\"").len(), 9);
        // // `"aaa\"aaa"` encodes to `"\"aaa\\\"aaa\""`, an increase from `10` characters to `16`.
        assert_eq!(encode("\"aaa\\\"aaa\""), "\"\\\"aaa\\\\\\\"aaa\\\"\"");
        assert_eq!(encode("\"aaa\\\"aaa\"").len(), 16);
        // // `"\x27"` encodes to `"\"\\x27\""`, an increase from `6` characters to `11`.
        assert_eq!(encode("\"\\x27\""), "\"\\\"\\\\x27\\\"\"");
        assert_eq!(encode("\"\\x27\"").len(), 11);
    }
}
