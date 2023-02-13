use bevy::prelude::*;

use crate::{
    buffered_input::{InputInversionFlag, MoveImmediate},
    ui::*,
};

use super::spawn_popup_panel;

pub fn spawn_popup_settings(
    mut commands: Commands,
    game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
    input_system: Query<(&InputInversionFlag, &MoveImmediate)>,
    game_mode: Res<State<GameMode>>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let (input_inversion, move_immediate) = input_system.single();
    let button_close_image = UiImage::from(asset_server.load("images/button_close.png"));
    let button_toggle_on_image = UiImage::from(asset_server.load("images/button_toggle_on.png"));
    let button_toggle_off_image = UiImage::from(asset_server.load("images/button_toggle_off.png"));

    commands
        .entity(game_ui_query.single())
        .with_children(|parent| {
            spawn_popup_panel(parent, button_close_image.clone(), font.clone(), |parent| {
                // settings text
                parent.spawn(
                    TextBundle::from_section(
                        "Settings",
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

                // animation toggle button
                spawn_toggle_button(
                    parent,
                    UiRect {
                        right: Val::Px(185.0),
                        top: Val::Px(140.0),
                        ..default()
                    },
                    "Animation".to_string(),
                    font.clone(),
                    MyButtonType::AnimationToggle,
                    match move_immediate.0 {
                        true => button_toggle_off_image.clone(),
                        false => button_toggle_on_image.clone(),
                    },
                );

                // input inversion button
                spawn_toggle_button(
                    parent,
                    UiRect {
                        right: Val::Px(185.0),
                        top: Val::Px(210.0),
                        ..default()
                    },
                    "Input Inversion".to_string(),
                    font.clone(),
                    MyButtonType::InputInversion,
                    match input_inversion.0 {
                        true => button_toggle_on_image.clone(),
                        false => button_toggle_off_image.clone(),
                    },
                );

                // share button
                spawn_image_button(
                    parent,
                    UiRect {
                        left: Val::Px(50.0),
                        bottom: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::Share,
                    asset_server.load("images/button_share.png").into(),
                    "Share URL of current game".to_string(),
                    font.clone(),
                );

                // share result UI
                parent.spawn((
                    TextBundle::from_sections([
                        TextSection::new(
                            "",
                            TextStyle {
                                font: font.clone(),
                                font_size: TEXT_SIZE,
                                color: Color::GRAY,
                            },
                        ),
                        TextSection::new(
                            "",
                            TextStyle {
                                font: font.clone(),
                                font_size: TEXT_SIZE * 0.7,
                                color: Color::GRAY,
                            },
                        ),
                    ])
                    .with_text_alignment(TextAlignment::CENTER_LEFT)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(170.0),
                            bottom: Val::Px(68.0),
                            ..default()
                        },
                        ..default()
                    }),
                    MyTextType::ShareURL,
                ));

                // load URL UI: practice mode only
                if *game_mode.current() == GameMode::Practice {
                    // load URL button
                    spawn_image_button(
                        parent,
                        UiRect {
                            bottom: Val::Px(170.0),
                            left: Val::Px(50.0),
                            ..default()
                        },
                        MyButtonType::LoadURL,
                        asset_server.load("images/button_load.png").into(),
                        "Load URL".to_string(),
                        font.clone(),
                    );

                    // load URL guide text
                    parent.spawn(
                        TextBundle::from_section(
                            "Enter URL (after \"/?\"):",
                            TextStyle {
                                font: font.clone(),
                                font_size: TEXT_SIZE,
                                color: Color::GRAY,
                            },
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                bottom: Val::Px(225.0),
                                left: Val::Px(170.0),
                                ..default()
                            },
                            ..default()
                        }),
                    );

                    // load URL text
                    spawn_text_input_box(
                        parent,
                        UiRect {
                            bottom: Val::Px(175.0),
                            left: Val::Px(170.0),
                            ..default()
                        },
                        Size::new(Val::Px(380.0), Val::Px(40.0)),
                        font.clone(),
                        12,
                    );
                }
            });
        });
}

fn spawn_toggle_button(
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
                    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
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
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(50.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        });
}
