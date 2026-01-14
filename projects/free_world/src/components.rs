use bevy::prelude::*;
use crate::resources::Action;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct MaxHealth(pub i32);

#[derive(Component)]
pub struct ArmorClass(pub i32);

#[derive(Component)]
pub struct AttackBonus(pub i32);

#[derive(Component)]
pub struct Damage {
    pub num_dice: u32,
    pub dice_sides: u32,
    pub bonus: i32,
}

#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct IsEnemy;

#[derive(Component)]
pub struct MovementRange(pub i32);

#[derive(Component)]
pub struct HasAction(pub bool);

#[derive(Component)]
pub struct HasBonusAction(pub bool);

#[derive(Component)]
pub struct HasMovement(pub bool);

#[derive(Component)]
pub struct Dodging(pub bool);

#[derive(Component)]
pub struct Level(pub i32);

#[derive(Component)]
pub struct Experience(pub i32);

#[derive(Component)]
pub struct Skills {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Component)]
pub struct MagicPoints(pub i32);

#[derive(Component)]
pub struct GridTile;

#[derive(Component)]
pub struct ActionButton(pub Action);

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct DiceButton;
