use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::game_rules::*;
use crate::spawning::*;
use crate::ui::*;
use crate::movement::*;
use crate::combat::*;
use crate::ai::*;

pub fn spawn_grid(mut commands: Commands) {
    spawning::spawn_grid(commands, 10);
}

pub fn spawn_entities(mut commands: Commands) {
    spawning::spawn_player(commands, IVec2::new(0, 0));
    spawning::spawn_enemy(commands, IVec2::new(9, 9));
}

pub fn handle_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut selected_action: ResMut<SelectedAction>,
    mut turn_state: ResMut<TurnState>,
    mut pending_roll: ResMut<PendingRoll>,
    mut query: Query<(&mut Position, &mut Transform, &IsPlayer, &mut HasMovement, &MovementRange, &HasAction), Without<IsEnemy>>,
    enemy_query: Query<(&Position, &IsEnemy), Without<IsPlayer>>,
) {
    // Select action with keys
    if keys.just_pressed(KeyCode::KeyA) {
        selected_action.0 = Action::Attack;
        println!("Selected Attack");
    } else if keys.just_pressed(KeyCode::KeyM) {
        selected_action.0 = Action::Move;
        println!("Selected Move");
    } else if keys.just_pressed(KeyCode::KeyD) {
        selected_action.0 = Action::Dash;
        println!("Selected Dash");
    } else if keys.just_pressed(KeyCode::KeyG) {
        selected_action.0 = Action::Dodge;
        println!("Selected Dodge");
    } else if keys.just_pressed(KeyCode::KeyR) {
        selected_action.0 = Action::Ready;
        println!("Selected Ready");
    } else if keys.just_pressed(KeyCode::KeyE) {
        // End turn
        turn_state.0 = Turn::Enemy;
        println!("End turn");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let grid_x = (world_pos.x / 32.0).round() as i32;
                let grid_y = (world_pos.y / 32.0).round() as i32;
                let target_pos = IVec2::new(grid_x, grid_y);

                if turn_state.0 == Turn::Player {
                    for (mut pos, mut transform, _, mut has_movement, range, has_action) in query.iter_mut() {
                        if selected_action.0 == Action::Move || selected_action.0 == Action::Dash {
                            let old_pos = pos.0;
                            if movement::move_entity(&mut pos, &mut transform, target_pos, range.0, 10) {
                                if selected_action.0 == Action::Move {
                                    *has_movement = HasMovement(false);
                                }
                                selected_action.0 = Action::None;

                                // Check for opportunity attack
                                let enemy_positions: Vec<IVec2> = enemy_query.iter().map(|(e_pos, _)| e_pos.0).collect();
                                if movement::check_opportunity_attack_trigger(old_pos, pos.0, &enemy_positions) {
                                    combat::perform_opportunity_attack(&mut commands, &enemy_query, pos.0, 16); // Player AC
                                }
                            }
                        } else if selected_action.0 == Action::Attack {
                            if let Some((e_pos, _)) = enemy_query.iter().find(|(e_pos, _)| e_pos.0 == target_pos) {
                                if is_adjacent(e_pos.0, pos.0) {
                                    // Set pending roll
                                    pending_roll.action = Some(Action::Attack);
                                    pending_roll.target = Some(target_pos);
                                    selected_action.0 = Action::None;
                                    println!("Pending attack roll on {:?}", target_pos);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}



// Setup UI
pub fn setup_ui(mut commands: Commands) {
    ui::setup_ui(commands);
}

// Button interaction system
pub fn button_system(
    mut interaction_query: Query<(&Interaction, &ActionButton), Changed<Interaction>>,
    mut dice_interaction: Query<(&Interaction, &DiceButton), Changed<Interaction>>,
    mut selected_action: ResMut<SelectedAction>,
    mut pending_roll: ResMut<PendingRoll>,
) {
    ui::button_system(interaction_query, dice_interaction, selected_action, pending_roll);
}

// Enemy turn simple AI
pub fn enemy_turn(
    mut turn_state: ResMut<TurnState>,
    mut enemy_query: Query<(&mut Position, &mut Transform, &IsEnemy, &AttackBonus, &Damage)>,
    player_query: Query<(&Position, &ArmorClass, &IsPlayer)>,
) {
    ai::enemy_turn_system(turn_state, enemy_query, player_query);
}
