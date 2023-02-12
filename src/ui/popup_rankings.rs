use crate::{
    daily_puzzle_info::DailyPuzzleInfo,
    network::{Network, NetworkChannel},
    player::PlayerState,
    ui::*,
};
use bevy::prelude::*;

const SCROLL_BAR_MAX_ITEMS: usize = 10;

#[derive(Component, PartialEq, Eq)]
pub enum RankingType {
    Time,
    Move,
}

pub fn spawn_popup_rankings(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
    daily_puzzle_info_query: Query<&DailyPuzzleInfo>,
    mut player_state: ResMut<State<PlayerState>>,
    network_channel: Res<NetworkChannel>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_close_image = UiImage::from(asset_server.load("images/button_close.png"));
    let button_small_image = UiImage::from(asset_server.load("images/button_small.png"));
    let daily_puzzle_info = daily_puzzle_info_query.single();

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, button_close_image.clone(), font.clone(), |parent| {
                // daily puzzle rankings text
                parent.spawn(
                    TextBundle::from_section(
                        "Daily Puzzle Rankings (WIP)",
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

                // time/move select button
                spawn_ranking_type_button(
                    parent,
                    UiRect {
                        left: Val::Px(140.0),
                        bottom: Val::Px(455.0),
                        ..default()
                    },
                    button_small_image.clone(),
                    RankingType::Time,
                    "Time".to_string(),
                    font.clone(),
                );
                spawn_ranking_type_button(
                    parent,
                    UiRect {
                        right: Val::Px(140.0),
                        bottom: Val::Px(455.0),
                        ..default()
                    },
                    button_small_image.clone(),
                    RankingType::Move,
                    "Move".to_string(),
                    font.clone(),
                );

                // scroll bar background
                spawn_scroll_bar(
                    parent,
                    Size::new(Val::Px(550.0), Val::Px(410.0)),
                    UiRect {
                        left: Val::Px(25.0),
                        bottom: Val::Px(25.0),
                        ..default()
                    },
                    vec![],
                    SCROLL_BAR_MAX_ITEMS,
                    font.clone(),
                    true,
                    Some(RankingType::Time),
                );
                spawn_scroll_bar(
                    parent,
                    Size::new(Val::Px(550.0), Val::Px(410.0)),
                    UiRect {
                        left: Val::Px(25.0),
                        bottom: Val::Px(25.0),
                        ..default()
                    },
                    vec![],
                    SCROLL_BAR_MAX_ITEMS,
                    font.clone(),
                    false,
                    Some(RankingType::Move),
                );
            });
        });

    Network::get_daily_ranking(
        daily_puzzle_info.current_date,
        &mut player_state,
        &network_channel,
    );
}

fn spawn_ranking_type_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    image: UiImage,
    ranking_type: RankingType,
    text: String,
    font: Handle<Font>,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position,
                    size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                    ..default()
                },
                image,
                ..default()
            },
            ranking_type,
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

pub fn popup_rankings_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &RankingType),
        (Changed<Interaction>, With<Button>),
    >,
    mut scroll_bar_query: Query<(&mut Visibility, &RankingType), With<ScrollBar>>,
) {
    for (interaction, mut color, button_type) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                for (mut visibility, scroll_bar_type) in scroll_bar_query.iter_mut() {
                    visibility.is_visible = *button_type == *scroll_bar_type;
                }
                *color = (BUTTON_WHITE * BUTTON_PRESS_MUL).into();
            }
            Interaction::Hovered => *color = (BUTTON_WHITE * BUTTON_HOVER_MUL).into(),
            Interaction::None => *color = BUTTON_WHITE.into(),
        }
    }
}
