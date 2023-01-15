use super::{board_to_string, BoardString};
use crate::game::{GameState, MoveTimer};
use bevy::{ecs::system::SystemState, math::vec3, prelude::*, utils::HashMap};
use rand::random;
use std::time::Duration;

const SHUFFLE_NUMBER: i32 = 1000;

pub fn shuffle() -> BoardString {
    let mut world = World::new();
    let mut transforms = HashMap::<(usize, usize), Entity>::new();

    for x in 0..4 {
        for z in 0..4 {
            if x != 3 || z != 3 {
                transforms.insert(
                    (x, z),
                    world
                        .spawn(Transform::from_translation(vec3(x as f32, 0.0, z as f32)))
                        .id(),
                );
            }
        }
    }
    let mut game_state = GameState::default();
    game_state.init(&transforms);

    let mut state: SystemState<Query<&mut Transform>> = SystemState::new(&mut world);
    let mut query = state.get_mut(&mut world);
    let mut move_timer = MoveTimer(Timer::from_seconds(0.0, TimerMode::Once));
    move_timer.0.tick(Duration::from_secs(1));

    for _ in 0..SHUFFLE_NUMBER {
        match random::<u32>() % 4 {
            0 => game_state.move_block(0, 1, true, &mut move_timer, &mut query),
            1 => game_state.move_block(0, -1, true, &mut move_timer, &mut query),
            2 => game_state.move_block(1, 0, true, &mut move_timer, &mut query),
            3 => game_state.move_block(-1, 0, true, &mut move_timer, &mut query),
            _ => unreachable!(),
        }
    }

    return board_to_string(&query, &mut game_state);
}
