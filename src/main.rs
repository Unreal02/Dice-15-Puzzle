mod block;
mod buffered_input;
mod daily_puzzle_info;
mod game;
mod local_storage;
mod network;
mod player;
mod statistics_manager;
mod ui;
mod utils;

use bevy::{prelude::*, DefaultPlugins};
use bevy_mod_picking::DefaultPickingPlugins;
use buffered_input::CustomInputPlugin;
use buffered_input::InputTimer;
use daily_puzzle_info::DailyPuzzleInfoPlugin;
use game::{GamePlugin, MoveTimer};
use network::NetworkPlugin;
use player::PlayerPlugin;
use statistics_manager::StatisticsManagerPlugin;
use ui::*;

#[cfg(not(feature = "debug"))]
fn main() {
    App::new()
        .init_resource::<MoveTimer>()
        .init_resource::<InputTimer>()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(NetworkPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameUIPlugin)
        .add_plugin(CustomInputPlugin)
        .add_plugin(PopupPlugin)
        .add_plugin(GameModeUIPlugin)
        .add_plugin(StatisticsManagerPlugin)
        .add_plugin(DailyPuzzleInfoPlugin)
        .add_plugin(HowToPlayPlugin)
        .run();
}

#[cfg(feature = "debug")]
fn main() {
    use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
    use game::GameState;
    App::new()
        .init_resource::<MoveTimer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<GameState>()
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameUIPlugin)
        .run();
}
