//! # [Day 7: Some Assembly Required](https://adventofcode.com/2015/day/7)
//!
//! This year, Santa brought little Bobby Tables a set of wires and
//! [bitwise logic gates](https://en.wikipedia.org/wiki/Bitwise_operation)! Unfortunately, little Bobby
//! is a little under the recommended age range, and he needs help assembling the circuit.
//!
//! Each wire has an identifier (some lowercase letters) and can carry a
//! [16-bit](https://en.wikipedia.org/wiki/16-bit) signal (a number from `0` to `65535`). A signal is
//! provided to each wire by a gate, another wire, or some specific value. Each wire can only get a
//! signal from one source, but can provide its signal to multiple destinations. A gate provides no
//! signal until all of its inputs have a signal.
//!
//! The included instructions booklet describes how to connect the parts together: `x AND y -> z` means
//! to connect wires `x` and `y` to an AND gate, and then connect its output to wire `z`.
//!
//! For example:
//!
//! -   `123 -> x` means that the signal `123` is provided to wire `x`.
//! -   `x AND y -> z` means that the [bitwise AND](https://en.wikipedia.org/wiki/Bitwise_operation#AND)
//!     of wire `x` and wire `y` is provided to wire `z`.
//! -   `p LSHIFT 2 -> q` means that the value from wire `p` is
//!     [left-shifted](https://en.wikipedia.org/wiki/Logical_shift) by `2` and then provided to
//!     wire `q`.
//! -   `NOT e -> f` means that the
//!     [bitwise complement](https://en.wikipedia.org/wiki/Bitwise_operation#NOT) of the value from
//!     wire `e` is provided to wire `f`.
//!
//! Other possible gates include `OR` ([bitwise OR](https://en.wikipedia.org/wiki/Bitwise_operation#OR))
//! and `RSHIFT` ([right-shift](https://en.wikipedia.org/wiki/Logical_shift)). If, for some reason,
//! you'd like to emulate the circuit instead, almost all programming languages (for
//! example, [C](https://en.wikipedia.org/wiki/Bitwise_operations_in_C),
//! [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_Operators),
//! or [Python](https://wiki.python.org/moin/BitwiseOperators)) provide operators for these gates.
//!
//! For example, here is a simple circuit:
//!
//!
//! ```plain
//! 123 -> x  
//! 456 -> y  
//! x AND y -> d  
//! x OR y -> e  
//! x LSHIFT 2 -> f  
//! y RSHIFT 2 -> g  
//! NOT x -> h
//! NOT y -> i
//! ```
//!
//! After it is run, these are the signals on the wires:
//!
//! ```plain
//! d: 72
//! e: 507
//! f: 492
//! g: 114
//! h: 65412
//! i: 65079
//! x: 123
//! y: 456
//! ```
//!
//! In little Bobby's kit's instructions booklet (provided as your puzzle input),
//! **what signal is ultimately provided to wire `a`?**
//!
//! # Part Two
//! Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires
//! (including wire a).
//!
//! **What new signal is ultimately provided to wire a?**

/// Part 1
#[aoc(day7, part1)]
pub fn part1(input: &str) -> u16 {
    let gates = parse_gates(input);
    let mut cache = HashMap::new();
    eval_wire("a", &gates, &mut cache)
}

/// Part 2
#[aoc(day7, part2)]
pub fn part2(input: &str) -> u16 {
    let mut gates = parse_gates(input);
    let a = eval_wire("a", &gates, &mut HashMap::new());
    gates.insert("b".into(), Gate::Set(Expr::Value(a)));
    eval_wire("a", &gates, &mut HashMap::new())
}

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Expr {
    Wire(String),
    Value(u16),
}

enum Gate {
    Set(Expr),
    BinaryNot(Expr),
    BinaryAnd(Expr, Expr),
    BinaryOr(Expr, Expr),
    LeftShift(Expr, Expr),
    RightShift(Expr, Expr),
}

fn parse_expr(input: &str) -> nom::IResult<&str, Expr, ()> {
    match alpha1::<&str, ()>(input) {
        Ok((tail, alpha)) => Ok((tail, Expr::Wire(alpha.into()))),
        _ => {
            let (tail, digit) = digit1(input)?;
            Ok((tail, Expr::Value(digit.parse().unwrap())))
        }
    }
}

