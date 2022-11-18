mod block;
mod buffered_input;
mod game;
mod game_ui;
mod player;

use bevy::{prelude::*, DefaultPlugins};
use buffered_input::CustomInputPlugin;
use game::{GamePlugin, MoveTimer};
use game_ui::GameUIPlugin;
use player::PlayerPlugin;

#[cfg(not(feature = "debug"))]
fn main() {
    use buffered_input::InputTimer;

    App::new()
        .init_resource::<MoveTimer>()
        .init_resource::<InputTimer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameUIPlugin)
        .add_plugin(CustomInputPlugin)
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
