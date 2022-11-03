mod block;
mod game;
mod player;

use bevy::{prelude::*, DefaultPlugins};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use game::{GamePlugin, GameState};
use player::PlayerPlugin;

fn main() {
    App::new()
        .init_resource::<Timer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<GameState>()
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .run();
}
