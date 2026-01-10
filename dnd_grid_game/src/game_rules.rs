use bevy::prelude::*;
use rand::Rng;

pub fn roll_d20() -> i32 {
    rand::thread_rng().gen_range(1..=20)
}

pub fn roll_damage(num_dice: u32, dice_sides: u32, bonus: i32) -> i32 {
    let mut dmg = bonus;
    for _ in 0..num_dice {
        dmg += rand::thread_rng().gen_range(1..=dice_sides) as i32;
    }
    dmg
}

pub fn is_hit(attack_roll: i32, attack_bonus: i32, armor_class: i32) -> bool {
    attack_roll + attack_bonus >= armor_class
}

pub fn is_crit(attack_roll: i32) -> bool {
    attack_roll == 20
}

pub fn calculate_damage(damage_roll: i32, crit: bool) -> i32 {
    if crit { damage_roll * 2 } else { damage_roll }
}

// Movement: check if within range
pub fn is_within_range(pos1: IVec2, pos2: IVec2, range: i32) -> bool {
    (pos1 - pos2).abs().max_element() <= range
}

// Melee range: adjacent
pub fn is_adjacent(pos1: IVec2, pos2: IVec2) -> bool {
    is_within_range(pos1, pos2, 1)
}
