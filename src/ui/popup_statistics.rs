use crate::{statistics_manager::StatisticsManager, ui::*};

const SCROLL_BAR_MAX_ITEMS: usize = 11;

#[derive(Default)]
pub struct DeleteStatisticsEvent;

#[derive(Component, Default)]
pub struct ScrollBar {
    pub content: Vec<String>,
    pub max_items: usize,
    pub position: f32,
}

#[derive(Component, Debug)]
pub enum PopupStatisticsTextType {
    Solves,
    Average,
    Best,
    Worst,
    Details,
}

pub fn spawn_popup_statistics(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    statistics_manager_query: Query<&StatisticsManager>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_close_image = UiImage::from(asset_server.load("images/button_close.png"));
    let statistics_manager = statistics_manager_query.single();

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, button_close_image.clone(), font.clone(), |parent| {
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
                    Some(PopupStatisticsTextType::Solves),
                );

                // export button
                spawn_image_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(25.0),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    MyButtonType::Export,
                    asset_server.load("images/button_share.png").into(),
                    "Copy to clipboard".to_string(),
                    font.clone(),
                );

                // delete statistics button
                spawn_image_button(
                    parent,
                    UiRect {
                        bottom: Val::Px(25.0),
                        left: Val::Px(145.0),
                        ..default()
                    },
                    MyButtonType::DeleteStatistics,
                    asset_server.load("images/button_delete.png").into(),
                    "Delete all".to_string(),
                    font.clone(),
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
                        Some(PopupStatisticsTextType::Average),
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
                        Some(PopupStatisticsTextType::Best),
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
                        Some(PopupStatisticsTextType::Worst),
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
                        Some(PopupStatisticsTextType::Details),
                    );

                    let mut content = vec![];
                    for i in 0..statistics_manager.solves() {
                        content.push(format!("{}. {}", i + 1, statistics_manager.get_record(i)));
                    }

                    // scroll bar background
                    spawn_scroll_bar(
                        parent,
                        Size::new(Val::Px(275.0), Val::Px(450.0)),
                        UiRect {
                            right: Val::Px(25.0),
                            bottom: Val::Px(25.0),
                            ..default()
                        },
                        content,
                        SCROLL_BAR_MAX_ITEMS,
                        font.clone(),
                        true,
                        None::<Node>, // 아무 타입이든 지정하긴 해야 되네
                    );
                }
            });
        });
}

pub fn popup_system_statistics(
    mut commands: Commands,
    scroll_bar_query: Query<(&mut ScrollBar, &Children, Entity)>,
    mut statistics_text_query: Query<(&mut Text, &PopupStatisticsTextType, Entity)>,
    delete_statistics_event: EventReader<DeleteStatisticsEvent>,
) {
    if !delete_statistics_event.is_empty() {
        for (mut text, text_type, entity) in statistics_text_query.iter_mut() {
            match text_type {
                PopupStatisticsTextType::Solves => text.sections[0].value = "Solves\n0".to_string(),
                PopupStatisticsTextType::Average
                | PopupStatisticsTextType::Best
                | PopupStatisticsTextType::Worst
                | PopupStatisticsTextType::Details => {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
        if let Ok((_, _, entity)) = scroll_bar_query.get_single() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_text(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    text_type: Option<PopupStatisticsTextType>,
) {
    let text_bundle = TextBundle::from_section(
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
    });
    if let Some(text_type) = text_type {
        parent.spawn((text_bundle, text_type));
    } else {
        parent.spawn(text_bundle);
    };
}
