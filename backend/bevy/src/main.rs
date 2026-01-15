
mod resources;
mod components;
mod systems;
mod game_rules;
mod combat;
mod movement;
mod ui;
mod ai;
mod spawning;
mod db;

use bevy::prelude::*;
use resources::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameState {
    Exploring,
    Combat { grid: Vec<Vec<Entity>>, enemy: Entity },
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let player_data = args.get(1).cloned().unwrap_or_else(|| "{}".to_string());
    println!("Player data: {}", player_data);
    // TODO: Deserialize and use player_data
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "UE-Rush Bevy Game".to_string(),
                resolution: (640.0, 480.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(TurnState(Turn::Player))
        .insert_resource(SelectedAction(Action::None))
        .insert_resource(PendingRoll { action: None, target: None })
        .add_systems(Startup, (
            |mut commands: Commands| spawning::spawn_grid(&mut commands, 10),
            |mut commands: Commands| spawning::spawn_entities(&mut commands),
            ui::setup_ui,
            setup_camera
        ))
        .add_systems(Update, (systems::handle_input.run_if(|turn_state: Res<TurnState>| turn_state.0 == Turn::Player), ui::button_system, systems::enemy_turn.run_if(|turn_state: Res<TurnState>| turn_state.0 == Turn::Enemy), systems::leveling_system))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
