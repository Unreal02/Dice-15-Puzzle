use crate::{
    game::{MAX_BOARD_SIZE, MIN_BOARD_SIZE},
    ui::*,
};
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
                // difficulty text
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

                // board size
                parent.spawn(
                    TextBundle::from_section(
                        "Board size",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(94.0),
                            top: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    }),
                );

                // board size button
                for i in MIN_BOARD_SIZE..MAX_BOARD_SIZE + 1 {
                    spawn_small_button(
                        parent,
                        UiRect {
                            left: Val::Px(100.0),
                            top: Val::Px(40.0 + 60.0 * i as f32),
                            ..default()
                        },
                        format!("{} x {}", i, i),
                        font.clone(),
                        MyButtonType::SetBoardSize(i),
                        button_small_image.clone(),
                    );
                }

                // difficulty
                parent.spawn(
                    TextBundle::from_section(
                        "Difficulty",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(100.0),
                            top: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    }),
                );

                // difficulty button
                spawn_small_button(
                    parent,
                    UiRect {
                        right: Val::Px(100.0),
                        top: Val::Px(160.0),
                        ..default()
                    },
                    format!("Easy"),
                    font.clone(),
                    MyButtonType::SetDifficulty(true),
                    button_small_image.clone(),
                );
                spawn_small_button(
                    parent,
                    UiRect {
                        right: Val::Px(100.0),
                        top: Val::Px(220.0),
                        ..default()
                    },
                    format!("Hard"),
                    font.clone(),
                    MyButtonType::SetDifficulty(false),
                    button_small_image.clone(),
                );
            })
        });
}

fn spawn_small_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: MyButtonType,
    image: UiImage,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                    position_type: PositionType::Absolute,
                    position,
                    ..default()
                },
                image,
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size: TEXT_SIZE,
                        color: Color::BLACK,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER),
            );
        });
}
