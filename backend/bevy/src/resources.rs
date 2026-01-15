use bevy::prelude::*;

#[derive(Resource)]
pub struct TurnState(pub Turn);

#[derive(PartialEq, Eq)]
pub enum Turn {
    Player,
    Enemy,
}

#[derive(Resource)]
pub struct SelectedAction(pub Action);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    None,
    Attack,
    Move,
    Dash,
    Dodge,
    Ready,
}

#[derive(Resource)]
pub struct GameState {
    pub player_turn: bool,
    pub game_over: bool,
}

#[derive(Resource)]
pub struct PendingRoll {
    pub action: Option<Action>,
    pub target: Option<IVec2>,
}
