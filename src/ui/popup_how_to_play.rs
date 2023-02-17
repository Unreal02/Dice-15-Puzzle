use std::collections::HashSet;

use crate::{local_storage::LocalStorage, player::PlayerState, ui::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Debug)]
pub struct GameModeChange(Option<GameMode>);

#[derive(Component)]
pub struct HowToPlayPopup;

#[derive(Serialize, Deserialize)]
pub struct SkipHowToPlay(HashSet<GameMode>);

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
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
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
                        z_index: ZIndex::Global(1),
                        ..default()
                    },
                    HowToPlayPopup,
                ))
                .with_children(|parent| {
                    // all:
                    // 조작법(방향키) 설명
                    // mode selection button
                    // player info text
                    // settings button

                    // practice:
                    // undo(Z) button
                    // redo(X) button

                    // time attack, minimal movement mode:
                    // statitsis popup

                    // not daily puzzle:
                    // shuffle button
                    // reset button

                    // daily puzzle:
                    // current date text
                    // date selection popup
                    // rankings popup
                    // restart button
                });
        }
    }
}

fn how_to_play_system(
    button_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<HowToPlayPopup>),
    >,
    mut player_state: ResMut<State<PlayerState>>,
) {
    for interaction in button_interaction.iter() {
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
