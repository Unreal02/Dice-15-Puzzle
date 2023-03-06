// UIs depends on game mode

use bevy::{prelude::*, ui::FocusPolicy};
use serde::{Deserialize, Serialize};

use crate::game::{GameState, MoveTimer};
use crate::network::{Network, NetworkChannel};
use crate::player::{PlayLog, PlayerInfo, PlayerState};
use crate::ui::TEXT_SIZE;
use crate::MyButtonType;
use crate::MyTextType;

use super::spawn_image_button;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum GameMode {
    Practice,
    TimeAttack,
    MinimalMovement,
    DailyPuzzle,
}

impl GameMode {
    pub fn entry_handler(
        &self,
        player_info: &mut PlayerInfo,
        game_state: &mut GameState,
        play_log: &mut PlayLog,
        transforms: &mut Query<&mut Transform>,
        move_timer: &mut ResMut<MoveTimer>,
        player_state: &mut ResMut<State<PlayerState>>,
        network_channel: &mut Res<NetworkChannel>,
    ) {
        fn reset(
            player_info: &mut PlayerInfo,
            game_state: &mut GameState,
            play_log: &mut PlayLog,
            transforms: &mut Query<&mut Transform>,
            move_timer: &mut ResMut<MoveTimer>,
        ) {
            player_info.reset();
            game_state.reset(move_timer, transforms);
            play_log.reset();
        }

        match self {
            GameMode::Practice => {
                reset(player_info, game_state, play_log, transforms, move_timer);
                player_state.replace(PlayerState::Idle).unwrap();
            }
            GameMode::TimeAttack => {
                reset(player_info, game_state, play_log, transforms, move_timer);
                player_state.replace(PlayerState::Idle).unwrap();
            }
            GameMode::MinimalMovement => {
                reset(player_info, game_state, play_log, transforms, move_timer);
                player_state.replace(PlayerState::Idle).unwrap();
            }
            GameMode::DailyPuzzle => {
                reset(player_info, game_state, play_log, transforms, move_timer);
                Network::get_daily_puzzle_date(player_state, network_channel);
            }
        }
    }
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
                SystemSet::on_enter(GameMode::DailyPuzzle)
                    .with_system(spawn_game_mode_ui)
                    .with_system(hide_shuffle_reset_button),
            )
            .add_system_set(
                SystemSet::on_exit(GameMode::DailyPuzzle).with_system(show_shuffle_reset_button),
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
                spawn_image_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(170.0),
                        right: Val::Px(170.0),
                        ..default()
                    },
                    MyButtonType::Undo,
                    asset_server.load("images/button_undo.png").into(),
                    "Undo (Z)".to_string(),
                    font.clone(),
                );

                // redo button
                spawn_image_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(170.0),
                        right: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::Redo,
                    asset_server.load("images/button_redo.png").into(),
                    "Redo (X)".to_string(),
                    font.clone(),
                );
            }
            GameMode::TimeAttack | GameMode::MinimalMovement => {
                // statistics button
                spawn_image_button(
                    parent,
                    UiRect {
                        top: Val::Px(275.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::Statistics,
                    asset_server.load("images/button_statistics.png").into(),
                    "Statistics".to_string(),
                    font.clone(),
                );
            }
            GameMode::DailyPuzzle => {
                // date text
                parent.spawn((
                    TextBundle::from_section(
                        "Date:",
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
                spawn_image_button(
                    parent,
                    UiRect {
                        top: Val::Px(320.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::DateSelection,
                    asset_server.load("images/button_date_selection.png").into(),
                    "Date Selection".to_string(),
                    font.clone(),
                );

                // rankings button
                spawn_image_button(
                    parent,
                    UiRect {
                        top: Val::Px(440.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::Rankings,
                    asset_server.load("images/button_rankings.png").into(),
                    "Rankings".to_string(),
                    font.clone(),
                );

                // restart button
                spawn_image_button(
                    parent,
                    UiRect {
                        right: Val::Px(50.0),
                        bottom: Val::Px(50.0),
                        ..default()
                    },
                    MyButtonType::Restart,
                    asset_server.load("images/button_restart.png").into(),
                    "Restart".to_string(),
                    font.clone(),
                );
            }
        });
}

fn hide_shuffle_reset_button(mut buttons_query: Query<(&mut Visibility, &MyButtonType)>) {
    for (mut visibility, button_type) in buttons_query.iter_mut() {
        match button_type {
            MyButtonType::Shuffle | MyButtonType::Reset => {
                visibility.is_visible = false;
            }
            _ => {}
        }
    }
}

fn show_shuffle_reset_button(mut buttons_query: Query<(&mut Visibility, &MyButtonType)>) {
    for (mut visibility, button_type) in buttons_query.iter_mut() {
        match button_type {
            MyButtonType::Shuffle | MyButtonType::Reset => {
                visibility.is_visible = true;
            }
            _ => {}
        }
    }
}
