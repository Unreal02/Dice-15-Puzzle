mod game_mode_ui;
mod game_ui;
mod game_ui_system;
mod popup;
mod popup_mode_selection;
mod popup_statistics;

pub use bevy::prelude::*;
pub use game_mode_ui::*;
pub use game_ui::*;
pub use game_ui_system::*;
pub use popup::*;
pub use popup_mode_selection::*;
pub use popup_statistics::*;

pub const TEXT_SIZE: f32 = 40.0;

pub const BUTTON_NORMAL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const BUTTON_HOVER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const BUTTON_PRESS_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component, PartialEq, Eq)]
pub enum MyButtonType {
    Reset,
    Shuffle,
    AnimationToggle,
    InputInversion,
    ModeSelection,
    Share,
    Undo,
    Redo,
    Statistics,
    DateSelection,
    Rankings,
    Export,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum MyTextType {
    AnimationToggle,
    InputInversion,
    ModeSelection,
    PlayerInfo,
    GameClear,
    Date,
}
