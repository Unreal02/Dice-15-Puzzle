use crate::{player::PlayerState, ui::*};

pub const POPUP_BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub struct PopupCloseButton;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Popup;

pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeleteStatisticsEvent>()
            .add_system_set(
                SystemSet::on_enter(PlayerState::SettingsPopup).with_system(spawn_popup_settings),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerState::DifficultyPopup)
                    .with_system(spawn_popup_difficulty),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerState::ModeSelectionPopup)
                    .with_system(spawn_popup_mode_selection),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerState::StatisticsPopup)
                    .with_system(spawn_popup_statistics),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerState::DateSelectionPopup)
                    .with_system(spawn_popup_date_selection),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerState::RankingsPopup).with_system(spawn_popup_rankings),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerState::EnrollScorePopup)
                    .with_system(spawn_popup_enroll_score),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::SettingsPopup)
                    .with_system(popup_close_button_system),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::DifficultyPopup)
                    .with_system(popup_system_difficulty)
                    .with_system(popup_difficulty_close_button_system),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::ModeSelectionPopup)
                    .with_system(popup_system_mode_selection)
                    .with_system(popup_close_button_system),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::StatisticsPopup)
                    .with_system(popup_system_statistics)
                    .with_system(popup_close_button_system),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::DateSelectionPopup)
                    .with_system(popup_system_date_selection)
                    .with_system(popup_close_button_system),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::RankingsPopup)
                    .with_system(popup_rankings_system)
                    .with_system(popup_close_button_system),
            )
            .add_system_set(
                SystemSet::on_update(PlayerState::EnrollScorePopup)
                    .with_system(popup_close_button_system),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::SettingsPopup).with_system(despawn_popup),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::DifficultyPopup).with_system(despawn_popup),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::ModeSelectionPopup).with_system(despawn_popup),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::StatisticsPopup).with_system(despawn_popup),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::DateSelectionPopup).with_system(despawn_popup),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::RankingsPopup).with_system(despawn_popup),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerState::EnrollScorePopup).with_system(despawn_popup),
            );
    }
}

fn popup_close_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&Popup>),
        (Changed<Interaction>, With<Button>, With<PopupCloseButton>),
    >,
    mut player_state: ResMut<State<PlayerState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
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
        player_state.pop().unwrap();
    }
}

fn despawn_popup(
    mut commands: Commands,
    popup_query: Query<Entity, With<Popup>>,
    mut mouse: ResMut<Input<MouseButton>>,
) {
    commands.entity(popup_query.single()).despawn_recursive();
    mouse.reset_all(); // prevent input after state change
}

pub fn spawn_popup_panel(
    parent: &mut ChildBuilder,
    close_button_image: UiImage,
    font: Handle<Font>,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn((
            // dark background
            ButtonBundle {
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
                z_index: ZIndex::Global(1),
                ..default()
            },
            Popup,
            PopupCloseButton,
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
                    // close button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    size: Size::new(Val::Px(60.0), Val::Px(60.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        right: Val::Px(20.0),
                                        top: Val::Px(20.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                image: close_button_image,
                                ..default()
                            },
                            PopupCloseButton,
                            ButtonInfoBundle::default(),
                        ))
                        .with_children(|parent| {
                            spawn_button_info(parent, "Close (Esc)".to_string(), None, font);
                        });
                })
                .with_children(child_builder);
        });
}
