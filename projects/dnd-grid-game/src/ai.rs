use bevy::prelude::*;
use crate::components::*;
use crate::movement::*;
use crate::combat::*;

pub fn enemy_turn_system(
    mut turn_state: ResMut<crate::resources::TurnState>,
    mut enemy_query: Query<(&mut Position, &mut Transform, &IsEnemy, &AttackBonus, &Damage)>,
    player_query: Query<(&Position, &ArmorClass, &IsPlayer)>,
) {
    if turn_state.0 == crate::resources::Turn::Enemy {
        if let Some((player_pos, player_ac, _)) = player_query.iter().next() {
            for (mut e_pos, mut e_transform, _, attack_bonus, damage) in enemy_query.iter_mut() {
                // Simple AI: move towards player
                let dx = player_pos.0.x - e_pos.0.x;
                let dy = player_pos.0.y - e_pos.0.y;
                let move_x = if dx > 0 { 1 } else if dx < 0 { -1 } else { 0 };
                let move_y = if dy > 0 { 1 } else if dy < 0 { -1 } else { 0 };
                let new_pos = IVec2::new(e_pos.0.x + move_x, e_pos.0.y + move_y);
                move_entity(e_pos.as_mut(), e_transform.as_mut(), new_pos, 6, 10); // Assume range 6, grid 10

                // If adjacent, attack
                if is_adjacent(player_pos.0, e_pos.0) {
                    if let Some(dmg) = resolve_attack(e_pos.0, player_pos.0, attack_bonus.0, player_ac.0, damage) {
                        // Apply damage to player
                        // Need to modify player health, but since query is separate, perhaps use events or commands
                        println!("Enemy deals {} damage to player", dmg);
                    }
                }
            }
        }
        turn_state.0 = crate::resources::Turn::Player;
    }
}
