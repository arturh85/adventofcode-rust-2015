//! # [Day 18: Like a GIF For Your Yard](https://adventofcode.com/2015/day/18)
//!
//! After the [million lights incident](https://adventofcode.com/2015/day/6), the fire code has
//! gotten stricter: now, at most ten thousand lights are allowed.
//! You arrange them in a 100x100 grid.
//!
//! Never one to let you down, Santa again mails you instructions on the ideal lighting
//! configuration. With so few lights, he says, you'll have to resort to animation.
//!
//! Start by setting your lights to the included initial configuration (your puzzle input).
//! A `#` means "on", and a `.` means "off".
//!
//! Then, animate your grid in steps, where each step decides the next configuration based on
//! the current one. Each light's next state (either on or off) depends on its current state
//! and the current states of the eight lights adjacent to it (including diagonals).
//! Lights on the edge of the grid might have fewer than eight neighbors;
//! the missing ones always count as "off".
//!
//! For example, in a simplified 6x6 grid, the light marked `A` has the neighbors numbered `1`
//! through `8`, and the light marked `B`, which is on an edge, only has the neighbors marked `1`
//! through `5`:
//!
//! ```plain
//! 1B5...
//! 234...
//! ......
//! ..123.
//! ..8A4.
//! ..765.
//!
//! ```
//!
//! The state a light should have next is based on its current state (on or off) plus
//! the number of neighbors that are on:
//!
//! - A light which is on stays on when `2` or `3` neighbors are on, and turns off otherwise.
//! - A light which is off turns on if exactly `3` neighbors are on, and stays off otherwise.
//!
//! All of the lights update simultaneously; they all consider the same current state
//! before moving to the next.
//!
//! Here's a few steps from an example configuration of another 6x6 grid:
//!
//! ```plain
//! Initial state:
//! .#.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####..
//!
//! After 1 step:
//! ..##..
//! ..##.#
//! ...##.
//! ......
//! #.....
//! #.##..
//!
//! After 2 steps:
//! ..###.
//! ......
//! ..###.
//! ......
//! .#....
//! .#....
//!
//! After 3 steps:
//! ...#..
//! ......
//! ...#..
//! ..##..
//! ......
//! ......
//!
//! After 4 steps:
//! ......
//! ......
//! ..##..
//! ..##..
//! ......
//! ......
//!
//! ```
//!
//! After `4` steps, this example has four lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration,
//! how many lights are on after 100 steps?
//!
//! # Part Two
//!
//! You flip the instructions over; Santa goes on to point out that this is all just an
//! implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life).
//! At least, it was, until you notice that something's wrong with the grid of lights you bought:
//! four lights, one in each corner, are stuck on and can't be turned off.
//!
//! The example above will actually run like this:
//!
//! ```plain
//! Initial state:
//! ##.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####.#
//!
//! After 1 step:
//! #.##.#
//! ####.#
//! ...##.
//! ......
//! #...#.
//! #.####
//!
//! After 2 steps:
//! #..#.#
//! #....#
//! .#.##.
//! ...##.
//! .#..##
//! ##.###
//!
//! After 3 steps:
//! #...##
//! ####.#
//! ..##.#
//! ......
//! ##....
//! ####.#
//!
//! After 4 steps:
//! #.####
//! #....#
//! ...#..
//! .##...
//! #.....
//! #.#..#
//!
//! After 5 steps:
//! ##.###
//! .##..#
//! .##...
//! .##...
//! #.#...
//! ##...#
//!
//! ```
//!
//! After `5` steps, this example now has `17` lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, but with the four
//! corners always in the on state, how many lights are on after 100 steps?

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

/// Part 1: In your grid of 100x100 lights, given your initial configuration,
/// how many lights are on after 100 steps?
#[aoc(day18, part1)]
fn part1(input: &[Vec<bool>]) -> usize {
    let grid = evolve1(input, 100);
    count_on(&grid)
}

fn evolve1(input: &[Vec<bool>], steps: usize) -> Vec<Vec<bool>> {
    let mut next = Vec::from(input);
    for _ in 0..steps {
        next = execute_step(&next);
    }
    next
}

fn execute_step(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut next = create_grid(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut alive_neighbors = 0;
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    let y = y as isize + dy;
                    let x = x as isize + dx;
                    if (dx == 0 && dy == 0)
                        || y < 0
                        || y >= height as isize
                        || x < 0
                        || x >= width as isize
                    {
                        continue;
                    }
                    if grid[y as usize][x as usize] {
                        alive_neighbors += 1;
                    }
                }
            }

            next[y][x] = match grid[y][x] {
                // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                true => alive_neighbors == 2 || alive_neighbors == 3,
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                false => alive_neighbors == 3,
            }
        }
    }
    next
}

fn create_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut rows = Vec::new();
    for _ in 0..height {
        rows.push(vec![false; width]);
    }
    rows
}

fn count_on(grid: &[Vec<bool>]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|col| **col).count())
        .sum()
}

/// Part 2: In your grid of 100x100 lights, given your initial configuration, but with the four
/// corners always in the on state, how many lights are on after 100 steps?
#[aoc(day18, part2)]
fn part2(input: &[Vec<bool>]) -> usize {
    let grid = evolve2(input, 100);
    count_on(&grid)
}

fn evolve2(input: &[Vec<bool>], steps: usize) -> Vec<Vec<bool>> {
    let mut next = Vec::from(input);
    for _ in 0..steps {
        fill_corners(&mut next);
        next = execute_step(&next);
    }
    fill_corners(&mut next);
    next
}

/// implements "Four lights, one in each corner, are stuck on and can't be turned off." on grid
fn fill_corners(grid: &mut [Vec<bool>]) {
    let width = grid[0].len();
    let height = grid.len();
    grid[0][0] = true;
    grid[0][width - 1] = true;
    grid[height - 1][0] = true;
    grid[height - 1][width - 1] = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    const EXAMPLE_2: &str = "##.#.#
...##.
#....#
..#...
#.#..#
####.#";

    fn display_grid(grid: &[Vec<bool>]) -> String {
        let mut s = String::new();
        for row in grid.iter() {
            for col in row {
                if *col {
                    s += "#";
                } else {
                    s += ".";
                }
            }
            s += "\n";
        }
        s.trim().to_string()
    }

    #[test]
    fn part1_examples() {
        // After `4` steps, this example has four lights on.
        let grid = evolve1(&parse_input(EXAMPLE_1), 4);
        assert_eq!(4, count_on(&grid));
        assert_eq!(
            "......
......
..##..
..##..
......
......",
            display_grid(&grid)
        );
    }

    #[test]
    fn part2_examples() {
        // After `5` steps, this example now has `17` lights on.
        let grid = evolve2(&parse_input(EXAMPLE_2), 5);
        assert_eq!(17, count_on(&grid));
        assert_eq!(
            "##.###
.##..#
.##...
.##...
#.#...
##...#",
            display_grid(&grid)
        );
    }
}
