use std::ops::Add;

/**
# [Day 6: Probably a Fire Hazard](https://adventofcode.com/2015/day/6)
Because your neighbors keep defeating you in the holiday house decorating contest year after year,
you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions
on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner
are at `0,0`, `0,999`, `999,999`, and `999,0`. The instructions include whether to `turn on`,
`turn off`, or `toggle` various inclusive ranges given as coordinate pairs. Each coordinate pair
represents opposite corners of a rectangle, inclusive; a coordinate pair like `0,0 through 2,2`
therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the
instructions Santa sent you in order.

For example:

-   `turn on 0,0 through 999,999` would turn on (or leave on) every light.
-   `toggle 0,0 through 999,0` would toggle the first line of 1000 lights, turning off the ones
    that were on, and turning on the ones that were off.
-   `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.

**After following the instructions, how many lights are lit?**
*/

// Solution Source: https://www.reddit.com/r/adventofcode/comments/3vmltn/day_6_solutions/cxptu4a/

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    count(input, turn_on1, turn_off1, toggle1)
}
struct Grid<T> {
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn operation<F: Fn(&mut T)>(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, f: F) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                f(&mut self.data[i + j * 1000]);
            }
        }
    }
}

trait GridCount {
    fn count(&self) -> usize;
}

impl GridCount for Grid<bool> {
    fn count(&self) -> usize {
        self.data.iter().filter(|&s| *s).count()
    }
}

impl GridCount for Grid<u32> {
    fn count(&self) -> usize {
        self.data.iter().fold(0u32, Add::add) as usize
    }
}

fn count<T>(input: &str, on: fn(&mut T), off: fn(&mut T), toggle: fn(&mut T)) -> usize
where
    Grid<T>: GridCount,
    T: Default,
    T: Copy,
{
    let ref mut grid = Grid {
        data: vec![Default::default(); 1_000_000],
    };

    for ref mut line in input.lines() {
        let instr: fn(&mut T) = if eat(line, "turn on ") {
            on
        } else if eat(line, "turn off ") {
            off
        } else if eat(line, "toggle ") {
            toggle
        } else {
            panic!("Invalid instruction: '{}'", line)
        };

        let coords = line
            .split(',')
            .flat_map(|s| s.split(" through "))
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        grid.operation(coords[0], coords[1], coords[2], coords[3], instr);
    }

    grid.count()
}

fn turn_on1(b: &mut bool) {
    *b = true
}
fn turn_off1(b: &mut bool) {
    *b = false
}
fn toggle1(b: &mut bool) {
    *b = !*b
}

fn eat(s: &mut &str, expect: &str) -> bool {
    if s.starts_with(expect) {
        *s = &s[expect.len()..];
        true
    } else {
        false
    }
}

/**
# Part Two

You just finish implementing your winning light pattern when you realize you mistranslated
Santa's message from Ancient Nordic Elvish.

The light grid you bought actually has individual brightness controls; each light can have a
brightness of zero or more. The lights all start at zero.

The phrase `turn on` actually means that you should increase the brightness of those lights by `1`.

The phrase `turn off` actually means that you should decrease the brightness of those lights by `1`,
to a minimum of zero.

The phrase `toggle` actually means that you should increase the brightness of those lights by `2`.

**What is the total brightness of all lights combined after following Santa's instructions?**

For example:

-   `turn on 0,0 through 0,0` would increase the total brightness by `1`.
-   `toggle 0,0 through 999,999` would increase the total brightness by `2000000`.
*/

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    count(input, turn_on2, turn_off2, toggle2)
}

fn turn_on2(i: &mut u32) {
    *i += 1
}
fn turn_off2(i: &mut u32) {
    if *i > 0 {
        *i -= 1
    }
}
fn toggle2(i: &mut u32) {
    *i += 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        // `turn on 0,0 through 999,999` would turn on (or leave on) every light.
        assert_eq!(part1("turn on 0,0 through 999,999"), 1_000_000);

        // `toggle 0,0 through 999,0` would toggle the first line of 1000 lights, turning off the ones
        // that were on, and turning on the ones that were off.
        assert_eq!(part1("toggle 0,0 through 999,0"), 1_000);

        // `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
        assert_eq!(part1("turn off 499,499 through 500,500"), 0);
    }

    #[test]
    fn part2_examples() {
        // `turn on 0,0 through 0,0` would increase the total brightness by `1`.
        assert_eq!(part2("turn on 0,0 through 0,0"), 1);

        // `toggle 0,0 through 999,999` would increase the total brightness by `2000000`.
        assert_eq!(part2("toggle 0,0 through 999,999"), 2_000_000);
    }
}
