use std::time::Duration;

use chrono::Datelike;

use crate::{
    buffered_input::{InputBuffer, InputHandler, InputInversionFlag, InputTimer, MoveImmediate},
    daily_puzzle_info::DailyPuzzleInfo,
    game::{EasyMode, GameState, MoveTimer},
    local_storage::LocalStorage,
    network::NetworkChannel,
    player::{PlayLog, PlayerInfo, PlayerState},
    statistics_manager::StatisticsManager,
    ui::*,
    utils::*,
};

pub fn game_ui_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&mut UiImage>,
            &MyButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<(&Text, &MyTextType)>,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<MoveTimer>,
    mut game_query: Query<&mut GameState>,
    mut input_system: Query<(
        &mut InputBuffer,
        &mut InputInversionFlag,
        &mut MoveImmediate,
    )>,
    mut play_log: Query<&mut PlayLog>,
    mut player_state: ResMut<State<PlayerState>>,
    mut input_timer: ResMut<InputTimer>,
    mut statistics_manager_query: Query<&mut StatisticsManager>,
    asset_server: Res<AssetServer>,
    daily_puzzle_info_query: Query<&DailyPuzzleInfo>,
    network_channel: Res<NetworkChannel>,
    mut delete_statistics_event: EventWriter<DeleteStatisticsEvent>,
    player_info_query: Query<&PlayerInfo>,
) {
    let mut game = game_query.single_mut();
    let daily_puzzle_info = daily_puzzle_info_query.single();

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
                    MyButtonType::AnimationToggle => {
                        match move_immediate.0 {
                            true => {
                                move_immediate.0 = false;
                                ui_image.unwrap().0 =
                                    asset_server.load("images/button_toggle_on.png");
                            }
                            false => {
                                move_immediate.0 = true;
                                ui_image.unwrap().0 =
                                    asset_server.load("images/button_toggle_off.png");
                            }
                        }
                        LocalStorage::set_move_immediate(&move_immediate.0);
                    }
                    MyButtonType::InputInversion => {
                        play_log.single_mut().reset();
                        match input_reveresion_flag.0 {
                            true => {
                                input_reveresion_flag.0 = false;
                                ui_image.unwrap().0 =
                                    asset_server.load("images/button_toggle_off.png");
                            }
                            false => {
                                input_reveresion_flag.0 = true;
                                ui_image.unwrap().0 =
                                    asset_server.load("images/button_toggle_on.png");
                            }
                        }
                        LocalStorage::set_input_inversion(&input_reveresion_flag.0);
                    }
                    MyButtonType::ModeSelection => {
                        let _ = player_state.push(PlayerState::ModeSelectionPopup);
                    }
                    MyButtonType::Settings => {
                        let _ = player_state.push(PlayerState::SettingsPopup);
                    }
                    MyButtonType::Difficulty => {
                        let _ = player_state.push(PlayerState::DifficultyPopup);
                    }
                    MyButtonType::Share => {
                        let board_string = board_to_string(&transforms, &mut game);
                        let puzzle_key = board_string.into_key();
                        crate::network::Network::enroll_puzzle_state(
                            puzzle_key,
                            board_string,
                            &mut player_state,
                            &network_channel,
                        );
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
                        let _ = player_state.push(PlayerState::RankingsPopup);
                    }
                    MyButtonType::PopupEnrollScore => {
                        let _ = player_state.push(PlayerState::EnrollScorePopup);
                    }
                    MyButtonType::EnrollScore => {
                        for (text, &text_type) in text_query.iter() {
                            if let MyTextType::TextInputBox(_) = text_type {
                                let user_name = text.sections[0].value.clone();
                                if !user_name.is_empty() {
                                    let player_info = player_info_query.single().get_player_info();
                                    info!("Enroll Score: {}", user_name);
                                    let daily_puzzle_info = daily_puzzle_info_query.single();
                                    crate::network::Network::enroll_daily_ranking(
                                        daily_puzzle_info.current_date,
                                        user_name,
                                        player_info.0,
                                        player_info.1,
                                        &mut player_state,
                                        &network_channel,
                                    );
                                }
                            }
                        }
                    }
                    MyButtonType::Restart => {
                        let _ = daily_puzzle_info.load_daily_puzzle(
                            daily_puzzle_info.current_date,
                            &mut transforms,
                            &mut game,
                            &mut player_state,
                            &network_channel,
                        );
                    }
                    MyButtonType::Export => {
                        let statistics_manager = statistics_manager_query.single();
                        statistics_manager.export();
                    }
                    MyButtonType::LoadURL => {
                        for (text, &text_type) in text_query.iter() {
                            if let MyTextType::TextInputBox(_) = text_type {
                                let url_key = &text.sections[0].value;
                                info!("Load URL: {}", url_key);
                                crate::network::Network::get_puzzle_state(
                                    url_key.to_string(),
                                    &mut player_state,
                                    &network_channel,
                                );
                            }
                        }
                    }
                    MyButtonType::DeleteStatistics => {
                        let mut statistics_manager = statistics_manager_query.single_mut();
                        statistics_manager.delete_statistics();
                        delete_statistics_event.send_default();
                    }
                }
                *color = (BUTTON_WHITE * BUTTON_PRESS_MUL).into();
            }
            Interaction::Hovered => *color = (BUTTON_WHITE * BUTTON_HOVER_MUL).into(),
            Interaction::None => *color = BUTTON_WHITE.into(),
        }
    }
}

