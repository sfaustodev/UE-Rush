use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::game_rules::*;
use crate::movement::*;
use crate::combat::*;
use crate::ai::*;

pub fn handle_input(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut selected_action: ResMut<SelectedAction>,
    mut turn_state: ResMut<TurnState>,
    mut pending_roll: ResMut<PendingRoll>,
    mut query: Query<(&mut Position, &mut Transform, &IsPlayer, &mut HasMovement, &MovementRange, &HasAction, &mut Experience), Without<IsEnemy>>,
    enemy_pos_query: Query<&Position, With<IsEnemy>>,
    enemy_attack_query: Query<(&Position, &AttackBonus, &Damage, &IsEnemy), Without<IsPlayer>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let grid_x = (world_pos.x / 32.0).round() as i32;
                let grid_y = (world_pos.y / 32.0).round() as i32;
                let target_pos = IVec2::new(grid_x, grid_y);

                if turn_state.0 == Turn::Player {
                    for (mut pos, mut transform, _, mut has_movement, range, _has_action, mut exp) in query.iter_mut() {
                        if selected_action.0 == Action::Move || selected_action.0 == Action::Dash {
                            let old_pos = pos.0;
                            if move_entity(&mut pos, &mut transform, target_pos, range.0, 10) {
                                if selected_action.0 == Action::Move {
                                    *has_movement = HasMovement(false);
                                }
                                selected_action.0 = Action::None;

                                // Check for opportunity attack
                                let enemy_positions: Vec<IVec2> = enemy_pos_query.iter().map(|p| p.0).collect();
                                if check_opportunity_attack_trigger(old_pos, pos.0, &enemy_positions) {
                                    perform_opportunity_attack(&mut commands, &enemy_attack_query, pos.0, 16); // Player AC
                                }
                            }
                        } else if selected_action.0 == Action::Attack {
                            if let Some((e_pos, _, _, _)) = enemy_attack_query.iter().find(|(e_pos, _, _, _)| e_pos.0 == target_pos) {
                                if is_adjacent(e_pos.0, pos.0) {
                                    // Set pending roll
                                    pending_roll.action = Some(Action::Attack);
                                    pending_roll.target = Some(target_pos);
                                    selected_action.0 = Action::None;
                                    exp.0 += 10; // Gain exp on attack
                                    println!("Pending attack roll on {:?}, gained 10 exp", target_pos);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}



// Leveling system
pub fn leveling_system(
    mut query: Query<(&mut Level, &mut Experience, &mut Skills, &mut MagicPoints, &mut Health, &mut MaxHealth), With<IsPlayer>>,
) {
    for (mut level, mut exp, mut skills, mut mp, mut health, mut max_health) in query.iter_mut() {
        let exp_needed = level.0 * 100; // Simple: 100 exp per level
        if exp.0 >= exp_needed {
            level.0 += 1;
            exp.0 -= exp_needed;
            // Increase stats
            skills.strength += 1;
            skills.dexterity += 1;
            skills.constitution += 1;
            skills.intelligence += 1;
            skills.wisdom += 1;
            skills.charisma += 1;
            max_health.0 += 2;
            health.0 = max_health.0; // Heal on level up
            mp.0 += 5;
            println!("Leveled up to level {}!", level.0);
        }
    }
}

// Enemy turn simple AI
pub fn enemy_turn(
    turn_state: ResMut<TurnState>,
    enemy_query: Query<(&Position, &AttackBonus, &Damage, &IsEnemy)>,
) {
    enemy_turn_system(turn_state, enemy_query);
}
