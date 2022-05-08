//! # [Day 23: Opening the Turing Lock](https://adventofcode.com/2015/day/23)
//!
//! Little Jane Marie just got her very first computer for Christmas from some unknown benefactor.
//! It comes with instructions and an example program, but the computer itself seems to be
//! malfunctioning. She's curious what the program does, and would like you to help her run it.
//!
//! The manual explains that the computer supports two
//! [registers](https://en.wikipedia.org/wiki/Processor_register) and six
//! [instructions](https://en.wikipedia.org/wiki/Instruction_set) (truly, it goes on to remind the
//! reader, a state-of-the-art technology). The registers are named `a` and `b`, can hold any
//! [non-negative integer](https://en.wikipedia.org/wiki/Natural_number), and begin with a value of
//! `0`. The instructions are as follows:
//!
//! - `hlf r` sets register `r` to half its current value, then continues with the next instruction.
//! - `tpl r` sets register `r` to triple its current value, then continues with the next
//!     instruction.
//! - `inc r` increments register `r`, adding `1` to it, then continues with the next instruction.
//! - `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
//! - `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
//! - `jio r, offset` is like `jmp`, but only jumps if register `r` is `1` ("jump if one", not odd).
//!
//! All three jump instructions work with an offset relative to that instruction.
//! The offset is always written with a prefix `+` or `-` to indicate the direction of the jump
//! (forward or backward, respectively). For example, `jmp +1` would simply continue with the next
//! instruction, while `jmp +0` would continuously jump back to itself forever.
//!
//! The program exits when it tries to run an instruction beyond the ones defined.
//!
//! For example, this program sets `a` to `2`, because the `jio` instruction causes it to
//! skip the `tpl` instruction:
//!
//! ```plain
//! inc a
//! jio a, +2
//! tpl a
//! inc a
//! ```
//!
//! **What is the value in register `b` when the program in your puzzle input is finished executing?**
//!
//! # Part Two
//!
//! The unknown benefactor is very thankful for releasi-- er, helping little Jane Marie with her
//! computer. Definitely not to distract you, **what is the value in register b after the program is
//! finished executing if register a starts as 1 instead?**

use std::collections::HashMap;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let command = &line[0..3];
            let args: Vec<&str> = line[4..line.len()].split(", ").collect();

            match command {
                "hlf" => Instruction::Hlf(args[0].to_owned()),
                "tpl" => Instruction::Tpl(args[0].to_owned()),
                "inc" => Instruction::Inc(args[0].to_owned()),
                "jmp" => Instruction::Jmp(args[0].parse().unwrap()),
                "jie" => Instruction::Jie(args[0].to_owned(), args[1].parse().unwrap()),
                "jio" => Instruction::Jio(args[0].to_owned(), args[1].parse().unwrap()),
                _ => {
                    panic!("unknown instruction: {}", command)
                }
            }
        })
        .collect()
}

enum Instruction {
    Hlf(String),
    Tpl(String),
    Inc(String),
    Jmp(i64),
    Jie(String, i64),
    Jio(String, i64),
}

/// Part 1: What is the value in register `b` when the program in your puzzle input is finished executing?
#[aoc(day23, part1)]
fn part1(input: &[Instruction]) -> i64 {
    let registers = exec(input, 0);
    registers["b"]
}

fn exec(code: &[Instruction], initial_a: i64) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    registers.insert("a".to_owned(), initial_a);
    registers.insert("b".to_owned(), 0);

    let mut pointer: isize = 0;

    loop {
        if pointer < 0 || pointer >= code.len() as isize {
            break;
        }
        let instr = &code[pointer as usize];
        match instr {
            // `hlf r` sets register `r` to half its current value,
            // then continues with the next instruction.
            Instruction::Hlf(r) => {
                if let Some(r) = registers.get_mut(r) {
                    *r /= 2;
                }
                pointer += 1;
            }
            // `tpl r` sets register `r` to triple its current value,
            // then continues with the next instruction.
            Instruction::Tpl(r) => {
                if let Some(r) = registers.get_mut(r) {
                    *r *= 3;
                }
                pointer += 1;
            }
            // `inc r` increments register `r`, adding `1` to it,
            // then continues with the next instruction.
            Instruction::Inc(r) => {
                if let Some(r) = registers.get_mut(r) {
                    *r += 1;
                }
                pointer += 1;
            }
            // `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
            Instruction::Jmp(offset) => {
                pointer += *offset as isize;
            }
            // `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
            Instruction::Jie(r, offset) => {
                if let Some(r) = registers.get_mut(r) {
                    if *r % 2 == 0 {
                        pointer += *offset as isize;
                    } else {
                        pointer += 1;
                    }
                }
            }
            // `jio r, offset` is like `jmp`, but only jumps if register `r` is `1` ("jump if one", not odd).
            Instruction::Jio(r, offset) => {
                if let Some(r) = registers.get_mut(r) {
                    if *r == 1 {
                        pointer += *offset as isize;
                    } else {
                        pointer += 1;
                    }
                }
            }
        }
    }

    registers
}

/// Part 2: what is the value in register b after the program is finished executing if
/// register a starts as 1 instead?
#[aoc(day23, part2)]
fn part2(input: &[Instruction]) -> i64 {
    let registers = exec(input, 1);
    registers["b"]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn part1_examples() {
        // For example, this program sets `a` to `2`, because the `jio` instruction causes it to
        // skip the `tpl` instructionlet registers = part1(&parse_input(EXAMPLE));
        let registers = exec(&parse_input(EXAMPLE), 0);
        assert_eq!(2, registers["a"]);
    }
}
