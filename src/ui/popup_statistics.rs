use crate::{statistics_manager::StatisticsManager, ui::*};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

const SCROLL_BAR_MAX_ITEMS: usize = 11;
const SCROLL_SPEED: f32 = 1.0;

#[derive(Component, Default)]
pub struct ScrollBar {
    position: f32,
}

pub fn spawn_popup_statistics(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    statistics_manager_query: Query<&StatisticsManager>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_image = UiImage::from(asset_server.load("images/button.png"));
    let statistics_manager = statistics_manager_query.single();

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, font.clone(), button_image.clone(), |parent| {
                // statistics text
                parent.spawn(
                    TextBundle::from_section(
                        "Statistics",
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

                // number of solves
                spawn_text(
                    parent,
                    UiRect {
                        top: Val::Px(80.0),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    format!("Solves\n{}", statistics_manager.solves()),
                    font.clone(),
                );

                // export button
                spawn_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(25.0),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    "Export".to_string(),
                    font.clone(),
                    MyButtonType::Export,
                    None,
                    button_image,
                );

                if statistics_manager.solves() != 0 {
                    // average
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(180.0),
                            left: Val::Px(25.0),
                            ..default()
                        },
                        format!("Average\n{}", statistics_manager.average()),
                        font.clone(),
                    );

                    // best
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(280.0),
                            left: Val::Px(25.0),
                            ..default()
                        },
                        format!("Best\n{}", statistics_manager.best()),
                        font.clone(),
                    );

                    // worst
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(380.0),
                            left: Val::Px(25.0),
                            ..default()
                        },
                        format!("Worst\n{}", statistics_manager.worst()),
                        font.clone(),
                    );

                    // details (text)
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(80.0),
                            left: Val::Px(325.0),
                            ..default()
                        },
                        "Details".to_string(),
                        font.clone(),
                    );

                    // scroll bar background
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    size: Size::new(Val::Px(275.0), Val::Px(450.0)),
                                    position: UiRect {
                                        right: Val::Px(25.0),
                                        bottom: Val::Px(25.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                                ..default()
                            },
                            ScrollBar::default(),
                        ))
                        .with_children(|parent| {
                            for i in 0..SCROLL_BAR_MAX_ITEMS {
                                spawn_text(
                                    parent,
                                    UiRect {
                                        top: Val::Px(5.0 + 40.0 * i as f32),
                                        left: Val::Px(25.0),
                                        ..default()
                                    },
                                    "".to_string(),
                                    font.clone(),
                                );
                            }
                        });
                }
            });
        });
}

pub fn popup_system_statistics(
    mut scroll_events: EventReader<MouseWheel>,
    mut scroll_bar_query: Query<(&mut ScrollBar, &Children)>,
    statistics_manager_query: Query<&StatisticsManager>,
    mut text_query: Query<&mut Text>,
) {
    let statistics_manager = statistics_manager_query.single();

    // scroll bar
    if let Ok((mut scroll_bar, children)) = scroll_bar_query.get_single_mut() {
        for scroll_event in scroll_events.iter() {
            let dy = match scroll_event.unit {
                MouseScrollUnit::Line => scroll_event.y,
                MouseScrollUnit::Pixel => scroll_event.y / 20.0,
            };
            scroll_bar.position -= dy * SCROLL_SPEED;
        }

        scroll_bar.position = scroll_bar
            .position
            .clamp(0.0, (statistics_manager.solves() as f32 - 11.0).max(0.0));

        let start_position = scroll_bar.position as usize;
        for i in 0..children.len() {
            let index = start_position + i;
            text_query.get_mut(children[i]).unwrap().sections[0].value =
                if index < statistics_manager.solves() {
                    format!("{}. {}", index + 1, statistics_manager.get_record(i))
                } else {
                    "".to_string()
                };
        }
    }
}

fn spawn_text(parent: &mut ChildBuilder, position: UiRect, text: String, font: Handle<Font>) {
    parent.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: TEXT_SIZE,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position,
            ..default()
        }),
    );
}
