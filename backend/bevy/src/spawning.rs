use bevy::prelude::*;
use crate::components::*;

pub fn spawn_grid(commands: &mut Commands, grid_size: i32) {
    for x in 0..grid_size {
        for y in 0..grid_size {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.5, 0.5, 0.5),
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 0.0),
                    ..default()
                },
                Position(IVec2::new(x, y)),
                GridTile,
            ));
        }
    }
}

pub fn spawn_player(commands: &mut Commands, pos: IVec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 1.0),
            ..default()
        },
        Position(pos),
        Health(12),
        MaxHealth(12),
        ArmorClass(16),
        AttackBonus(4),
        Damage { num_dice: 1, dice_sides: 8, bonus: 2 },
        IsPlayer,
        MovementRange(6),
        HasAction(true),
        HasBonusAction(false),
        HasMovement(true),
        Dodging(false),
    )).insert((
        Level(1),
        Experience(0),
        Skills {
            strength: 15,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
        },
        MagicPoints(10),
    ));
}

pub fn spawn_enemy(commands: &mut Commands, pos: IVec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x as f32 * 32.0, pos.y as f32 * 32.0, 1.0),
            ..default()
        },
        Position(pos),
        Health(7),
        MaxHealth(7),
        ArmorClass(15),
        AttackBonus(4),
        Damage { num_dice: 1, dice_sides: 6, bonus: 2 },
        IsEnemy,
        MovementRange(6),
        HasAction(true),
        HasBonusAction(false),
        HasMovement(true),
        Dodging(false),
    ));
}

pub fn spawn_entities(commands: &mut Commands) {
    spawn_player(commands, IVec2::new(0, 0));
    spawn_enemy(commands, IVec2::new(9, 9));
}
