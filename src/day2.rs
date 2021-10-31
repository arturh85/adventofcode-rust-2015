//! # [Day 2: I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2)
//!
//! The elves are running low on wrapping paper, and so they need to submit an order
//! for more.
//! They have a list of the dimensions (length `l`, width `w`, and height `h`) of
//! each present,
//! and only want to order exactly as much as they need.
//!
//! Fortunately, every present is a box (a perfect
//! [right rectangular prism](https://en.wikipedia.org/wiki/Cuboid#Rectangular_cuboid)),
//! which makes calculating the required wrapping paper for each gift a
//! little easier:
//! find the surface area of the box, which is `2*l*w + 2*w*h + 2*h*l`.
//! The elves also need a little extra paper for each present: the area of the
//! smallest side.
//!
//! For example:
//!
//! -   A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square
//!     feet of wrapping paper plus `6` square feet of slack, for a total of `58`
//!     square feet.
//! -   A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square
//!     feet of wrapping paper plus `1` square foot of slack, for a total of `43`
//!     square feet.
//!
//! **All numbers in the elves' list are in feet. How many total square feet of
//! wrapping paper should they order?**
//!
//! # Part Two
//!
//! The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry
//! about the length they need to order, which they would again like to be exact.
//!
//! The ribbon required to wrap a present is the shortest distance around its sides, or the smallest
//! perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet
//! of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
//! Don't ask how they tie the bow, though; they'll never tell.
//!
//! For example:
//!
//! - A present with dimensions `2x3x4` requires `2+2+3+3 = 10` feet of ribbon to wrap the
//! present plus `2*3*4 = 24` feet of ribbon for the bow, for a total of `34` feet.
//! - A present with dimensions `1x1x10` requires `1+1+1+1 = 4` feet of ribbon to wrap the present
//! plus `1*1*10 = 10` feet of ribbon for the bow, for a total of `14` feet.
//!
//! **How many total feet of ribbon should they order?**

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<Present>, std::num::ParseIntError> {
    input
        .lines()
        .map(|l| {
            // l = 2x3x4
            l.split('x')
                .map(|f| f.parse::<u32>())
                .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
                .map(|mut p| {
                    p.sort();
                    Present {
                        l: p[0],
                        w: p[1],
                        h: p[2],
                    }
                })
        })
        .collect()
}

/// All numbers in the elves' list are in feet. How many total square feet of wrapping paper
/// should they order?
#[aoc(day2, part1)]
fn part1(presents: &[Present]) -> u32 {
    presents
        .iter()
        .map(|present| present.paper_required())
        .sum()
}

/// How many total feet of ribbon should they order?
#[aoc(day2, part2)]
fn part2(presents: &[Present]) -> u32 {
    presents
        .iter()
        .map(|present| present.ribbon_required())
        .sum()
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn paper_required(&self) -> u32 {
        3 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }
    fn ribbon_required(&self) -> u32 {
        2 * (self.l + self.w) + (self.l * self.w) * self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let present234 = Present { l: 2, w: 3, h: 4 };
        /* A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square
        feet of wrapping paper plus `6` square feet of slack, for a total of `58`
        square feet. */
        assert_eq!(present234.paper_required(), 58);

        /* A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square
        feet of wrapping paper plus `1` square foot of slack, for a total of `43`
        square feet. */
        let present1110 = Present { l: 1, w: 1, h: 10 };
        assert_eq!(present1110.paper_required(), 43);
    }

    #[test]
    fn part2_examples() {
        let present234 = Present { l: 2, w: 3, h: 4 };
        /* A present with dimensions `2x3x4` requires `2+2+3+3 = 10` feet of ribbon to wrap the
        present plus `2*3*4 = 24` feet of ribbon for the bow, for a total of `34` feet. */
        assert_eq!(present234.ribbon_required(), 34);

        /* A present with dimensions `1x1x10` requires `1+1+1+1 = 4` feet of ribbon to wrap the present
        plus `1*1*10 = 10` feet of ribbon for the bow, for a total of `14` feet. */
        let present1110 = Present { l: 1, w: 1, h: 10 };
        assert_eq!(present1110.ribbon_required(), 14);
    }
}
