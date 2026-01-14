use bevy::prelude::*;
use crate::components::*;
use crate::movement::*;
use crate::combat::*;
use crate::game_rules::is_adjacent;

pub fn enemy_turn_system(
    mut turn_state: ResMut<crate::resources::TurnState>,
    enemy_query: Query<(&Position, &AttackBonus, &Damage, &IsEnemy)>,
) {
    if turn_state.0 == crate::resources::Turn::Enemy {
        // Assume player AC is 16, and player position is (0,0) for simplicity
        let player_ac = 16;
        let player_pos = IVec2::new(0, 0);
        for (e_pos, attack_bonus, damage, _) in enemy_query.iter() {
            // If adjacent, attack
            if is_adjacent(player_pos, e_pos.0) {
                if let Some(dmg) = resolve_attack(e_pos.0, player_pos, attack_bonus.0, player_ac, damage) {
                    // Apply damage to player
                    // Need to modify player health, but since query is separate, perhaps use events or commands
                    println!("Enemy deals {} damage to player", dmg);
                }
            }
        }
        turn_state.0 = crate::resources::Turn::Player;
    }
}
