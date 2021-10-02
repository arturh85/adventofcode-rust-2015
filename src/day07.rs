use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use std::collections::HashMap;

/**
# [Day 7: Some Assembly Required](https://adventofcode.com/2015/day/7)

This year, Santa brought little Bobby Tables a set of wires and
[bitwise logic gates](https://en.wikipedia.org/wiki/Bitwise_operation)! Unfortunately, little Bobby
is a little under the recommended age range, and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a
[16-bit](https://en.wikipedia.org/wiki/16-bit) signal (a number from `0` to `65535`). A signal is
provided to each wire by a gate, another wire, or some specific value. Each wire can only get a
signal from one source, but can provide its signal to multiple destinations. A gate provides no
signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: `x AND y -> z` means
to connect wires `x` and `y` to an AND gate, and then connect its output to wire `z`.

For example:

-   `123 -> x` means that the signal `123` is provided to wire `x`.
-   `x AND y -> z` means that the [bitwise AND](https://en.wikipedia.org/wiki/Bitwise_operation#AND)
    of wire `x` and wire `y` is provided to wire `z`.
-   `p LSHIFT 2 -> q` means that the value from wire `p` is
    [left-shifted](https://en.wikipedia.org/wiki/Logical_shift) by `2` and then provided to
    wire `q`.
-   `NOT e -> f` means that the
    [bitwise complement](https://en.wikipedia.org/wiki/Bitwise_operation#NOT) of the value from
    wire `e` is provided to wire `f`.

Other possible gates include `OR` ([bitwise OR](https://en.wikipedia.org/wiki/Bitwise_operation#OR))
and `RSHIFT` ([right-shift](https://en.wikipedia.org/wiki/Logical_shift)). If, for some reason,
you'd like to emulate the circuit instead, almost all programming languages (for
example, [C](https://en.wikipedia.org/wiki/Bitwise_operations_in_C),
[JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_Operators),
or [Python](https://wiki.python.org/moin/BitwiseOperators)) provide operators for these gates.

For example, here is a simple circuit:


123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i

After it is run, these are the signals on the wires:

d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456

In little Bobby's kit's instructions booklet (provided as your puzzle input),
**what signal is ultimately provided to wire `a`?**
*/

#[derive(PartialEq, Debug)]
enum RegisterOrValue {
    Register(String),
    Value(u16),
}

enum Operation {
    Set(RegisterOrValue, String),
    BinaryAnd(RegisterOrValue, RegisterOrValue, String),
    BinaryOr(RegisterOrValue, RegisterOrValue, String),
    BinaryNot(RegisterOrValue, String),
    LeftShift(RegisterOrValue, RegisterOrValue, String),
    RightShift(RegisterOrValue, RegisterOrValue, String),
}

fn parse_register_or_value(input: &str) -> nom::IResult<&str, RegisterOrValue, ()> {
    match alpha1::<&str, ()>(input) {
        Ok((tail, alpha)) => Ok((tail, RegisterOrValue::Register(alpha.into()))),
        _ => {
            let (tail, digit) = digit1(input)?;
            Ok((tail, RegisterOrValue::Value(digit.parse().unwrap())))
        }
    }
}

fn parse_line(line: &str) -> nom::IResult<&str, Operation, ()> {
    let assign = tag::<&str, &str, ()>(" -> ");
    Ok(match tag::<&str, &str, ()>("NOT ")(line) {
        Ok((tail, _)) => {
            let (tail, rov) = parse_register_or_value(tail)?;
            let (tail, _) = assign(tail)?;
            (tail, Operation::BinaryNot(rov, tail.into()))
        }
        _ => {
            let (tail, left) = parse_register_or_value(line)?;

            match assign(tail) {
                Ok((tail, _)) => (tail, Operation::Set(left, tail.into())),
                _ => match tag::<&str, &str, ()>(" AND ")(tail) {
                    Ok((tail, _)) => {
                        let (tail, right) = parse_register_or_value(tail)?;
                        let (tail, _) = assign(tail)?;
                        (tail, Operation::BinaryAnd(left, right, tail.into()))
                    }
                    _ => match tag::<&str, &str, ()>(" OR ")(tail) {
                        Ok((tail, _)) => {
                            let (tail, right) = parse_register_or_value(tail)?;
                            let (tail, _) = assign(tail)?;
                            (tail, Operation::BinaryOr(left, right, tail.into()))
                        }
                        _ => match tag::<&str, &str, ()>(" LSHIFT ")(tail) {
                            Ok((tail, _)) => {
                                let (tail, right) = parse_register_or_value(tail)?;
                                let (tail, _) = assign(tail)?;
                                (tail, Operation::LeftShift(left, right, tail.into()))
                            }
                            _ => {
                                let (tail, _) = tag::<&str, &str, ()>(" RSHIFT ")(tail)?;
                                let (tail, right) = parse_register_or_value(tail)?;
                                let (tail, _) = assign(tail)?;
                                (tail, Operation::RightShift(left, right, tail.into()))
                            }
                        },
                    },
                },
            }
        }
    })
}

fn apply_line(line: &str, data: &HashMap<String, u16>) -> (String, u16) {
    let resolve = |rov: RegisterOrValue| -> u16 {
        match rov {
            RegisterOrValue::Register(name) => *data.get(&name).unwrap_or(&0),
            RegisterOrValue::Value(value) => value,
        }
    };
    let (_, operation) = parse_line(line).expect("failed to parse line");

    match operation {
        Operation::Set(value, target) => (target, resolve(value)),
        Operation::BinaryAnd(left, right, target) => (target, resolve(left) & resolve(right)),
        Operation::BinaryOr(left, right, target) => (target, resolve(left) | resolve(right)),
        Operation::BinaryNot(value, target) => (target, !resolve(value)),
        Operation::LeftShift(value, amount, target) => (target, resolve(value) << resolve(amount)),
        Operation::RightShift(value, amount, target) => (target, resolve(value) >> resolve(amount)),
    }
}

fn run(input: &str, mut data: HashMap<String, u16>) -> HashMap<String, u16> {
    for line in input.lines() {
        let (key, value) = apply_line(line, &data);
        data.insert(key, value);
    }
    data
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u16 {
    let results = run(input, HashMap::new());
    // println!("{:?}", results);
    *results.get("a").unwrap()
}

/**
# --- Part Two ---
Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires
(including wire a).

**What new signal is ultimately provided to wire a?**
*/
#[aoc(day7, part2)]
fn part2(input: &str) -> u16 {
    let mut results = run(input, HashMap::new());
    results.insert("b".into(), results["a"]);
    let results = run(input, results);
    // println!("{:?}", results);
    *results.get("a").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pov() {
        assert_eq!(
            parse_register_or_value("foo!").unwrap(),
            ("!", RegisterOrValue::Register("foo".into()))
        );
        assert_eq!(
            parse_register_or_value("123!").unwrap(),
            ("!", RegisterOrValue::Value(123))
        );
    }

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
        let result = run(input, HashMap::new());
        // println!("{:?}", result);
        // After it is run, these are the signals on the wires:
        assert_eq!(result["d"], 72);
        assert_eq!(result["e"], 507);
        assert_eq!(result["f"], 492);
        assert_eq!(result["g"], 114);
        assert_eq!(result["h"], 65412);
        assert_eq!(result["i"], 65079);
        assert_eq!(result["x"], 123);
        assert_eq!(result["y"], 456);
    }
}
