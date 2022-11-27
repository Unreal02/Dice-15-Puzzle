use bevy::prelude::*;

use crate::{
    buffered_input::{InputInversionFlag, MoveImmediate},
    game::{GameState, MoveTimer},
    player::{PlayerInfo, PlayerState},
};

const TEXT_SIZE: f32 = 40.0;

const NORMAL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const PRESSED_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Component, PartialEq, Eq)]
enum MyButtons {
    Reset,
    Shuffle,
    AnimationToggle,
    InputInversion,
}

#[derive(Component)]
struct AnimationToggleButton;

#[derive(Component)]
struct InputInversionButton;

#[derive(Component)]
struct PlayerInfoUI;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_buttons)
            .add_system_set(SystemSet::on_update(PlayerState::Playing).with_system(button_system))
            .add_system_set(SystemSet::on_enter(PlayerState::GameClear).with_system(init_clear_ui))
            .add_system_set(
                SystemSet::on_update(PlayerState::GameClear).with_system(clear_ui_stystem),
            );
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MyButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<MoveTimer>,
    mut game_query: Query<&mut GameState>,
    mut input_system: Query<(&mut InputInversionFlag, &mut MoveImmediate)>,
    mut player_info: Query<&mut PlayerInfo>,
    mut text_set: ParamSet<(
        Query<(&mut Text, &mut AnimationToggleButton)>,
        Query<(&mut Text, &mut InputInversionButton)>,
        Query<(&mut Text, &PlayerInfoUI)>,
    )>,
) {
    let mut game = game_query.single_mut();

    // button interactions
    for (interaction, mut color, buttons) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match buttons {
                    MyButtons::Reset => {
                        game.reset(&mut move_timer, &mut transforms);
                        player_info.single_mut().reset();
                        game.is_shuffled = false;
                    }
                    MyButtons::Shuffle => {
                        game.shuffle(&mut move_timer, &mut transforms);
                        player_info.single_mut().start_tracking();
                        game.is_shuffled = true;
                    }
                    MyButtons::AnimationToggle => {
                        let (_, mut move_immediate) = input_system.single_mut();
                        match move_immediate.0 {
                            true => move_immediate.0 = false,
                            false => move_immediate.0 = true,
                        }
                        text_set.p0().single_mut().0.sections[0].value = match move_immediate.0 {
                            true => "Animation\nOff".to_string(),
                            false => "Animation\nOn".to_string(),
                        };
                    }
                    MyButtons::InputInversion => {
                        let (mut input_reveresion_flag, _) = input_system.single_mut();
                        match input_reveresion_flag.0 {
                            true => input_reveresion_flag.0 = false,
                            false => input_reveresion_flag.0 = true,
                        }
                        text_set.p1().single_mut().0.sections[0].value =
                            match input_reveresion_flag.0 {
                                true => "Input\nInverse".to_string(),
                                false => "Input\nNormal".to_string(),
                            };
                    }
                }
                *color = PRESSED_COLOR.into();
            }
            _ => *color = NORMAL_COLOR.into(),
        }
    }

    // player info
    let play_time = player_info.single().get_play_timer();
    let move_count = player_info.single().get_move_count();
    text_set.p2().single_mut().0.sections[0].value = format!(
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
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            Name::new("GAME_UI"),
        ))
        .with_children(|parent| {
            // reset button
            add_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                "Reset".to_string(),
                font.clone(),
                MyButtons::Reset,
            );

            // shuffle button
            add_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    bottom: Val::Px(175.0),
                    ..default()
                },
                "Shuffle".to_string(),
                font.clone(),
                MyButtons::Shuffle,
            );

            // animation toggle button
            add_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                "Animation\nOn".to_string(),
                font.clone(),
                MyButtons::AnimationToggle,
            );

            // input inversion button
            add_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(175.0),
                    ..default()
                },
                "Input\nNormal".to_string(),
                font.clone(),
                MyButtons::InputInversion,
            );

            // play timer
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
                    position: UiRect {
                        left: Val::Px(50.0),
                        top: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                }),
                PlayerInfoUI,
            ));
        });
}

fn init_clear_ui() {
    println!("GAME CLEAR")
}

fn clear_ui_stystem() {
    println!("CLEAR UI")
}

fn add_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: MyButtons,
) -> impl Bundle {
    parent
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let mut entity_commands = parent.spawn(
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
            match button_type {
                MyButtons::AnimationToggle => {
                    entity_commands.insert(AnimationToggleButton);
                }
                MyButtons::InputInversion => {
                    entity_commands.insert(InputInversionButton);
                }
                _ => (),
            }
        })
        .insert(button_type);
}
