mod block;
mod game;
mod game_ui;
mod player;

use bevy::{prelude::*, DefaultPlugins};
use game::GamePlugin;
use game_ui::GameUIPlugin;
use player::PlayerPlugin;

#[cfg(not(feature = "debug"))]
fn main() {
    App::new()
        .init_resource::<Timer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameUIPlugin)
        .run();
}

#[cfg(feature = "debug")]
fn main() {
    use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
    use game::GameState;
    App::new()
        .init_resource::<Timer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<GameState>()
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameUIPlugin)
        .run();
}
