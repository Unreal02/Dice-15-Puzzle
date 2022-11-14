use bevy::prelude::*;

use crate::{game::GameState, player::PlayerState};

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
            .add_system_set(SystemSet::on_update(PlayerState::Playing).with_system(button_system));
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MyButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<Timer>,
    mut game_query: Query<&mut GameState>,
) {
    let mut game = game_query.single_mut();
    for (interaction, mut color, buttons) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match buttons {
                    MyButtons::Reset => game.reset(&mut move_timer, &mut transforms),
                    MyButtons::Shuffle => game.shuffle(&mut move_timer, &mut transforms),
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
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert_bundle(SpatialBundle::default())
        .insert(Name::new("GAME_UI"))
        .with_children(|parent| {
            // reset button
            parent
                .spawn_bundle(ButtonBundle {
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
                    color: NORMAL_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
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
                .spawn_bundle(ButtonBundle {
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
                    color: NORMAL_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
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
