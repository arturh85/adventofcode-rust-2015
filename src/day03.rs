use std::num::ParseIntError;

/**
# [Day 3: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3)
Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and
then an elf at the North Pole calls him via radio and tells him where to move
next. Moves are always exactly one house to the north (`^`), south (`v`),
east (`>`), or west (`<`). After each move, he delivers another present to the
house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and
so his directions are a little off, and Santa ends up visiting some houses more
than once. **How many houses receive at least one present?**

For example:

-   `>` delivers presents to `2` houses: one at the starting location, and one
    to the east.
-   `^>v<` delivers presents to `4` houses in a square, including twice to the
    house at his starting/ending location.
-   `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at
    only `2` houses.
*/

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    presents
        .iter()
        .map(|present| present.paper_required())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        /* A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square
        feet of wrapping paper plus `6` square feet of slack, for a total of `58`
        square feet. */
        assert_eq!(Present { l: 2, w: 3, h: 4 }.paper_required(), 58);

        /* A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square
        feet of wrapping paper plus `1` square foot of slack, for a total of `43`
        square feet. */
        assert_eq!(Present { l: 1, w: 1, h: 10 }.paper_required(), 42);
    }

    // #[test]
    // fn part2_example() {
    //     // `)` causes him to enter the basement at character position `1`.
    //     assert_eq!(part2(")"), 1);
    //     // `()())` causes him to enter the basement at character position `5`.
    //     assert_eq!(part2("()())"), 5);
    // }
}
