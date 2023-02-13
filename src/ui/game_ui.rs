use crate::{player::PlayerState, ui::*};

#[derive(Component)]
pub struct GameUI;

pub struct GameUIPlugin;

#[derive(Bundle, Default)]
pub struct ButtonInfoBundle {
    pub interaction_history: InteractionHistory,
    pub hover_timer: HoverTimer,
}

#[derive(Component, Default)]
pub struct InteractionHistory {
    pub prev: Interaction,
    pub curr: Interaction,
}

#[derive(Component, Default)]
pub struct HoverTimer(pub Timer);

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_game_ui)
            .add_system(game_ui_button_system)
            .add_system(button_hover_system)
            .add_system(game_ui_text_system)
            .add_system(scroll_bar_system)
            .add_system_set(SystemSet::on_enter(PlayerState::Clear).with_system(spawn_clear_ui))
            .add_system_set(SystemSet::on_exit(PlayerState::Clear).with_system(despawn_clear_ui));
    }
}

fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            spawn_image_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::Reset,
                asset_server.load("images/button_reset.png").into(),
                "Reset".to_string(),
                font.clone(),
            );

            // shuffle button
            spawn_image_button(
                parent,
                UiRect {
                    right: Val::Px(170.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::Shuffle,
                asset_server.load("images/button_shuffle.png").into(),
                "Shuffle".to_string(),
                font.clone(),
            );

            // mode selection button
            spawn_button(
                parent,
                UiRect {
                    left: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                "Mode\nPractice".to_string(),
                font.clone(),
                MyButtonType::ModeSelection,
                Some(MyTextType::ModeSelection),
                asset_server.load("images/button.png").into(),
            );

            // settings button
            spawn_image_button(
                parent,
                UiRect {
                    right: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::Settings,
                asset_server.load("images/button_settings.png").into(),
                "Settings".to_string(),
                font.clone(),
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
                MyTextType::PlayerInfo,
            ));
        });
}

fn spawn_clear_ui(
    mut commands: Commands,
    game_ui: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
    game_mode: Res<State<GameMode>>,
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
                    top: Val::Px(70.0),
                    ..default()
                },
                ..default()
            }),
            MyTextType::GameClear,
        ));

        if *game_mode.current() == GameMode::DailyPuzzle {
            // enroll score button
            spawn_image_button(
                parent,
                UiRect {
                    right: Val::Px(450.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                MyButtonType::PopupEnrollScore,
                asset_server.load("images/button_enroll_score.png").into(),
                "Enroll Score".to_string(),
                font.clone(),
            );
        }
    });
}

fn despawn_clear_ui(
    mut commands: Commands,
    button_query: Query<(Entity, &MyButtonType)>,
    text_query: Query<(Entity, &MyTextType)>,
) {
    for (button, button_type) in button_query.iter() {
        if *button_type == MyButtonType::PopupEnrollScore {
            commands.entity(button).despawn_recursive();
        }
    }
    for (text, &text_type) in text_query.iter() {
        if text_type == MyTextType::GameClear {
            commands.entity(text).despawn_recursive();
        }
    }
}

pub fn spawn_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: MyButtonType,
    text_type: Option<MyTextType>,
    image: UiImage,
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
                image,
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            let bundle = TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: TEXT_SIZE,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER);
            match text_type {
                Some(text_type) => parent.spawn((bundle, text_type)),
                None => parent.spawn(bundle),
            };
        });
}

pub fn spawn_image_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    button_type: MyButtonType,
    image: UiImage,
    info_text: String,
    font: Handle<Font>,
) {
    let info_position = match button_type {
        MyButtonType::Settings | MyButtonType::PopupEnrollScore => Some(UiRect {
            bottom: Val::Px(-60.0),
            ..default()
        }),
        MyButtonType::DateSelection => Some(UiRect {
            top: Val::Px(-60.0),
            left: Val::Px(-40.0),
            ..default()
        }),
        _ => None,
    };
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position,
                    ..default()
                },
                image,
                ..default()
            },
            button_type,
            ButtonInfoBundle::default(),
        ))
        .with_children(|parent| {
            spawn_button_info(parent, info_text, info_position, font);
        });
}

pub fn spawn_button_info(
    parent: &mut ChildBuilder,
    text: String,
    position: Option<UiRect>,
    font: Handle<Font>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: position.unwrap_or(UiRect {
                    top: Val::Px(-60.0),
                    ..default()
                }),
                ..default()
            },
            visibility: Visibility::INVISIBLE,
            background_color: Color::BLACK.into(),
            z_index: ZIndex::Global(3),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size: TEXT_SIZE,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::Px(5.0), Val::Px(5.0)),
                    ..default()
                }),
            );
        });
}
