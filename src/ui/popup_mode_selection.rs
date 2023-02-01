use crate::{
    game::{GameState, MoveTimer},
    network::NetworkChannel,
    player::{PlayLog, PlayerInfo, PlayerState},
    ui::*,
};

pub fn spawn_popup_mode_selection(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_image = UiImage::from(asset_server.load("images/button.png"));

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, font.clone(), button_image.clone(), |parent| {
                // mode selection text
                parent.spawn(
                    TextBundle::from_section(
                        "Mode Selection",
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

                // practice mode button
                spawn_button_and_description(
                    parent,
                    Val::Px(400.0),
                    "Practice".to_string(),
                    "Practice with\nundo and snapshots".to_string(),
                    font.clone(),
                    GameMode::Practice,
                    button_image.clone(),
                );

                // time attack mode button
                spawn_button_and_description(
                    parent,
                    Val::Px(275.0),
                    "Time Attack".to_string(),
                    "Solve as fast\nas you can".to_string(),
                    font.clone(),
                    GameMode::TimeAttack,
                    button_image.clone(),
                );

                // minimal movement mode button
                spawn_button_and_description(
                    parent,
                    Val::Px(150.0),
                    "Minimal Movement".to_string(),
                    "Solve with\nminimal movement".to_string(),
                    font.clone(),
                    GameMode::MinimalMovement,
                    button_image.clone(),
                );

                // daily puzzle mode button
                spawn_button_and_description(
                    parent,
                    Val::Px(25.0),
                    "Daily Puzzle".to_string(),
                    "Puzzle for\neveryday life".to_string(),
                    font.clone(),
                    GameMode::DailyPuzzle,
                    button_image.clone(),
                );
            });
        });
}

pub fn popup_system_mode_selection(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameMode),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_state: ResMut<State<PlayerState>>,
    mut player_info: Query<&mut PlayerInfo>,
    mut game_mode: ResMut<State<GameMode>>,
    mut play_log: Query<&mut PlayLog>,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<MoveTimer>,
    mut game_query: Query<&mut GameState>,
    mut network_channel: Res<NetworkChannel>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if button_type != game_mode.current() {
                    let _ = game_mode.set(*button_type);
                    button_type.entry_handler(
                        &mut player_info.single_mut(),
                        &mut game_query.single_mut(),
                        &mut play_log.single_mut(),
                        &mut transforms,
                        &mut move_timer,
                        &mut player_state,
                        &mut network_channel,
                    );
                } else {
                    player_state.pop().unwrap();
                }
                *color = (BUTTON_WHITE * BUTTON_PRESS_MUL).into();
            }
            Interaction::Hovered => *color = (BUTTON_WHITE * BUTTON_HOVER_MUL).into(),
            Interaction::None => *color = BUTTON_WHITE.into(),
        }
    }
}

fn spawn_button_and_description(
    parent: &mut ChildBuilder,
    bottom_position: Val,
    button_text: String,
    description_text: String,
    font: Handle<Font>,
    button_type: GameMode,
    image: UiImage,
) {
    // button
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(25.0),
                        bottom: bottom_position,
                        ..default()
                    },
                    ..default()
                },
                image,
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            // button text
            parent.spawn(
                TextBundle::from_section(
                    button_text,
                    TextStyle {
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::BLACK,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    max_size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                    ..default()
                }),
            );
        });

    // description text
    parent.spawn(
        TextBundle::from_section(
            description_text,
            TextStyle {
                font,
                font_size: TEXT_SIZE,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::CENTER_LEFT)
        .with_style(Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(250.0),
                bottom: bottom_position.try_sub(Val::Px(8.0)).unwrap(),
                ..default()
            },
            size: Size::new(Val::Px(325.0), Val::Px(100.0)),
            ..default()
        }),
    );
}
