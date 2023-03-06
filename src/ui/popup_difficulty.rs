use crate::{
    block::{spawn_meshes, BlockMesh},
    game::{BoardSize, EasyMode, GameState, MAX_BOARD_SIZE, MIN_BOARD_SIZE},
    local_storage::LocalStorage,
    player::PlayerState,
    ui::*,
};
use bevy::{math::vec3, prelude::*};

#[derive(Component)]
pub enum DifficultyButtonType {
    SetBoardSize(usize),
    SetEasyMode(bool),
}

pub fn spawn_popup_difficulty(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let button_close_image = UiImage::from(asset_server.load("images/button_close.png"));
    let button_small_image = UiImage::from(asset_server.load("images/button_small.png"));

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, button_close_image.clone(), font.clone(), |parent| {
                // difficulty text
                parent.spawn(
                    TextBundle::from_section(
                        "Difficulty",
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

                // board size
                parent.spawn(
                    TextBundle::from_section(
                        "Board size",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(94.0),
                            top: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    }),
                );

                // board size button
                for i in MIN_BOARD_SIZE..MAX_BOARD_SIZE + 1 {
                    spawn_small_button(
                        parent,
                        UiRect {
                            left: Val::Px(100.0),
                            top: Val::Px(40.0 + 60.0 * i as f32),
                            ..default()
                        },
                        format!("{} x {}", i, i),
                        font.clone(),
                        DifficultyButtonType::SetBoardSize(i),
                        button_small_image.clone(),
                    );
                }

                // difficulty
                parent.spawn(
                    TextBundle::from_section(
                        "Difficulty",
                        TextStyle {
                            font: font.clone(),
                            font_size: TEXT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            right: Val::Px(100.0),
                            top: Val::Px(100.0),
                            ..default()
                        },
                        ..default()
                    }),
                );

                // difficulty button
                spawn_small_button(
                    parent,
                    UiRect {
                        right: Val::Px(100.0),
                        top: Val::Px(160.0),
                        ..default()
                    },
                    "Easy".to_string(),
                    font.clone(),
                    DifficultyButtonType::SetEasyMode(true),
                    button_small_image.clone(),
                );
                spawn_small_button(
                    parent,
                    UiRect {
                        right: Val::Px(100.0),
                        top: Val::Px(220.0),
                        ..default()
                    },
                    "Hard".to_string(),
                    font.clone(),
                    DifficultyButtonType::SetEasyMode(false),
                    button_small_image.clone(),
                );
            })
        });
}

pub fn popup_system_difficulty(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DifficultyButtonType),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button_type {
                    DifficultyButtonType::SetBoardSize(size) => {
                        LocalStorage::set_board_size(&BoardSize(*size));
                    }
                    DifficultyButtonType::SetEasyMode(value) => {
                        LocalStorage::set_easy_mode(&EasyMode(*value));
                    }
                }
                *color = (BUTTON_WHITE * BUTTON_PRESS_MUL).into();
            }
            Interaction::Hovered => *color = (BUTTON_WHITE * BUTTON_HOVER_MUL).into(),
            Interaction::None => *color = BUTTON_WHITE.into(),
        }
    }
}

pub fn popup_difficulty_close_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&Popup>),
        (Changed<Interaction>, With<Button>, With<PopupCloseButton>),
    >,
    mut player_state: ResMut<State<PlayerState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut board_size: ResMut<BoardSize>,
    mut easy_mode: ResMut<EasyMode>,
    mut game_query: Query<&mut GameState>,
    block_mesh_query: Query<Entity, With<BlockMesh>>,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut game = game_query.single_mut();
    let mut close = false;

    for (interaction, mut color, popup) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            close = true;
        }
        if popup.is_none() {
            *color = match *interaction {
                Interaction::Clicked => (BUTTON_WHITE * BUTTON_PRESS_MUL).into(),
                Interaction::Hovered => (BUTTON_WHITE * BUTTON_HOVER_MUL).into(),
                Interaction::None => BUTTON_WHITE.into(),
            };
        }
    }

    // press Esc: popup close
    if keyboard_input.just_pressed(KeyCode::Escape) {
        close = true;
    }

    if close {
        let mut change = false;
        if let Some(BoardSize(size)) = LocalStorage::get_board_size() {
            if board_size.0 != size {
                board_size.0 = size;
                change = true;
            }
        }
        if let Some(EasyMode(value)) = LocalStorage::get_easy_mode() {
            if easy_mode.0 != value {
                easy_mode.0 = value;
                change = true;
            }
        }

        if change {
            // spawn new meshes
            block_mesh_query.for_each(|entity| commands.entity(entity).despawn());
            let mesh_entities =
                spawn_meshes(&mut commands, board_size.0, meshes, materials, asset_server);
            *camera_query.single_mut() = Transform::from_xyz(
                (board_size.0 as f32 - 1.0) / 2.0,
                board_size.0 as f32 * 1.25,
                (board_size.0 as f32 - 1.0) / 2.0 + board_size.0 as f32 * 1.25,
            )
            .looking_at(
                vec3(
                    (board_size.0 as f32 - 1.0) / 2.0,
                    0.0,
                    (board_size.0 as f32 - 1.0) / 2.0,
                ),
                Vec3::Y,
            );
            game.init(board_size.0, &mesh_entities);
            player_state.replace(PlayerState::Idle).unwrap();
        } else {
            player_state.pop().unwrap();
        }
    }
}

fn spawn_small_button(
    parent: &mut ChildBuilder,
    position: UiRect,
    text: String,
    font: Handle<Font>,
    button_type: DifficultyButtonType,
    image: UiImage,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(150.0), Val::Px(50.0)),
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