fn parse_gate(line: &str) -> nom::IResult<&str, (Gate, String), ()> {
    let parse_assign = tag::<&str, &str, ()>(" -> ");
    Ok(match tag::<&str, &str, ()>("NOT ")(line) {
        Ok((tail, _)) => {
            let (tail, rov) = parse_expr(tail)?;
            let (tail, _) = parse_assign(tail)?;
            (tail, (Gate::BinaryNot(rov), tail.into()))
        }
        _ => {
            let (tail, left) = parse_expr(line)?;

            match parse_assign(tail) {
                Ok((tail, _)) => (tail, (Gate::Set(left), tail.into())),
                _ => match tag::<&str, &str, ()>(" AND ")(tail) {
                    Ok((tail, _)) => {
                        let (tail, right) = parse_expr(tail)?;
                        let (tail, _) = parse_assign(tail)?;
                        (tail, (Gate::BinaryAnd(left, right), tail.into()))
                    }
                    _ => match tag::<&str, &str, ()>(" OR ")(tail) {
                        Ok((tail, _)) => {
                            let (tail, right) = parse_expr(tail)?;
                            let (tail, _) = parse_assign(tail)?;
                            (tail, (Gate::BinaryOr(left, right), tail.into()))
                        }
                        _ => match tag::<&str, &str, ()>(" LSHIFT ")(tail) {
                            Ok((tail, _)) => {
                                let (tail, right) = parse_expr(tail)?;
                                let (tail, _) = parse_assign(tail)?;
                                (tail, (Gate::LeftShift(left, right), tail.into()))
                            }
                            _ => {
                                let (tail, _) = tag::<&str, &str, ()>(" RSHIFT ")(tail)?;
                                let (tail, right) = parse_expr(tail)?;
                                let (tail, _) = parse_assign(tail)?;
                                (tail, (Gate::RightShift(left, right), tail.into()))
                            }
                        },
                    },
                },
            }
        }
    })
}

fn eval_wire(wire: &str, gates: &HashMap<String, Gate>, cache: &mut HashMap<String, u16>) -> u16 {
    if cache.contains_key(wire) {
        return cache[wire];
    }
    let mut resolve = |expr: &Expr| -> u16 {
        match expr {
            Expr::Wire(name) => eval_wire(&name, gates, cache),
            Expr::Value(value) => *value,
        }
    };
    let result = match &gates[wire] {
        Gate::Set(value) => resolve(value),
        Gate::BinaryAnd(left, right) => resolve(left) & resolve(right),
        Gate::BinaryOr(left, right) => resolve(left) | resolve(right),
        Gate::BinaryNot(value) => !resolve(value),
        Gate::LeftShift(value, amount) => resolve(value) << resolve(amount),
        Gate::RightShift(value, amount) => resolve(value) >> resolve(amount),
    };
    cache.insert(wire.into(), result);
    result
}

fn parse_gates(input: &str) -> HashMap<String, Gate> {
    let mut gates: HashMap<String, Gate> = HashMap::new();
    for line in input.lines() {
        let (_, (gate, key)) = parse_gate(line).unwrap();
        gates.insert(key, gate);
    }
    gates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // For example, here is a simple circuit:
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let gates = parse_gates(input);
        let mut cache = HashMap::new();
        // After it is run, these are the signals on the wires:
        assert_eq!(eval_wire("d", &gates, &mut cache), 72);
        assert_eq!(eval_wire("e", &gates, &mut cache), 507);
        assert_eq!(eval_wire("f", &gates, &mut cache), 492);
        assert_eq!(eval_wire("g", &gates, &mut cache), 114);
        assert_eq!(eval_wire("h", &gates, &mut cache), 65412);
        assert_eq!(eval_wire("i", &gates, &mut cache), 65079);
        assert_eq!(eval_wire("x", &gates, &mut cache), 123);
        assert_eq!(eval_wire("y", &gates, &mut cache), 456);
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(parse_expr("foo!").unwrap(), ("!", Expr::Wire("foo".into())));
        assert_eq!(parse_expr("123!").unwrap(), ("!", Expr::Value(123)));
    }
}
