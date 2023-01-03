// UIs depends on game mode

use bevy::{prelude::*, ui::FocusPolicy};

use crate::game_ui::TEXT_SIZE;

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
                parent.spawn(
                    TextBundle::from_section(
                        "Practice Mode UI Test",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(275.0),
                            top: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    }),
                );
            }
            GameMode::TimeAttack => {
                parent.spawn(
                    TextBundle::from_section(
                        "Time Attack Mode UI Test",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(275.0),
                            top: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    }),
                );
            }
            GameMode::MinimalMovement => {
                parent.spawn(
                    TextBundle::from_section(
                        "Minimal Movement Mode UI Test",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(275.0),
                            top: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    }),
                );
            }
            GameMode::DailyPuzzle => {
                parent.spawn(
                    TextBundle::from_section(
                        "Daily Puzzle Mode UI Test",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(275.0),
                            top: Val::Px(50.0),
                            ..default()
                        },
                        ..default()
                    }),
                );
            }
        });
}
