use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

use crate::{game::GameState, player::PlayerState};

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_input).add_system_set(
            SystemSet::on_update(PlayerState::Playing)
                .with_system(input_keyboard)
                .with_system(input_click),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameInput {
    Up(i32, i32),
    Down(i32, i32),
    Left(i32, i32),
    Right(i32, i32),
}

impl GameInput {
    pub fn get_keycode(&self) -> KeyCode {
        match self {
            GameInput::Up(_, _) => KeyCode::Up,
            GameInput::Down(_, _) => KeyCode::Down,
            GameInput::Left(_, _) => KeyCode::Left,
            GameInput::Right(_, _) => KeyCode::Right,
        }
    }

    pub fn dx(&self) -> i32 {
        match self {
            GameInput::Up(dx, _) => *dx,
            GameInput::Down(dx, _) => *dx,
            GameInput::Left(dx, _) => *dx,
            GameInput::Right(dx, _) => *dx,
        }
    }

    pub fn dy(&self) -> i32 {
        match self {
            GameInput::Up(_, dy) => *dy,
            GameInput::Down(_, dy) => *dy,
            GameInput::Left(_, dy) => *dy,
            GameInput::Right(_, dy) => *dy,
        }
    }
}

const BUFFER_MAX: usize = 3;

#[derive(Component)]
pub struct InputBuffer {
    buffer: VecDeque<GameInput>,
    last_input: Option<GameInput>,
}

#[derive(Component)]
pub struct InputInversionFlag(pub bool);
#[derive(Component)]
pub struct MoveImmediate(pub bool);
#[derive(Resource, Default)]
pub struct InputTimer(Timer);

impl InputBuffer {
    fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            last_input: None,
        }
    }

    fn push(&mut self, value: GameInput) {
        self.buffer.push_back(value);
        self.last_input = Some(value);
    }

    pub fn pop(&mut self) -> Option<GameInput> {
        self.buffer.pop_front()
    }

    fn try_push(&mut self, value: GameInput, input_timer: &mut ResMut<InputTimer>) {
        if let Some(last_input) = self.last_input {
            if last_input == value {
                if input_timer.0.finished() {
                    self.push(value);
                    input_timer.0.reset();
                }
            } else {
                self.push(value)
            }
        } else {
            self.push(value)
        }
    }
}

fn setup_input(mut commands: Commands, mut input_timer: ResMut<InputTimer>) {
    commands.spawn((
        Name::new("InputSystem"),
        InputBuffer::new(),
        InputInversionFlag(false),
        MoveImmediate(false),
    ));
    *input_timer = InputTimer(Timer::from_seconds(0.03, TimerMode::Once));
}

fn just_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::Down)
        || keyboard_input.just_pressed(KeyCode::Left)
        || keyboard_input.just_pressed(KeyCode::Right)
}

fn input_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_system: Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut input_timer: ResMut<InputTimer>,
    time: Res<Time>,
) {
    let (_, inversion_flag) = input_system.single_mut();
    if just_pressed(&keyboard_input) {
        if keyboard_input.just_pressed(KeyCode::Up) {
            enqueue_input(0, 1, inversion_flag.0, &mut input_system, &mut input_timer);
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            enqueue_input(0, -1, inversion_flag.0, &mut input_system, &mut input_timer);
        } else if keyboard_input.just_pressed(KeyCode::Left) {
            enqueue_input(1, 0, inversion_flag.0, &mut input_system, &mut input_timer);
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            enqueue_input(-1, 0, inversion_flag.0, &mut input_system, &mut input_timer);
        }
    }
    input_timer.0.tick(time.delta());
}

fn input_click(
    mut game_query: Query<&GameState>,
    transforms: Query<&mut Transform>,
    mut input_timer: ResMut<InputTimer>,
    mut input_system: Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut events: EventReader<PickingEvent>,
) {
    let game = game_query.single_mut();
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            let transform = transforms.get(*e).unwrap();
            let dx = transform.translation.x.round() as i32 - game.x;
            let dz = transform.translation.z.round() as i32 - game.z;
            enqueue_input(dx, dz, false, &mut input_system, &mut input_timer);
        }
    }
}

fn enqueue_input(
    dx: i32,
    dz: i32,
    inverse: bool,
    input_system: &mut Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut input_timer: &mut ResMut<InputTimer>,
) {
    let (mut input_buffer, _) = input_system.single_mut();
    if input_buffer.buffer.len() < BUFFER_MAX {
        match (dx, dz) {
            (0, 1) => {
                if inverse {
                    input_buffer.try_push(GameInput::Down(0, -1), &mut input_timer);
                } else {
                    input_buffer.try_push(GameInput::Up(0, 1), &mut input_timer);
                }
            }
            (0, -1) => {
                if inverse {
                    input_buffer.try_push(GameInput::Up(0, 1), &mut input_timer);
                } else {
                    input_buffer.try_push(GameInput::Down(0, -1), &mut input_timer);
                }
            }
            (1, 0) => {
                if inverse {
                    input_buffer.try_push(GameInput::Right(-1, 0), &mut input_timer);
                } else {
                    input_buffer.try_push(GameInput::Left(1, 0), &mut input_timer);
                }
            }
            (-1, 0) => {
                if inverse {
                    input_buffer.try_push(GameInput::Left(1, 0), &mut input_timer);
                } else {
                    input_buffer.try_push(GameInput::Right(-1, 0), &mut input_timer);
                }
            }
            _ => (),
        }
    }
}
