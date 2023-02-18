use std::collections::HashSet;

use crate::{local_storage::LocalStorage, player::PlayerState, ui::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Debug)]
pub struct GameModeChange(Option<GameMode>);

#[derive(Component)]
pub struct HowToPlayPopup;

#[derive(Serialize, Deserialize, Default)]
pub struct SkipHowToPlay(HashSet<GameMode>);

#[derive(Component)]
struct SkipHowToPlayButton;

pub struct HowToPlayPlugin;

impl Plugin for HowToPlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameModeChange(None))
            .add_system(try_spawn_how_to_play)
            .add_system_set(
                SystemSet::on_enter(GameMode::Practice).with_system(set_game_mode_change),
            )
            .add_system_set(
                SystemSet::on_enter(GameMode::TimeAttack).with_system(set_game_mode_change),
            )
            .add_system_set(
                SystemSet::on_enter(GameMode::MinimalMovement).with_system(set_game_mode_change),
            )
            .add_system_set(
                SystemSet::on_enter(GameMode::DailyPuzzle).with_system(set_game_mode_change),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::HowToPlayPopup).with_system(how_to_play_system),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::HowToPlayPopup).with_system(despawn_how_to_play),
            );
    }
}

fn set_game_mode_change(
    game_mode: Res<State<GameMode>>,
    mut game_mode_change: ResMut<GameModeChange>,
) {
    game_mode_change.0 = Some(*game_mode.current());
}

fn try_spawn_how_to_play(
    mut commands: Commands,
    mut game_mode_change: ResMut<GameModeChange>,
    mut player_state: ResMut<State<PlayerState>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(game_mode) = game_mode_change.0 {
        // skip
        if let Some(skip_how_to_play) = LocalStorage::get_skip_how_to_play() {
            if skip_how_to_play.0.contains(&game_mode) {
                return;
            }
        }

        // skip if state is PlayerState::ResponseWaiting
        if !player_state.inactives().is_empty() {
            return;
        }

        if player_state.push(PlayerState::HowToPlayPopup).is_ok() {
            game_mode_change.0 = None;
            info!("how_to_play popup");

            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(0.0),
                                right: Val::Px(0.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            ..default()
                        },
                        image: asset_server
                            .load(format!(
                                "images/how_to_play_{}.png",
                                match game_mode {
                                    GameMode::Practice => "practice",
                                    GameMode::TimeAttack | GameMode::MinimalMovement =>
                                        "time_attack",
                                    GameMode::DailyPuzzle => "daily_puzzle",
                                }
                            ))
                            .into(),
                        z_index: ZIndex::Global(1),
                        ..default()
                    },
                    HowToPlayPopup,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            image: asset_server.load("images/button_toggle_off.png").into(),
                            style: Style {
                                size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    right: Val::Px(490.0),
                                    bottom: Val::Px(20.0),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                        SkipHowToPlayButton,
                    ));
                });
        }
    }
}

fn how_to_play_system(
    popup_close_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<HowToPlayPopup>),
    >,
    mut skip_button_interaction: Query<
        (&Interaction, &mut BackgroundColor, &mut UiImage),
        (
            Changed<Interaction>,
            With<Button>,
            With<SkipHowToPlayButton>,
        ),
    >,
    mut player_state: ResMut<State<PlayerState>>,
    game_mode: Res<State<GameMode>>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color, mut image) in skip_button_interaction.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let game_mode = game_mode.current();
                let mut skip_how_to_play = LocalStorage::get_skip_how_to_play().unwrap_or_default();
                if skip_how_to_play.0.contains(game_mode) {
                    skip_how_to_play.0.remove(game_mode);
                    image.0 = asset_server.load("images/button_toggle_off.png");
                } else {
                    skip_how_to_play.0.insert(*game_mode);
                    image.0 = asset_server.load("images/button_toggle_on.png");
                }
                LocalStorage::set_skip_how_to_play(&skip_how_to_play);
                *color = (BUTTON_WHITE * BUTTON_PRESS_MUL).into();
            }
            Interaction::Hovered => *color = (BUTTON_WHITE * BUTTON_HOVER_MUL).into(),
            Interaction::None => *color = BUTTON_WHITE.into(),
        }
    }
    for interaction in popup_close_interaction.iter() {
        if *interaction == Interaction::Clicked {
            assert_eq!(*player_state.current(), PlayerState::HowToPlayPopup);
            player_state.pop().unwrap();
        }
    }
}

fn despawn_how_to_play(
    mut commands: Commands,
    how_to_play_query: Query<Entity, With<HowToPlayPopup>>,
    mut mouse: ResMut<Input<MouseButton>>,
) {
    commands
        .entity(how_to_play_query.single())
        .despawn_recursive();
    mouse.reset_all(); // prevent input after state change
}
