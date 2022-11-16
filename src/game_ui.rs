use bevy::prelude::*;

use crate::{
    game::{GameState, MyTimer},
    player::PlayerState,
};

const NORMAL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const PRESSED_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Component)]
enum MyButtons {
    Reset,
    Shuffle,
}

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
    mut move_timer: ResMut<MyTimer>,
    mut game_query: Query<&mut GameState>,
) {
    let mut game = game_query.single_mut();
    for (interaction, mut color, buttons) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match buttons {
                    MyButtons::Reset => {
                        game.reset(&mut move_timer, &mut transforms);
                        // game.is_shuffled = true; // uncomment it to develop game clear part
                    }
                    MyButtons::Shuffle => {
                        game.shuffle(&mut move_timer, &mut transforms);
                        game.is_shuffled = false;
                    }
                }
                *color = PRESSED_COLOR.into();
            }
            _ => *color = NORMAL_COLOR.into(),
        }
    }
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
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(50.0),
                            bottom: Val::Px(50.0),
                            ..default()
                        },
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Reset",
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MyButtons::Reset);

            // shuffle button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(50.0),
                            bottom: Val::Px(200.0),
                            ..default()
                        },
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Shuffle",
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MyButtons::Shuffle);
        });
}

fn init_clear_ui() {
    println!("GAME CLEAR")
}

fn clear_ui_stystem() {
    println!("CLEAR UI")
}
