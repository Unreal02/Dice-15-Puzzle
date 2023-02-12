use crate::ui::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

const SCROLL_SPEED: f32 = 1.0;

pub fn spawn_scroll_bar(
    parent: &mut ChildBuilder,
    size: Size,
    position: UiRect,
    content: Vec<String>,
    max_items: usize,
    font: Handle<Font>,
    visible: bool,
    extra_component: Option<impl Component>,
) {
    let node_bundle = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size,
            position,
            ..default()
        },
        visibility: Visibility {
            is_visible: visible,
        },
        background_color: Color::rgb(0.2, 0.2, 0.2).into(),
        ..default()
    };
    let scroll_bar = ScrollBar {
        content,
        max_items,
        position: 0.0,
    };
    match extra_component {
        Some(component) => parent.spawn((node_bundle, scroll_bar, component)),
        None => parent.spawn((node_bundle, scroll_bar)),
    }
    .with_children(|parent| {
        for i in 0..max_items {
            parent.spawn(
                TextBundle::from_section(
                    String::new(),
                    TextStyle {
                        font: font.clone(),
                        font_size: TEXT_SIZE,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(5.0 + 40.0 * i as f32),
                        left: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        }
    });
}

pub fn scroll_bar_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut scroll_bar_query: Query<(&mut ScrollBar, &Children, &Visibility)>,
    mut text_query: Query<&mut Text>,
) {
    // scroll bar
    for (mut scroll_bar, children, visibility) in scroll_bar_query.iter_mut() {
        if !visibility.is_visible {
            continue;
        }
        for scroll_event in scroll_events.iter() {
            let dy = match scroll_event.unit {
                MouseScrollUnit::Line => scroll_event.y,
                MouseScrollUnit::Pixel => scroll_event.y / 20.0,
            };
            scroll_bar.position -= dy * SCROLL_SPEED;
        }

        scroll_bar.position = scroll_bar.position.clamp(
            0.0,
            (if scroll_bar.content.len() >= scroll_bar.max_items {
                scroll_bar.content.len() - scroll_bar.max_items
            } else {
                0
            } as f32),
        );

        let start_position = scroll_bar.position as usize;
        for i in 0..children.len() {
            let index = start_position + i;
            text_query.get_mut(children[i]).unwrap().sections[0].value =
                if index < scroll_bar.content.len() {
                    scroll_bar.content[index].clone()
                } else {
                    "".to_string()
                };
        }
    }
}
