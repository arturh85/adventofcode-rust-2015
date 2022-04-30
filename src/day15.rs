//! # [Day 15: Science for Hungry People](https://adventofcode.com/2015/day/15)
//!
//! Today, you set out on the task of perfecting your milk-dunking cookie recipe.
//! All you have to do is find the right balance of ingredients.
//!
//! Your recipe leaves room for exactly `100` teaspoons of ingredients. You make a list of the
//! remaining ingredients you could use to finish the recipe (your puzzle input) and their
//! properties per teaspoon:
//!
//! -   `capacity` (how well it helps the cookie absorb milk)
//! -   `durability` (how well it keeps the cookie intact when full of milk)
//! -   `flavor` (how tasty it makes the cookie)
//! -   `texture` (how it improves the feel of the cookie)
//! -   `calories` (how many calories it adds to the cookie)
//!
//! You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be
//! accurate so you can reproduce your results in the future. The total score of a cookie can be
//! found by adding up each of the properties (negative totals become `0`) and then multiplying
//! together everything except calories.
//!
//! For instance, suppose you have these two ingredients:
//!
//! ```plain
//! Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
//! Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
//! ```
//!
//! Then, choosing to use `44` teaspoons of butterscotch and `56` teaspoons of cinnamon (because
//! the amounts of each ingredient must add up to `100`) would result in a cookie with the
//! following properties:
//!
//! -   A `capacity` of `44*-1 + 56*2 = 68`
//! -   A `durability` of `44*-2 + 56*3 = 80`
//! -   A `flavor` of `44*6 + 56*-2 = 152`
//! -   A `texture` of `44*3 + 56*-1 = 76`
//!
//! Multiplying these together (`68 * 80 * 152 * 76`, ignoring `calories` for now) results in a
//! total score of `62842880`, which happens to be the best score possible given these ingredients.
//! If any properties had produced a negative total, it would have instead become zero, causing
//! the whole score to multiply to zero.
//!
//! **Given the ingredients in your kitchen and their properties, what is the total score of the
//! highest-scoring cookie you can make?**
//!
//! # Part Two
//!
//! Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe that
//! has exactly `500` calories per cookie (so they can use it as a meal replacement). Keep the rest
//! of your award-winning process the same (100 teaspoons, same ingredients, same scoring system).
//!
//! For example, given the ingredients above, if you had instead selected `40` teaspoons of
//! butterscotch and `60` teaspoons of cinnamon (which still adds to `100`), the total calorie
//! count would be `40*8 + 60*3 = 500`. The total score would go down, though: only `57600000`,
//! the best you can do in such trying circumstances.
//!
//! **Given the ingredients in your kitchen and their properties, what is the total score of the
//! highest-scoring cookie you can make with a calorie total of `500`?**

use regex::Regex;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> anyhow::Result<Vec<Ingredient>> {
    let mut ingredients: Vec<Ingredient> = Vec::new();
    // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
    let re = Regex::new(
        r"^(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)$",
    )?;
    for line in input.lines() {
        if let Some(matches) = re.captures(line) {
            let name = matches.name("name").unwrap().as_str().to_string();
            let capacity = matches.name("capacity").unwrap().as_str().parse()?;
            let durability = matches.name("durability").unwrap().as_str().parse()?;
            let flavor = matches.name("flavor").unwrap().as_str().parse()?;
            let texture = matches.name("texture").unwrap().as_str().parse()?;
            let calories = matches.name("calories").unwrap().as_str().parse()?;

            ingredients.push(Ingredient {
                name,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            })
        } else {
            return Err(anyhow!("failed to parse: {}", line));
        }
    }

    Ok(ingredients)
}

/// Part 1: what is the total score of the highest-scoring cookie you can make?
#[aoc(day15, part1)]
fn part1(input: &[Ingredient]) -> u64 {
    best_cookie_score1(input)
}

/// Part 2: what is the total score of the highest-scoring cookie you can make with a calorie total of `500`
#[aoc(day15, part2)]
fn part2(input: &[Ingredient]) -> u64 {
    best_cookie_score2(input)
}

#[allow(dead_code)]
#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn cookie_score(ingredients: &[Ingredient], ratios: &[i64]) -> u64 {
    let mut scores: Vec<i64> = vec![0, 0, 0, 0];
    assert_eq!(ingredients.len(), ratios.len());

    for (idx, ingredient) in ingredients.iter().enumerate() {
        scores[0] += ingredient.capacity * ratios[idx];
        scores[1] += ingredient.durability * ratios[idx];
        scores[2] += ingredient.flavor * ratios[idx];
        scores[3] += ingredient.texture * ratios[idx];
    }
    use std::iter::Product;
    Product::product(scores.iter().map(|s| if *s > 0 { *s as u64 } else { 0 }))
}

fn cookie_calories(ingredients: &[Ingredient], ratios: &[i64]) -> i64 {
    let mut calories = 0;
    for (idx, ratio) in ratios.iter().enumerate() {
        calories += ingredients[idx].calories * ratio;
    }
    calories
}

fn best_cookie_score1(ingredients: &[Ingredient]) -> u64 {
    let mut best = 0;
    for a in 1..100 {
        for b in 1..100 - a {
            for c in 1..100 - (a + b) {
                let d = 100 - a - b - c;
                let score = cookie_score(ingredients, &[a, b, c, d]);
                if score > best {
                    best = score
                }
            }
        }
    }
    best
}

fn best_cookie_score2(ingredients: &[Ingredient]) -> u64 {
    let mut best = 0;
    for a in 1..100i64 {
        for b in 1..100i64 - a {
            for c in 1..100i64 - (a + b) {
                let d = 100i64 - a - b - c;
                let score = cookie_score(ingredients, &[a, b, c, d]);
                if score > best && cookie_calories(ingredients, &[a, b, c, d]) == 500 {
                    best = score
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    fn best_test_cookie_score1(ingredients: &Vec<Ingredient>) -> u64 {
        let mut best = 0;
        for a in 1..100 {
            let b = 100 - a;
            let score = cookie_score(ingredients, &vec![a, b]);
            if score > best {
                best = score
            }
        }
        best
    }

    fn best_test_cookie_score2(ingredients: &Vec<Ingredient>) -> u64 {
        let mut best = 0;
        for a in 1..100i64 {
            let b = 100i64 - a;
            let score = cookie_score(ingredients, &vec![a, b]);
            if score > best && cookie_calories(ingredients, &vec![a, b]) == 500 {
                best = score
            }
        }
        best
    }

    #[test]
    fn part1_examples_score() {
        // Then, choosing to use `44` teaspoons of butterscotch and `56` teaspoons of cinnamon (because
        // the amounts of each ingredient must add up to `100`) would result in a cookie with the
        // following properties:
        let ingredients = parse_input(EXAMPLE).expect("failed to parse");
        assert_eq!(cookie_score(&ingredients, &vec![44, 56]), 62842880);
    }

    #[test]
    fn part1_examples() {
        //  Multiplying these together (`68 * 80 * 152 * 76`, ignoring `calories` for now) results in a
        // total score of `62842880`, which happens to be the best score possible given these ingredients.
        let ingredients = parse_input(EXAMPLE).expect("failed to parse");
        assert_eq!(best_test_cookie_score1(&ingredients), 62842880);
    }

    #[test]
    fn part2_examples() {
        // The total score would go down, though: only `57600000`,
        // the best you can do in such trying circumstances.
        let ingredients = parse_input(EXAMPLE).expect("failed to parse");
        assert_eq!(best_test_cookie_score2(&ingredients), 57600000);
    }
}