pub fn game_ui_text_system(
    player_info: Query<&mut PlayerInfo>,
    mut text_query: Query<(&mut Text, &MyTextType)>,
    game_mode: Res<State<GameMode>>,
    daily_puzzle_info_query: Query<&DailyPuzzleInfo>,
    mut char_evr: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let daily_puzzle_info = daily_puzzle_info_query.single();

    // text
    for (mut text, &ui_type) in &mut text_query {
        match ui_type {
            MyTextType::ModeSelection => {
                text.sections[0].value = "Mode\n".to_string()
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
            MyTextType::Date => {
                let current_date = daily_puzzle_info.current_date;
                text.sections[0].value = format!(
                    "Date: {}. {}. {}.",
                    current_date.year(),
                    current_date.month(),
                    current_date.day()
                );
            }
            MyTextType::TextInputBox(max_len) => {
                if keyboard_input.just_pressed(KeyCode::Back) {
                    text.sections[0].value.pop();
                }
                for ev in char_evr.iter() {
                    if text.sections[0].value.len() < max_len {
                        text.sections[0].value.push(ev.char);
                    }
                }
            }
            MyTextType::Difficulty => {
                let size = LocalStorage::get_board_size().unwrap_or_default().0;
                let easy_mode = LocalStorage::get_easy_mode().unwrap_or(EasyMode(false)).0;
                text.sections[0].value = format!(
                    "{} x {}\n{}",
                    size,
                    size,
                    if easy_mode { "Easy" } else { "Hard" }
                );
            }
            _ => {}
        }
    }
}

pub fn button_hover_system(
    mut interaction_query: Query<
        (
            &Interaction,
            ChangeTrackers<Interaction>,
            &mut InteractionHistory,
            &mut HoverTimer,
            &mut Children,
        ),
        With<Button>,
    >,
    time: Res<Time>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (interaction, tracker, mut interaction_history, mut hover_timer, children) in
        interaction_query.iter_mut()
    {
        assert_eq!(children.len(), 1);
        let mut info_visibility = visibility_query.get_mut(children[0]).unwrap();

        if tracker.is_changed() {
            interaction_history.prev = interaction_history.curr;
            interaction_history.curr = *interaction;
            info_visibility.is_visible = false;
        }

        if interaction_history.prev == Interaction::None && *interaction == Interaction::Hovered {
            if tracker.is_changed() {
                hover_timer.0.reset();
                hover_timer.0.set_duration(Duration::from_secs_f32(0.7));
            }

            hover_timer.0.tick(time.delta());
            if hover_timer.0.just_finished() {
                info_visibility.is_visible = true;
            }
        }
    }
}
