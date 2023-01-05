// UIs depends on game mode

use bevy::{prelude::*, ui::FocusPolicy};

use crate::ui::TEXT_SIZE;
use crate::MyButtonType;
use crate::{spawn_button, MyTextType};

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum GameMode {
    Practice,
    TimeAttack,
    MinimalMovement,
    DailyPuzzle,
}

#[derive(Component)]
pub struct GameModeUI;

pub struct GameModeUIPlugin;

impl Plugin for GameModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameMode::Practice)
            .add_system_set(SystemSet::on_enter(GameMode::Practice).with_system(spawn_game_mode_ui))
            .add_system_set(
                SystemSet::on_enter(GameMode::TimeAttack).with_system(spawn_game_mode_ui),
            )
            .add_system_set(
                SystemSet::on_enter(GameMode::MinimalMovement).with_system(spawn_game_mode_ui),
            )
            .add_system_set(
                SystemSet::on_enter(GameMode::DailyPuzzle).with_system(spawn_game_mode_ui),
            );
    }
}

fn spawn_game_mode_ui(
    mut commands: Commands,
    game_mode_ui_query: Query<Entity, With<GameModeUI>>,
    asset_server: Res<AssetServer>,
    game_mode: Res<State<GameMode>>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");

    if !game_mode_ui_query.is_empty() {
        commands
            .entity(game_mode_ui_query.single())
            .despawn_recursive();
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                ..default()
            },
            GameModeUI,
        ))
        .with_children(|parent| match game_mode.current() {
            GameMode::Practice => {
                // undo button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(87.5), Val::Px(87.5)),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                bottom: Val::Px(300.0),
                                right: Val::Px(162.5),
                                ..default()
                            },
                            ..default()
                        },
                        image: asset_server.load("images/button_undo.png").into(),
                        ..default()
                    },
                    MyButtonType::Undo,
                ));

                // redo button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(87.5), Val::Px(87.5)),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                bottom: Val::Px(300.0),
                                right: Val::Px(50.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: asset_server.load("images/button_redo.png").into(),
                        ..default()
                    },
                    MyButtonType::Redo,
                ));
            }
            GameMode::TimeAttack | GameMode::MinimalMovement => {
                // statistics button
                spawn_button(
                    parent,
                    UiRect {
                        top: Val::Px(275.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    "Statistics\n(WIP)".to_string(),
                    font.clone(),
                    MyButtonType::Statistics,
                    None,
                );
            }
            GameMode::DailyPuzzle => {
                // date text
                parent.spawn((
                    TextBundle::from_section(
                        "Date: 2023. 0. 0.",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(260.0),
                            left: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    }),
                    MyTextType::Date,
                ));

                // date selection button
                spawn_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(300.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    "Date\nSelection".to_string(),
                    font.clone(),
                    MyButtonType::DateSelection,
                    None,
                );

                // rankings button
                spawn_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(175.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    "Rankings".to_string(),
                    font.clone(),
                    MyButtonType::Rankings,
                    None,
                );
            }
        });
}