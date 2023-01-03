use bevy::prelude::*;

use crate::{
    board_string::board_to_string,
    buffered_input::{InputInversionFlag, MoveImmediate},
    game::{GameState, MoveTimer},
    game_mode_ui::GameMode,
    player::{PlayerInfo, PlayerState},
};

pub const TEXT_SIZE: f32 = 40.0;

pub const BUTTON_NORMAL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const BUTTON_HOVER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const BUTTON_PRESS_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component, PartialEq, Eq)]
enum MyButtonType {
    Reset,
    Shuffle,
    AnimationToggle,
    InputInversion,
    ModeSelection,
    Share,
}

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
struct PlayerInfoUI;

#[derive(Component)]
struct GameClearUI;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_buttons)
            .add_system_set(SystemSet::on_update(PlayerState::Playing).with_system(button_system))
            .add_system_set(SystemSet::on_enter(PlayerState::GameClear).with_system(spawn_clear_ui))
            .add_system_set(
                SystemSet::on_update(PlayerState::GameClear)
                    .with_system(clear_ui_stystem)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::GameClear).with_system(despawn_clear_ui),
            );
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MyButtonType, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<MoveTimer>,
    mut game_query: Query<&mut GameState>,
    mut input_system: Query<(&mut InputInversionFlag, &mut MoveImmediate)>,
    mut player_info: Query<&mut PlayerInfo>,
    mut text_query: Query<&mut Text, Without<PlayerInfoUI>>,
    mut player_info_text_query: Query<&mut Text, With<PlayerInfoUI>>,
    mut player_state: ResMut<State<PlayerState>>,
    game_mode: Res<State<GameMode>>,
) {
    let mut game = game_query.single_mut();

    // button interactions
    for (interaction, mut color, button_type, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                match button_type {
                    MyButtonType::Reset => {
                        game.reset(&mut move_timer, &mut transforms);
                        player_info.single_mut().reset();
                        game.is_shuffled = false;
                        if *player_state.current() == PlayerState::GameClear {
                            let _ = player_state.set(PlayerState::Playing);
                        }
                    }
                    MyButtonType::Shuffle => {
                        game.shuffle(&mut move_timer, &mut transforms);
                        player_info.single_mut().start_tracking();
                        game.is_shuffled = true;
                        if *player_state.current() == PlayerState::GameClear {
                            let _ = player_state.set(PlayerState::Playing);
                        }
                    }
                    MyButtonType::AnimationToggle => {
                        let (_, mut move_immediate) = input_system.single_mut();
                        match move_immediate.0 {
                            true => move_immediate.0 = false,
                            false => move_immediate.0 = true,
                        }
                        text.sections[0].value = match move_immediate.0 {
                            true => "Animation\nOff".to_string(),
                            false => "Animation\nOn".to_string(),
                        };
                    }
                    MyButtonType::InputInversion => {
                        let (mut input_reveresion_flag, _) = input_system.single_mut();
                        match input_reveresion_flag.0 {
                            true => input_reveresion_flag.0 = false,
                            false => input_reveresion_flag.0 = true,
                        }
                        text.sections[0].value = match input_reveresion_flag.0 {
                            true => "Input\nInverse".to_string(),
                            false => "Input\nNormal".to_string(),
                        };
                    }
                    MyButtonType::ModeSelection => {
                        let _ = player_state.set(PlayerState::ModeSelectionPopup);
                    }
                    MyButtonType::Share => {
                        let board_string = board_to_string(&transforms, &mut game);
                        println!("{:?}", board_string);
                    }
                }
                *color = BUTTON_PRESS_COLOR.into();
            }
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_NORMAL_COLOR.into(),
        }
        if *button_type == MyButtonType::ModeSelection {
            text.sections[0].value = "Mode (WIP)\n".to_string()
                + match game_mode.current() {
                    GameMode::Practice => "Practice",
                    GameMode::TimeAttack => "Time Attack",
                    GameMode::MinimalMovement => "Min Move",
                    GameMode::DailyPuzzle => "Daily Puzzle",
                };
        }
    }

    // player info
    let (play_time, move_count) = player_info.single().get_player_info();
    player_info_text_query.single_mut().sections[0].value = format!(
        "Time: {:02}:{:02}.{:02}\nMoves: {}",
        play_time.as_secs() / 60,
        play_time.as_secs() % 60,
        play_time.subsec_millis() / 10,
        move_count
    );
}

fn setup_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("GAME_UI"),
            GameUI,
        ))
        .with_children(|parent| {
            // reset button
            spawn_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                "Reset".to_string(),
                font.clone(),
                MyButtonType::Reset,
            );

            // shuffle button
            spawn_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    bottom: Val::Px(175.0),
                    ..default()
                },
                "Shuffle".to_string(),
                font.clone(),
                MyButtonType::Shuffle,
            );

            // animation toggle button
            spawn_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                "Animation\nOn".to_string(),
                font.clone(),
                MyButtonType::AnimationToggle,
            );

            // input inversion button
            spawn_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(175.0),
                    ..default()
                },
                "Input\nNormal".to_string(),
                font.clone(),
                MyButtonType::InputInversion,
            );

            // mode selection button
            spawn_button(
                parent,
                UiRect {
                    left: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                "Mode (WIP)\nPractice".to_string(),
                font.clone(),
                MyButtonType::ModeSelection,
            );

            // share button
            spawn_button(
                parent,
                UiRect {
                    left: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                "Share\n(WIP)".to_string(),
                font.clone(),
                MyButtonType::Share,
            );

            // player info
            parent.spawn((
                TextBundle::from_section(
                    "Time: 00:00.00\nMoves: 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(50.0),
                        top: Val::Px(175.0),
                        ..default()
                    },
                    ..default()
                }),
                PlayerInfoUI,
            ));
        });
}

fn spawn_clear_ui(
    mut commands: Commands,
    game_ui: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");

    commands.entity(game_ui.single()).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Clear!!",
                TextStyle {
                    font: font.clone(),
                    font_size: TEXT_SIZE,
                    color: Color::BLACK,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(50.0),
                    top: Val::Px(275.0),
                    ..default()
                },
                ..default()
            }),
            GameClearUI,
        ));
    });
}

fn clear_ui_stystem() {
    // println!("CLEAR UI")
}

fn despawn_clear_ui(mut commands: Commands, clear_ui: Query<Entity, With<GameClearUI>>) {
    commands.entity(clear_ui.single()).despawn_recursive();
}

fn spawn_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: MyButtonType,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position,
                    ..default()
                },
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
