use crate::ui::*;
use bevy::prelude::*;

pub fn spawn_text_input_box(
    parent: &mut ChildBuilder,
    position: UiRect,
    size: Size,
    font: Handle<Font>,
    max_len: usize,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position,
                size,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::BLACK,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER_LEFT)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(5.0),
                        right: Val::Px(5.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    ..default()
                }),
                MyTextType::TextInputBox(max_len),
            ));
        });
}
