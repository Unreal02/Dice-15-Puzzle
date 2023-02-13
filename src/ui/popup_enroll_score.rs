use crate::ui::*;
use bevy::prelude::*;

pub fn spawn_popup_enroll_score(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_close_image = UiImage::from(asset_server.load("images/button_close.png"));

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, button_close_image.clone(), font.clone(), |parent| {
                // enroll score text
                parent.spawn(
                    TextBundle::from_section(
                        "Enroll Score",
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

                // time, move text
                parent.spawn((
                    TextBundle::from_section(
                        "".to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(180.0),
                            top: Val::Px(150.0),
                            ..default()
                        },
                        ..default()
                    }),
                    MyTextType::PlayerInfo,
                ));

                // user name input guide text
                parent.spawn(
                    TextBundle::from_section(
                        "Enter your name:",
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

                // user name text input box
                spawn_text_input_box(
                    parent,
                    UiRect {
                        bottom: Val::Px(175.0),
                        left: Val::Px(170.0),
                        ..default()
                    },
                    Size::new(Val::Px(380.0), Val::Px(40.0)),
                    font.clone(),
                    20,
                );

                // enroll score button
                spawn_image_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(170.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::EnrollScore,
                    asset_server.load("images/button_enroll_score.png").into(),
                    "Enroll Score".to_string(),
                    font.clone(),
                );

                // enroll score result text
                parent.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::GRAY,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    }),
                    MyTextType::EnrollDailyScoreResult,
                ));
            });
        });
}
