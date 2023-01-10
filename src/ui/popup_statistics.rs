use crate::{player::PlayerState, ui::*};

pub fn spawn_popup_statistics(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, font.clone(), |parent| {
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
            });
        });
}

pub fn popup_system_statistics(
    mut player_state: ResMut<State<PlayerState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // press Esc: popup close
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let _ = player_state.pop();
    }
}
