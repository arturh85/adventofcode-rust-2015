//! # [Day 22: Wizard Simulator 20XX](https://adventofcode.com/2015/day/22)
//!
//! Little Henry Case decides that defeating bosses with
//! [swords and stuff](https://adventofcode.com/2015/day/21) is boring. Now he's playing the game
//! with a wizard. Of course, he gets stuck on another boss and needs your help again.
//!
//! In this version, combat still proceeds with the player and the boss taking alternating turns.
//! The player still goes first. Now, however, you don't get any equipment; instead, you must
//! choose one of your SPELLS to cast. The first character at or below `0` hit points loses.
//!
//! Since you're a wizard, you don't get to wear armor, and you can't attack normally.
//! However, since you do magic damage, your opponent's armor is ignored, and so the boss
//! effectively has zero armor as well. As before, if armor (from a spell, in this case) would
//! reduce damage below `1`, it becomes `1` instead - that is, the boss' attacks always deal at
//! least `1` damage.
//!
//! On each of your turns, you must select one of your SPELLS to cast. If you cannot afford to
//! cast any spell, you lose. Spells cost mana; you start with 500 mana, but have no maximum limit.
//! You must have enough mana to cast a spell, and its cost is immediately deducted when you
//! cast it. Your SPELLS are Magic Missile, Drain, Shield, Poison, and Recharge.
//!
//! - Magic Missile costs `53` mana. It instantly does `4` damage.
//! - Drain costs `73` mana. It instantly does `2` damage and heals you for `2` hit points.
//! - Shield costs `113` mana. It starts an effect that lasts for `6` turns.
//!     While it is active, your armor is increased by `7`.
//! - Poison costs `173` mana. It starts an effect that lasts for `6` turns.
//!     At the start of each turn while it is active, it deals the boss `3` damage.
//! - Recharge costs `229` mana. It starts an effect that lasts for `5` turns.
//!     At the start of each turn while it is active, it gives you `101` new mana.
//!
//! Effects all work the same way. Effects apply at the start of both the player's turns and the
//! boss' turns. Effects are created with a timer (the number of turns they last); at the start of
//! each turn, after they apply any effect they have, their timer is decreased by one. If this
//! decreases the timer to zero, the effect ends. You cannot cast a spell that would start an
//! effect which is already active. However, effects can be started on the same turn they end.
//!
//! For example, suppose the player has `10` hit points and `250` mana, and that the
//! boss has `13` hit points and `8` damage:
//!
//! ```plain
//! -- Player turn --
//! - Player has 10 hit points, 0 armor, 250 mana
//! - Boss has 13 hit points
//! Player casts Poison.
//!
//! -- Boss turn --
//! - Player has 10 hit points, 0 armor, 77 mana
//! - Boss has 13 hit points
//! Poison deals 3 damage; its timer is now 5.
//! Boss attacks for 8 damage.
//!
//! -- Player turn --
//! - Player has 2 hit points, 0 armor, 77 mana
//! - Boss has 10 hit points
//! Poison deals 3 damage; its timer is now 4.
//! Player casts Magic Missile, dealing 4 damage.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 0 armor, 24 mana
//! - Boss has 3 hit points
//! Poison deals 3 damage. This kills the boss, and the player wins.
//! ```
//!
//! Now, suppose the same initial conditions, except that the boss has `14` hit points instead:
//!
//! ```plain
//! -- Player turn --
//! - Player has 10 hit points, 0 armor, 250 mana
//! - Boss has 14 hit points
//! Player casts Recharge.
//!
//! -- Boss turn --
//! - Player has 10 hit points, 0 armor, 21 mana
//! - Boss has 14 hit points
//! Recharge provides 101 mana; its timer is now 4.
//! Boss attacks for 8 damage!
//!
//! -- Player turn --
//! - Player has 2 hit points, 0 armor, 122 mana
//! - Boss has 14 hit points
//! Recharge provides 101 mana; its timer is now 3.
//! Player casts Shield, increasing armor by 7.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 7 armor, 110 mana
//! - Boss has 14 hit points
//! Shield's timer is now 5.
//! Recharge provides 101 mana; its timer is now 2.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 1 hit point, 7 armor, 211 mana
//! - Boss has 14 hit points
//! Shield's timer is now 4.
//! Recharge provides 101 mana; its timer is now 1.
//! Player casts Drain, dealing 2 damage, and healing 2 hit points.
//!
//! -- Boss turn --
//! - Player has 3 hit points, 7 armor, 239 mana
//! - Boss has 12 hit points
//! Shield's timer is now 3.
//! Recharge provides 101 mana; its timer is now 0.
//! Recharge wears off.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 2 hit points, 7 armor, 340 mana
//! - Boss has 12 hit points
//! Shield's timer is now 2.
//! Player casts Poison.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 7 armor, 167 mana
//! - Boss has 12 hit points
//! Shield's timer is now 1.
//! Poison deals 3 damage; its timer is now 5.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 1 hit point, 7 armor, 167 mana
//! - Boss has 9 hit points
//! Shield's timer is now 0.
//! Shield wears off, decreasing armor by 7.
//! Poison deals 3 damage; its timer is now 4.
//! Player casts Magic Missile, dealing 4 damage.
//!
//! -- Boss turn --
//! - Player has 1 hit point, 0 armor, 114 mana
//! - Boss has 2 hit points
//! Poison deals 3 damage. This kills the boss, and the player wins.
//! ```
//!
//! You start with 50 hit points and 500 mana points.
//! The boss's actual stats are in your puzzle input.
//! **What is the least amount of mana you can spend and still win the fight?**
//! (Do not include mana RECHARGE effects as "spending" negative mana.)
//!
//! # Part Two
//!
//! On the next run through the game, you increase the difficulty to hard.
//!
//! At the start of each player turn (before any other effects apply), you lose 1 hit point.
//! If this brings you to or below 0 hit points, you lose.
//!
//! With the same starting stats for you and the boss, **what is the least amount of mana you can
//! spend and still win the fight?**
use std::sync::{Arc, Mutex};

