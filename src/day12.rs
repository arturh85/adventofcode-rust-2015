//! # [Day 12: JSAbacusFramework.io](https://adventofcode.com/2015/day/12)
//!
//! Santa's Accounting-Elves need help balancing the books after a recent order.
//! Unfortunately, their accounting software uses a peculiar storage format.
//! That's where you come in.
//!
//! They have a [JSON](http://json.org/) document which contains a variety of things:
//! arrays (`[1,2,3]`), objects (`{"a":1, "b":2}`), numbers, and strings.
//! Your first job is to simply find all of the numbers throughout the document
//! and add them together.
//!
//! For example:
//!
//! - `[1,2,3]` and `{"a":2,"b":4}` both have a sum of `6`.
//! - `[[[3]]]` and `{"a":{"b":4},"c":-1}` both have a sum of `3`.
//! - `{"a":[-1,1]}` and `[-1,{"a":1}]` both have a sum of `0`.
//! - `[]` and `{}` both have a sum of `0`.
//!
//! You will not encounter any strings containing numbers.
//!
//! **What is the sum of all numbers in the document?**
//!
//! # Part Two
//!
//! Uh oh - the Accounting-Elves have realized that they double-counted everything red.
//!
//! **Ignore any object (and all of its children) which has any property with the value `"red"`.
//! Do this only for objects (`{...}`), not arrays (`[...]`).**
//!
//! - `[1,2,3]` still has a sum of `6`.
//! - `[1,{"c":"red","b":2},3]` now has a sum of `4`, because the middle object is ignored.
//! - `{"d":"red","e":[1,2,3,4],"f":5}` now has a sum of `0`, because the entire structure is ignored.
//! - `[1,"red",5]` has a sum of `6`, because `"red"` in an array has no effect.

use serde_json::Value;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> anyhow::Result<Value> {
    Ok(serde_json::from_str(input)?)
}

/// Part 1: What is the sum of all numbers in the document?
#[aoc(day12, part1)]
fn part1(input: &Value) -> i64 {
    count(input)
}

/// Part 2: Ignore any object (and all of its children) which has any property with the value `"red"`.
/// Do this only for objects (`{...}`), not arrays (`[...]`).
#[aoc(day12, part2)]
fn part2(input: &Value) -> i64 {
    count_no_red(input)
}

fn count(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(count).sum(),
        Value::Object(object) => object.values().map(count).sum(),
        _ => 0,
    }
}

fn count_no_red(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(count_no_red).sum(),
        Value::Object(object) => {
            let mut sum = 0;
            for value in object.values() {
                if let Value::String(value) = value {
                    if value == "red" {
                        return 0;
                    }
                }
                sum += count_no_red(value)
            }
            sum
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        // `[1,2,3]` and `{"a":2,"b":4}` both have a sum of `6`.
        assert_eq!(count(&json!([1, 2, 3])), 6);
        assert_eq!(count(&json!( {"a":2,"b":4})), 6);

        // `[[[3]]]` and `{"a":{"b":4},"c":-1}` both have a sum of `3`.
        assert_eq!(count(&json!([[[3]]])), 3);
        assert_eq!(count(&json!({"a":{"b":4},"c":-1})), 3);

        // `{"a":[-1,1]}` and `[-1,{"a":1}]` both have a sum of `0`.
        assert_eq!(count(&json!( {"a":[-1,1]})), 0);
        assert_eq!(count(&json!( [-1,{"a":1}])), 0);

        // `[]` and `{}` both have a sum of `0`.
        assert_eq!(count(&json!([])), 0);
        assert_eq!(count(&json!({})), 0);
    }

    #[test]
    fn part2_examples() {
        // `[1,2,3]` still has a sum of `6`.
        assert_eq!(count_no_red(&json!([1, 2, 3])), 6);
        // `[1,{"c":"red","b":2},3]` now has a sum of `4`, because the middle object is ignored.
        assert_eq!(count_no_red(&json!([1,{"c":"red","b":2},3])), 4);
        // `{"d":"red","e":[1,2,3,4],"f":5}` now has a sum of `0`, because the entire structure is ignored.
        assert_eq!(count_no_red(&json!({"d":"red","e":[1,2,3,4],"f":5})), 0);
        // `[1,"red",5]` has a sum of `6`, because `"red"` in an array has no effect.
        assert_eq!(count_no_red(&json!([1, "red", 5])), 6);
    }
}
