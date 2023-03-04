use crate::ui::*;
use bevy::prelude::*;

pub fn spawn_popup_difficulty(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_close_image = UiImage::from(asset_server.load("images/button_close.png"));
    let button_small_image = UiImage::from(asset_server.load("images/button_small.png"));

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, button_close_image.clone(), font.clone(), |parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Difficulty (WIP)",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position: UiRect {
                            top: Val::Px(-250.0),
                            ..default()
                        },
                        ..default()
                    }),
                );
            })
        });
}

pub fn popup_system_difficulty() {}
