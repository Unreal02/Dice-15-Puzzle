use crate::{
    buffered_input::{InputBuffer, InputHandler, InputInversionFlag, InputTimer, MoveImmediate},
    game::{GameState, MoveTimer},
    player::{PlayLog, PlayerInfo, PlayerState},
    statistics_manager::StatisticsManager,
    ui::*,
    utils::*,
};

pub fn game_ui_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&mut UiImage>,
            &MyButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<MoveTimer>,
    mut game_query: Query<&mut GameState>,
    mut input_system: Query<(
        &mut InputBuffer,
        &mut InputInversionFlag,
        &mut MoveImmediate,
    )>,
    player_info: Query<&mut PlayerInfo>,
    mut play_log: Query<&mut PlayLog>,
    mut text_query: Query<(&mut Text, &MyTextType)>,
    mut player_state: ResMut<State<PlayerState>>,
    mut input_timer: ResMut<InputTimer>,
    game_mode: Res<State<GameMode>>,
    statistics_manager_query: Query<&StatisticsManager>,
    asset_server: Res<AssetServer>,
) {
    let mut game = game_query.single_mut();
    let button_toggle_on_image = asset_server.load("images/button_toggle_on.png");
    let button_toggle_off_image = asset_server.load("images/button_toggle_off.png");

    let (mut input_buffer, mut input_reveresion_flag, mut move_immediate) =
        input_system.single_mut();

    // button interactions
    for (interaction, mut color, ui_image, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button_type {
                    MyButtonType::Reset => {
                        game.reset(&mut move_timer, &mut transforms);
                        play_log.single_mut().reset();
                        game.is_shuffled = false;
                        if *player_state.current() != PlayerState::Idle {
                            let _ = player_state.set(PlayerState::Idle);
                        }
                    }
                    MyButtonType::Shuffle => {
                        game.shuffle(&mut transforms);
                        play_log.single_mut().reset();
                        game.is_shuffled = true;
                        if *player_state.current() != PlayerState::Shuffled {
                            let _ = player_state.set(PlayerState::Shuffled);
                        }
                    }
                    MyButtonType::AnimationToggle => match move_immediate.0 {
                        true => {
                            move_immediate.0 = false;
                            ui_image.unwrap().0 = button_toggle_on_image.clone();
                        }
                        false => {
                            move_immediate.0 = true;
                            ui_image.unwrap().0 = button_toggle_off_image.clone();
                        }
                    },
                    MyButtonType::InputInversion => {
                        play_log.single_mut().reset();
                        match input_reveresion_flag.0 {
                            true => {
                                input_reveresion_flag.0 = false;
                                ui_image.unwrap().0 = button_toggle_off_image.clone();
                            }
                            false => {
                                input_reveresion_flag.0 = true;
                                ui_image.unwrap().0 = button_toggle_on_image.clone();
                            }
                        }
                    }
                    MyButtonType::ModeSelection => {
                        let _ = player_state.push(PlayerState::ModeSelectionPopup);
                    }
                    MyButtonType::Share => {
                        let board_string = board_to_string(&transforms, &mut game);
                        info!("board string: {:?}\n", board_string);
                        string_to_board(board_string, &mut transforms, &mut game);
                    }
                    MyButtonType::Undo => InputHandler::undo(
                        input_reveresion_flag.0,
                        &mut input_buffer,
                        &mut play_log.single_mut(),
                        &mut input_timer,
                    ),
                    MyButtonType::Redo => InputHandler::redo(
                        input_reveresion_flag.0,
                        &mut input_buffer,
                        &mut play_log.single_mut(),
                        &mut input_timer,
                    ),
                    MyButtonType::Statistics => {
                        let _ = player_state.push(PlayerState::StatisticsPopup);
                    }
                    MyButtonType::DateSelection => {
                        let _ = player_state.push(PlayerState::DateSelectionPopup);
                    }
                    MyButtonType::Rankings => {
                        info!("rankings\n");
                    }
                    MyButtonType::Export => {
                        let statistics_manager = statistics_manager_query.single();
                        statistics_manager.export();
                    }
                }
                *color = BUTTON_PRESS_COLOR.into();
            }
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_NORMAL_COLOR.into(),
        }
    }

    // text
    for (mut text, &ui_type) in &mut text_query {
        match ui_type {
            MyTextType::ModeSelection => {
                text.sections[0].value = "Mode (WIP)\n".to_string()
                    + match game_mode.current() {
                        GameMode::Practice => "Practice",
                        GameMode::TimeAttack => "Time Attack",
                        GameMode::MinimalMovement => "Min Move",
                        GameMode::DailyPuzzle => "Daily Puzzle",
                    };
            }
            MyTextType::PlayerInfo => {
                let (play_time, move_count) = player_info.single().get_player_info();
                text.sections[0].value = format!(
                    "Time: {}\nMoves: {}",
                    duration_to_string(play_time),
                    move_count
                );
            }
            MyTextType::GameClear => {}
            MyTextType::Date => {}
        }
    }
}
