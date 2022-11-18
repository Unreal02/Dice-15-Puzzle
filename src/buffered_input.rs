use std::collections::VecDeque;

use bevy::prelude::*;

use crate::player::PlayerState;

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_input)
            .add_system_set(SystemSet::on_update(PlayerState::Playing).with_system(enqueue_input));
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
pub struct InputInversionFlag(bool);
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

fn enqueue_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_system: Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut input_timer: ResMut<InputTimer>,
    time: Res<Time>,
) {
    let (mut input_buffer, inversion_flag) = input_system.single_mut();
    if just_pressed(&keyboard_input) {
        if input_buffer.buffer.len() < BUFFER_MAX {
            if keyboard_input.pressed(KeyCode::Up) {
                if inversion_flag.0 {
                    input_buffer.try_push(GameInput::Down(0, -1), &mut input_timer)
                } else {
                    input_buffer.try_push(GameInput::Up(0, 1), &mut input_timer);
                }
            } else if keyboard_input.pressed(KeyCode::Down) {
                if inversion_flag.0 {
                    input_buffer.try_push(GameInput::Up(0, 1), &mut input_timer)
                } else {
                    input_buffer.try_push(GameInput::Down(0, -1), &mut input_timer);
                }
            } else if keyboard_input.pressed(KeyCode::Left) {
                if inversion_flag.0 {
                    input_buffer.try_push(GameInput::Right(-1, 0), &mut input_timer)
                } else {
                    input_buffer.try_push(GameInput::Left(1, 0), &mut input_timer);
                }
            } else if keyboard_input.pressed(KeyCode::Right) {
                if inversion_flag.0 {
                    input_buffer.try_push(GameInput::Left(1, 0), &mut input_timer)
                } else {
                    input_buffer.try_push(GameInput::Right(-1, 0), &mut input_timer);
                }
            }
        }
    }
    input_timer.0.tick(time.delta());
}
