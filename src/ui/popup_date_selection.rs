use crate::{
    network::{Network, NetworkChannel},
    player::PlayerState,
    *,
};
use bevy::prelude::*;
use chrono::{Datelike, Month, Months, NaiveDate};
use num_traits::FromPrimitive;

#[derive(Component)]
pub struct CalendarUI(NaiveDate);

#[derive(Component)]
pub struct MonthYearText;

#[derive(Component)]
pub enum PopupDateSelectionButtonType {
    MonthPrev,
    MonthNext,
    Date(NaiveDate),
}

pub fn spawn_popup_date_selection(
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
                // date selection text
                spawn_text(
                    parent,
                    UiRect {
                        top: Val::Px(25.0),
                        right: Val::Px(0.0),
                        ..default()
                    },
                    Size::new(Val::Percent(100.0), Val::Auto),
                    "Date Selection".to_string(),
                    font.clone(),
                    Color::WHITE,
                    None,
                );

                // month, year text
                spawn_text(
                    parent,
                    UiRect {
                        top: Val::Px(100.0),
                        left: Val::Px(0.0),
                        right: Val::Px(0.0),
                        ..default()
                    },
                    Size::new(Val::Percent(100.0), Val::Auto),
                    "January 2023".to_string(),
                    font.clone(),
                    Color::WHITE,
                    Some(MonthYearText),
                );

                // month prev button
                spawn_popup_date_selection_button(
                    parent,
                    UiRect {
                        top: Val::Px(95.0),
                        left: Val::Px(90.0),
                        ..default()
                    },
                    "<".to_string(),
                    font.clone(),
                    PopupDateSelectionButtonType::MonthPrev,
                );

                // month next button
                spawn_popup_date_selection_button(
                    parent,
                    UiRect {
                        top: Val::Px(95.0),
                        right: Val::Px(90.0),
                        ..default()
                    },
                    ">".to_string(),
                    font.clone(),
                    PopupDateSelectionButtonType::MonthNext,
                );

                // calendar
                parent
                    .spawn((
                        NodeBundle {
                            background_color: Color::WHITE.into(),
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    left: Val::Px(90.0),
                                    bottom: Val::Px(25.0),
                                    ..default()
                                },
                                size: Size::new(Val::Px(420.0), Val::Px(420.0)),
                                ..default()
                            },
                            ..default()
                        },
                        CalendarUI(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
                    ))
                    .with_children(|parent| {
                        // date (1 ~ 28,29,30,31)
                        spawn_calendar_ui(
                            parent,
                            font.clone(),
                            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                        );
                    });
            });
        });
}

pub fn popup_system_date_selection(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &PopupDateSelectionButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut calendar_ui_query: Query<(&mut CalendarUI, Entity, &Children)>,
    mut month_year_text_query: Query<&mut Text, (With<MonthYearText>, Without<MyTextType>)>,
    asset_server: Res<AssetServer>,
    mut player_state: ResMut<State<PlayerState>>,
    mut date_text_query: Query<(&mut Text, &MyTextType)>,
    mut network_channel: Res<NetworkChannel>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let (mut calendar_ui, entity, children) = calendar_ui_query.single_mut();
    let mut month_year_text = month_year_text_query.single_mut();

    // button interactions
    for (interaction, mut color, button_type) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                match button_type {
                    PopupDateSelectionButtonType::MonthPrev => {
                        calendar_ui.0 = calendar_ui.0.checked_sub_months(Months::new(1)).unwrap();
                        let date = calendar_ui.0;
                        for &child in children {
                            commands.entity(child).despawn_recursive();
                        }
                        commands.entity(entity).with_children(|parent| {
                            spawn_calendar_ui(parent, font.clone(), date);
                        });
                        month_year_text.sections[0].value = format!(
                            "{} {}",
                            Month::from_u32(date.month()).unwrap().name(),
                            date.year_ce().1
                        )
                    }
                    PopupDateSelectionButtonType::MonthNext => {
                        calendar_ui.0 = calendar_ui.0.checked_add_months(Months::new(1)).unwrap();
                        let date = calendar_ui.0;
                        for &child in children {
                            commands.entity(child).despawn_recursive();
                        }
                        commands.entity(entity).with_children(|parent| {
                            spawn_calendar_ui(parent, font.clone(), date);
                        });
                        month_year_text.sections[0].value = format!(
                            "{} {}",
                            Month::from_u32(date.month()).unwrap().name(),
                            date.year_ce().1
                        )
                    }
                    PopupDateSelectionButtonType::Date(date) => {
                        info!("{}", date);
                        Network::get_daily_puzzle(*date, &mut player_state, &mut network_channel);
                        for (mut text, &text_type) in date_text_query.iter_mut() {
                            if text_type == MyTextType::Date {
                                text.sections[0].value = format!(
                                    "Date: {}. {}. {}.",
                                    date.year_ce().1,
                                    date.month(),
                                    date.day()
                                );
                            }
                        }
                    }
                }
                *color = BUTTON_PRESS_COLOR.into();
            }
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_NORMAL_COLOR.into(),
        }
    }
}

fn spawn_text(
    parent: &mut ChildBuilder,
    position: UiRect,
    size: Size,
    text: String,
    font: Handle<Font>,
    color: Color,
    text_type: Option<MonthYearText>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position,
                size,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let text_bundle = TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: TEXT_SIZE,
                    color,
                },
            )
            .with_text_alignment(TextAlignment::CENTER);
            if let Some(text_type) = text_type {
                parent.spawn((text_bundle, text_type));
            } else {
                parent.spawn(text_bundle);
            }
        });
}

fn spawn_popup_date_selection_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: PopupDateSelectionButtonType,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(60.0), Val::Px(60.0)),
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

fn spawn_calendar_ui(parent: &mut ChildBuilder, font: Handle<Font>, first_date: NaiveDate) {
    // day (sun ~ sat)
    let day_char_arr = ['S', 'M', 'T', 'W', 'T', 'F', 'S'];
    for (i, day_char) in day_char_arr.iter().enumerate() {
        spawn_text(
            parent,
            UiRect {
                top: Val::Px(0.0),
                left: Val::Px(60.0 * i as f32),
                ..default()
            },
            Size::new(Val::Px(60.0), Val::Px(60.0)),
            day_char.to_string(),
            font.clone(),
            Color::BLACK,
            None,
        );
    }

    // get_week: week start from sunday
    let get_week = |date: NaiveDate| {
        let week = date.iso_week().week();
        let year_border_corrected = if date.month() == 1 && week > 50 {
            0
        } else {
            week
        };
        let sunday_addition = if date.weekday().number_from_sunday() == 1 {
            1
        } else {
            0
        };
        year_border_corrected + sunday_addition
    };
    for date in first_date.iter_days() {
        if date.month() != first_date.month() {
            break;
        }
        spawn_popup_date_selection_button(
            parent,
            UiRect {
                left: Val::Px(60.0 * date.weekday().num_days_from_sunday() as f32),
                top: Val::Px(60.0 * (1 + get_week(date) - get_week(first_date)) as f32),
                ..default()
            },
            date.day().to_string(),
            font.clone(),
            PopupDateSelectionButtonType::Date(date),
        );
    }
}
