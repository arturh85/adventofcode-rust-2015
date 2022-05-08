//! # [Day 21: RPG Simulator 20XX](https://adventofcode.com/2015/day/21)
//!
//! Little Henry Case got a new video game for Christmas.
//! It's an [RPG](https://en.wikipedia.org/wiki/Role-playing_video_game), and he's stuck on a boss.
//! He needs to know what equipment to buy at the shop.
//! He hands you the [controller](https://en.wikipedia.org/wiki/Game_controller).
//!
//! In this game, the player (you) and the enemy (the boss) take turns attacking.
//! The player always goes first. Each attack reduces the opponent's hit points by at least `1`.
//! The first character at or below `0` hit points loses.
//!
//! Damage dealt by an attacker each turn is equal to the attacker's damage score minus the
//! defender's armor score. An attacker always does at least `1` damage. So, if the attacker has a
//! damage score of `8`, and the defender has an armor score of `3`, the defender loses
//! `5` hit points. If the defender had an armor score of `300`, the defender would
//! still lose `1` hit point.
//!
//! Your damage score and armor score both start at zero. They can be increased by buying items in
//! exchange for gold. You start with no items and have as much gold as you need. Your total damage
//! or armor is equal to the sum of those stats from all of your items. You have 100 hit points.
//!
//! Here is what the item shop is selling:
//!
//! ```plain
//! Weapons:    Cost  Damage  Armor
//! Dagger        8     4       0
//! Shortsword   10     5       0
//! Warhammer    25     6       0
//! Longsword    40     7       0
//! Greataxe     74     8       0
//!
//! Armor:      Cost  Damage  Armor
//! Leather      13     0       1
//! Chainmail    31     0       2
//! Splintmail   53     0       3
//! Bandedmail   75     0       4
//! Platemail   102     0       5
//!
//! Rings:      Cost  Damage  Armor
//! Damage +1    25     1       0
//! Damage +2    50     2       0
//! Damage +3   100     3       0
//! Defense +1   20     0       1
//! Defense +2   40     0       2
//! Defense +3   80     0       3
//! ```
//!
//! You must buy exactly one weapon; no dual-wielding. Armor is optional, but you can't use more
//! than one. You can buy 0-2 rings (at most one for each hand). You must use any items you buy.
//! The shop only has one of each item, so you can't buy, for example, two rings of Damage +3.
//!
//! For example, suppose you have `8` hit points, `5` damage, and `5` armor, and that the boss has
//! `12` hit points, `7` damage, and `2` armor:
//!
//! - The player deals `5-2 = 3` damage; the boss goes down to 9 hit points.
//! - The boss deals `7-5 = 2` damage; the player goes down to 6 hit points.
//! - The player deals `5-2 = 3` damage; the boss goes down to 6 hit points.
//! - The boss deals `7-5 = 2` damage; the player goes down to 4 hit points.
//! - The player deals `5-2 = 3` damage; the boss goes down to 3 hit points.
//! - The boss deals `7-5 = 2` damage; the player goes down to 2 hit points.
//! - The player deals `5-2 = 3` damage; the boss goes down to 0 hit points.
//!
//! In this scenario, the player wins! (Barely.)
//!
//! You have 100 hit points. The boss's actual stats are in your puzzle input.
//! **What is the least amount of gold you can spend and still win the fight?**
//!
//! # Part 2
//!
//! Turns out the shopkeeper is working with the boss, and can persuade you to buy whatever items
//! he wants. The other rules still apply, and he still only has one of each item.
//! **What is the most amount of gold you can spend and still lose the fight?**

type Stats = (u64, u64, u64);
type Item = (u64, u64, u64);

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Stats {
    let numbers: Vec<u64> = input
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(": ").collect();
            tokens[1].parse().unwrap()
        })
        .collect();
    (numbers[0], numbers[1], numbers[2])
}

/// Part 1: What is the least amount of gold you can spend and still win the fight?
#[aoc(day21, part1)]
fn part1(input: &Stats) -> u64 {
    min_shop_gold(input)
}

