use bevy::prelude::*;

use crate::{
    game_ui::{GameUI, BUTTON_HOVER_COLOR, BUTTON_NORMAL_COLOR, BUTTON_PRESS_COLOR, TEXT_SIZE},
    player::PlayerState,
};

const POPUP_BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
struct ModeSelectionPopup;

#[derive(Component, PartialEq, Eq)]
enum ModeButtonType {
    Practice,
    TimeAttack,
    MinimalMovement,
    DailyPuzzle,
}

pub struct ModeSelectionPopupPlugin;

impl Plugin for ModeSelectionPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(PlayerState::ModeSelectionPopup)
                .with_system(spawn_mode_selection_popup),
        )
        .add_system_set(
            SystemSet::on_update(PlayerState::ModeSelectionPopup)
                .with_system(mode_selection_system),
        )
        .add_system_set(
            SystemSet::on_exit(PlayerState::ModeSelectionPopup)
                .with_system(despawn_mode_selection_popup),
        );
    }
}

fn spawn_mode_selection_popup(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            // dark background
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position: UiRect {
                                left: Val::Px(0.0),
                                right: Val::Px(0.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            ..default()
                        },
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                        ..default()
                    },
                    ModeSelectionPopup,
                ))
                .with_children(|parent| {
                    // UI panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Px(600.0), Val::Px(600.0)),
                                ..default()
                            },
                            background_color: POPUP_BACKGROUND_COLOR.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // mode selection text
                            parent.spawn(
                                TextBundle::from_section(
                                    "Mode Selection (Not implemented!)",
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
                                ModeButtonType::Practice,
                            );

                            // time attack mode button
                            spawn_button_and_description(
                                parent,
                                Val::Px(275.0),
                                "Time Attack".to_string(),
                                "Solve as fast\nas you can".to_string(),
                                font.clone(),
                                ModeButtonType::TimeAttack,
                            );

                            // minimal movement mode button
                            spawn_button_and_description(
                                parent,
                                Val::Px(150.0),
                                "Minimal Movement".to_string(),
                                "Solve with\nminimal movement".to_string(),
                                font.clone(),
                                ModeButtonType::MinimalMovement,
                            );

                            // daily puzzle mode button
                            spawn_button_and_description(
                                parent,
                                Val::Px(25.0),
                                "Daily Puzzle".to_string(),
                                "Puzzle for\neveryday life".to_string(),
                                font.clone(),
                                ModeButtonType::DailyPuzzle,
                            );
                        });
                });
        });
}

fn despawn_mode_selection_popup(
    mut commands: Commands,
    popup_query: Query<Entity, With<ModeSelectionPopup>>,
    mut mouse: ResMut<Input<MouseButton>>,
) {
    commands.entity(popup_query.single()).despawn_recursive();
    mouse.reset_all(); // prevent input after state change
}

fn mode_selection_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ModeButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_state: ResMut<State<PlayerState>>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button_type {
                    ModeButtonType::Practice => println!("practice"),
                    ModeButtonType::TimeAttack => println!("time attack"),
                    ModeButtonType::MinimalMovement => println!("minimal movement"),
                    ModeButtonType::DailyPuzzle => println!("daily puzzle"),
                }
                *color = BUTTON_PRESS_COLOR.into();
                let _ = player_state.set(PlayerState::Playing);
            }
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_NORMAL_COLOR.into(),
        }
    }
}

fn spawn_button_and_description(
    parent: &mut ChildBuilder,
    bottom_position: Val,
    button_text: String,
    description_text: String,
    font: Handle<Font>,
    button_type: ModeButtonType,
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
