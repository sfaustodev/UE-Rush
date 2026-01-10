mod resources;
mod components;
mod systems;
mod game_rules;
mod combat;
mod movement;
mod ui;
mod ai;
mod spawning;

use bevy::prelude::*;
use crate::components::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "D&D Grid Game".to_string(),
                resolution: (640.0, 480.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(TurnState(Turn::Player))
        .insert_resource(SelectedAction(Action::None))
        .insert_resource(PendingRoll { action: None, target: None })
        .add_systems(Startup, (spawn_grid, spawn_entities, setup_ui, setup_camera))
        .add_systems(Update, (handle_input, button_system, enemy_turn))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
