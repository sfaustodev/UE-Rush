use bevy::prelude::*;
use crate::components::*;
use crate::resources::Action;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        // Top bar for buttons
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..default()
        }).with_children(|parent| {
            spawn_action_button(parent, "Attack", Action::Attack, Color::rgb(0.8, 0.2, 0.2));
            spawn_action_button(parent, "Move", Action::Move, Color::rgb(0.2, 0.8, 0.2));
            spawn_action_button(parent, "Dash", Action::Dash, Color::rgb(0.2, 0.2, 0.8));
            spawn_action_button(parent, "Dodge", Action::Dodge, Color::rgb(0.8, 0.8, 0.2));
            spawn_action_button(parent, "Ready", Action::Ready, Color::rgb(0.5, 0.5, 0.5));
            spawn_end_turn_button(parent);
            spawn_dice_button(parent);
        });

        // Bottom stats panel
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
            ..default()
        }).with_children(|parent| {
            // Player stats
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Player",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Health: 12/12",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::GREEN,
                        ..default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "AC: 16 | Attack: +4",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::YELLOW,
                        ..default()
                    },
                ));
            });

            // Enemy stats
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Enemy",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Health: 7/7",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::RED,
                        ..default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "AC: 15 | Attack: +4",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::YELLOW,
                        ..default()
                    },
                ));
            });
        });
    });
}

fn spawn_action_button(parent: &mut ChildBuilder, text: &str, action: Action, color: Color) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        },
        ActionButton(action),
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn spawn_end_turn_button(parent: &mut ChildBuilder) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::rgb(0.8, 0.4, 0.4).into(),
            ..default()
        },
        EndTurnButton,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "End Turn",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn spawn_dice_button(parent: &mut ChildBuilder) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..default()
        },
        DiceButton,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Roll",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

pub fn button_system(
    interaction_query: Query<(&Interaction, &ActionButton), Changed<Interaction>>,
    end_turn_interaction: Query<&Interaction, (Changed<Interaction>, With<EndTurnButton>)>,
    dice_interaction: Query<(&Interaction, &DiceButton), Changed<Interaction>>,
    mut selected_action: ResMut<crate::resources::SelectedAction>,
    mut pending_roll: ResMut<crate::resources::PendingRoll>,
    mut turn_state: ResMut<crate::resources::TurnState>,
) {
    for (interaction, action_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            selected_action.0 = action_button.0;
        }
    }

    for interaction in &end_turn_interaction {
        if *interaction == Interaction::Pressed {
            turn_state.0 = crate::resources::Turn::Enemy;
            println!("End turn");
        }
    }

    for (interaction, _) in &dice_interaction {
        if *interaction == Interaction::Pressed {
            if let Some(action) = pending_roll.action {
                if let Some(_target) = pending_roll.target {
                    // Perform the roll - this will be handled in combat system
                    println!("Rolling for {:?}", action);
                }
            }
        }
    }
}