fn min_shop_gold(boss: &Stats) -> u64 {
    let weapons: Vec<Item> = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armors: Vec<Item> = vec![(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    let rings: Vec<Item> = vec![
        (20, 0, 1),
        (25, 1, 0),
        (40, 0, 2),
        (50, 2, 0),
        (80, 0, 3),
        (100, 3, 0),
    ];
    // You must buy exactly one weapon; no dual-wielding.
    let mut best_by_weapon = Vec::new();
    for weapon in &weapons {
        let gold = weapon.0;
        let player = (100, weapon.1, weapon.2);
        if is_player_win(&player, boss) {
            best_by_weapon.push(gold);
            continue;
        }
        // Armor is optional, but you can't use more than one.
        // You can buy 0-2 rings (at most one for each hand).
        for item in armor_ring_combos(&armors, &rings) {
            let gold = gold + item.0;
            let stats = (player.0, player.1 + item.1, player.2 + item.2);
            if is_player_win(&stats, boss) {
                best_by_weapon.push(gold);
                break;
            }
        }
    }
    *best_by_weapon.iter().min().unwrap()
}

// You can buy 0-2 rings (at most one for each hand).
fn ring_combos(rings: &[Item]) -> Vec<Item> {
    let mut combos = Vec::new();
    for a in rings {
        combos.push(*a);
        for b in rings {
            if a != b {
                combos.push(combine(a, b));
            }
        }
    }
    combos.sort_by(|a, b| a.0.cmp(&b.0));
    combos
}

fn combine(a: &Item, b: &Item) -> Item {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

// Combine ring options with armor
fn armor_ring_combos(armors: &[Item], rings: &[Item]) -> Vec<Item> {
    let rings = ring_combos(rings);
    let mut combos = rings.clone();
    for armor in armors {
        combos.push(*armor);
        for ring in &rings {
            combos.push(combine(armor, ring));
        }
    }
    combos.sort_by(|a, b| a.0.cmp(&b.0));
    combos
}

fn is_player_win(player: &Stats, boss: &Stats) -> bool {
    let mut player_turn = true;
    let mut player_hitpoints = player.0 as i64;
    let player_damage = player.1;
    let player_armor = player.2;
    let mut boss_hitpoints = boss.0 as i64;
    let boss_damage = boss.1;
    let boss_armor = boss.2;

    loop {
        if player_turn {
            let damage = (player_damage - boss_armor).max(1);
            boss_hitpoints -= damage as i64;
            if boss_hitpoints <= 0 {
                return true;
            }
        } else {
            let damage = (boss_damage - player_armor).max(1);
            player_hitpoints -= damage as i64;
            if player_hitpoints <= 0 {
                return false;
            }
        }
        player_turn = !player_turn;
    }
}

/// Part 2: What is the most amount of gold you can spend and still lose the fight?
#[aoc(day21, part2)]
fn part2(input: &Stats) -> u64 {
    max_shop_gold(input)
}

fn max_shop_gold(boss: &Stats) -> u64 {
    let mut weapons: Vec<Item> = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    weapons.reverse();
    let mut armors: Vec<Item> = vec![(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    armors.reverse();
    let mut rings: Vec<Item> = vec![
        (20, 0, 1),
        (25, 1, 0),
        (40, 0, 2),
        (50, 2, 0),
        (80, 0, 3),
        (100, 3, 0),
    ];
    rings.reverse();
    // You must buy exactly one weapon; no dual-wielding.
    let mut best_by_weapon = Vec::new();
    for weapon in &weapons {
        let gold = weapon.0;
        let player = (100, weapon.1, weapon.2);
        // Armor is optional, but you can't use more than one.
        // You can buy 0-2 rings (at most one for each hand).
        for item in armor_ring_combos(&armors, &rings).iter().rev() {
            let gold = gold + item.0;
            let stats = (player.0, player.1 + item.1, player.2 + item.2);
            if !is_player_win(&stats, boss) {
                best_by_weapon.push(gold);
                break;
            }
        }
    }
    *best_by_weapon.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PLAYER: Stats = (8, 5, 5);
    const EXAMPLE_BOSS: Stats = (12, 7, 2);

    #[test]
    fn part1_examples() {
        assert_eq!(true, is_player_win(&EXAMPLE_PLAYER, &EXAMPLE_BOSS));
    }
}
