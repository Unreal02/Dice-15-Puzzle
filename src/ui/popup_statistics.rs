use crate::{
    duration_to_string::duration_to_string, player::PlayerState,
    statistics_manager::StatisticsManager, ui::*,
};

pub fn spawn_popup_statistics(
    mut commands: Commands,
    mut game_ui_query: Query<Entity, With<GameUI>>,
    statistics_manager_query: Query<&StatisticsManager>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Quicksand-Bold.ttf");
    let statistics_manager = statistics_manager_query.single();

    commands
        .entity(game_ui_query.single_mut())
        .with_children(|parent| {
            spawn_popup_panel(parent, font.clone(), |parent| {
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
                    format!("Solves: {}", statistics_manager.solves()),
                    font.clone(),
                );

                if statistics_manager.solves() != 0 {
                    // average
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(120.0),
                            left: Val::Px(25.0),
                            ..default()
                        },
                        format!(
                            "Average: {}",
                            duration_to_string(statistics_manager.average())
                        ),
                        font.clone(),
                    );

                    // best
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(160.0),
                            left: Val::Px(25.0),
                            ..default()
                        },
                        format!("Best: {}", duration_to_string(statistics_manager.best())),
                        font.clone(),
                    );

                    // worst
                    spawn_text(
                        parent,
                        UiRect {
                            top: Val::Px(200.0),
                            left: Val::Px(25.0),
                            ..default()
                        },
                        format!("Worst: {}", duration_to_string(statistics_manager.worst())),
                        font.clone(),
                    );
                }
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
