use crate::{player::PlayerState, ui::*};

pub const POPUP_BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub struct PopupCloseButton;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Popup;

pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(PlayerState::ModeSelectionPopup)
                .with_system(spawn_popup_mode_selection),
        )
        .add_system_set(
            SystemSet::on_enter(PlayerState::StatisticsPopup).with_system(spawn_popup_statistics),
        )
        .add_system_set(
            SystemSet::on_enter(PlayerState::DateSelectionPopup)
                .with_system(spawn_popup_date_selection),
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
            SystemSet::on_exit(PlayerState::ModeSelectionPopup).with_system(despawn_popup),
        )
        .add_system_set(SystemSet::on_exit(PlayerState::StatisticsPopup).with_system(despawn_popup))
        .add_system_set(
            SystemSet::on_exit(PlayerState::DateSelectionPopup).with_system(despawn_popup),
        );
    }
}

fn popup_close_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &PopupCloseButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_state: ResMut<State<PlayerState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (interaction, mut color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = BUTTON_PRESS_COLOR.into();
                let _ = player_state.pop();
            }
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_NORMAL_COLOR.into(),
        }
    }

    // press Esc: popup close
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let _ = player_state.pop();
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
    font: Handle<Font>,
    child_builder: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn((
            // dark background
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
                z_index: ZIndex::Global(1),
                ..default()
            },
            Popup,
        ))
        .with_children(|parent| {
            // close button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(50.0),
                                top: Val::Px(50.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    },
                    PopupCloseButton,
                ))
                .with_children(|parent| {
                    // button text
                    parent.spawn(
                        TextBundle::from_section(
                            "Close",
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
                .with_children(child_builder);
        });
}
