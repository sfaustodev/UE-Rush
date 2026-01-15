use bevy::prelude::*;
use crate::components::*;
use crate::game_rules::*;

pub fn move_entity(
    position: &mut Position,
    transform: &mut Transform,
    target_pos: IVec2,
    range: i32,
    grid_size: i32,
) -> bool {
    let distance = (target_pos - position.0).abs().max_element();
    if distance <= range && distance > 0 && target_pos.x >= 0 && target_pos.x < grid_size && target_pos.y >= 0 && target_pos.y < grid_size {
        transform.translation.x = target_pos.x as f32 * 32.0;
        transform.translation.y = target_pos.y as f32 * 32.0;
        position.0 = target_pos;
        true
    } else {
        false
    }
}

pub fn check_opportunity_attack_trigger(
    old_pos: IVec2,
    new_pos: IVec2,
    enemy_positions: &Vec<IVec2>,
) -> bool {
    let in_range_before = enemy_positions.iter().any(|&e_pos| is_within_range(e_pos, old_pos, 1));
    let in_range_after = enemy_positions.iter().any(|&e_pos| is_within_range(e_pos, new_pos, 1));
    in_range_before && !in_range_after
}