type Stats = (u64, u64);

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Stats {
    let numbers: Vec<u64> = input
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(": ").collect();
            tokens[1].parse().unwrap()
        })
        .collect();
    (numbers[0], numbers[1])
}

type Spell = (i64, i64, i64, u64, u64, i64, u64);

const MISSILE: Spell = (53, 4, 0, 0, 0, 0, 0);
const DRAIN: Spell = (73, 2, 2, 0, 0, 0, 1);
const SHIELD: Spell = (113, 0, 0, 7, 0, 6, 2);
const POISON: Spell = (173, 3, 0, 0, 0, 6, 3);
const RECHARGE: Spell = (229, 0, 0, 0, 101, 5, 4);
const SPELLS: [Spell; 5] = [MISSILE, DRAIN, SHIELD, POISON, RECHARGE];

/// Part 1: What is the least amount of mana you can spend and still win the fight?
#[aoc(day22, part1)]
fn part1(input: &Stats) -> u64 {
    let least_mana = Arc::new(Mutex::new(u64::MAX));
    sim(
        least_mana.clone(),
        input.0 as i64,
        input.1,
        50,
        500,
        vec![],
        true,
        0,
        false,
    );
    let least_mana = least_mana.lock().unwrap();
    *least_mana
}

#[allow(clippy::too_many_arguments)]
fn sim(
    least_mana_used: Arc<Mutex<u64>>,
    mut boss_hitpoints: i64,
    boss_damage: u64,
    mut player_hitpoints: i64,
    mut player_mana: i64,
    active_spells: Vec<Spell>,
    player_turn: bool,
    mana_used: u64,
    part_two: bool,
) -> bool {
    let mut player_armor = 0;
    if part_two && player_turn {
        player_hitpoints -= 1;
        if player_hitpoints <= 0 {
            return false;
        }
    }
    let mut new_active_spells: Vec<Spell> = vec![];
    for active_spell in active_spells {
        if active_spell.5 >= 0 {
            boss_hitpoints -= active_spell.1;
            player_hitpoints += active_spell.2;
            player_armor += active_spell.3;
            player_mana += active_spell.4 as i64;
        }
        let new_active_spell: Spell = (
            active_spell.0,
            active_spell.1,
            active_spell.2,
            active_spell.3,
            active_spell.4,
            active_spell.5 - 1,
            active_spell.6,
        );
        if new_active_spell.5 > 0 {
            new_active_spells.push(new_active_spell);
        }
    }
    {
        let mut least_mana_used = least_mana_used.lock().unwrap();
        if boss_hitpoints <= 0 {
            if mana_used < *least_mana_used {
                *least_mana_used = mana_used;
            }
            return true;
        }
        if mana_used >= *least_mana_used {
            return false;
        }
        drop(least_mana_used);
    }
    if player_turn {
        for spell in SPELLS {
            let mut spell_already_active = false;
            for new_active_spell in &new_active_spells {
                if new_active_spell.6 == spell.6 {
                    spell_already_active = true;
                    break;
                }
            }
            let spell_mana_cost = spell.0;
            if spell_mana_cost <= player_mana && !spell_already_active {
                let mut a: Vec<Spell> = new_active_spells.clone();
                a.push(spell);
                sim(
                    least_mana_used.clone(),
                    boss_hitpoints,
                    boss_damage,
                    player_hitpoints,
                    player_mana - spell_mana_cost,
                    a,
                    false,
                    mana_used + spell_mana_cost as u64,
                    part_two,
                );
            }
        }
    } else {
        let damage = (boss_damage - player_armor).max(1) as i64;
        player_hitpoints -= damage;
        if player_hitpoints > 0 {
            sim(
                least_mana_used,
                boss_hitpoints,
                boss_damage,
                player_hitpoints,
                player_mana,
                new_active_spells,
                true,
                mana_used,
                part_two,
            );
        }
    }
    true
}

/// Part 2: What is the least amount of mana you can spend and still win the fight?
#[aoc(day22, part2)]
fn part2(input: &Stats) -> u64 {
    let least_mana = Arc::new(Mutex::new(u64::MAX));
    sim(
        least_mana.clone(),
        input.0 as i64,
        input.1,
        50,
        500,
        vec![],
        true,
        0,
        true,
    );
    let least_mana = least_mana.lock().unwrap();
    *least_mana
}
