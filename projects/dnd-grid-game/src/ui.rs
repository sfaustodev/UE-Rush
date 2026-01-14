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
            spawn_dice_button(parent);
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
    dice_interaction: Query<(&Interaction, &DiceButton), Changed<Interaction>>,
    mut selected_action: ResMut<crate::resources::SelectedAction>,
    mut pending_roll: ResMut<crate::resources::PendingRoll>,
) {
    for (interaction, action_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            selected_action.0 = action_button.0;
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
