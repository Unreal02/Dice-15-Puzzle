use bevy::prelude::*;

use crate::{game::GameStages, player::PlayerState};

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_input).add_system_set(
            SystemSet::on_update(PlayerState::Playing)
                .with_system(enqueue_input.before(GameStages::UpdateBlock)),
        );
    }
}

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
pub struct InputBuffer(Vec<GameInput>);
#[derive(Component)]
pub struct InputInversionFlag(bool);
#[derive(Component)]
pub struct MoveImmediate(pub bool);
#[derive(Resource, Default)]
pub struct InputTimer(Timer);

impl InputBuffer {
    pub fn push(&mut self, value: GameInput) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> Option<GameInput> {
        self.0.pop()
    }
}

fn setup_input(mut commands: Commands, mut input_timer: ResMut<InputTimer>) {
    commands.spawn((
        Name::new("InputSystem"),
        InputBuffer(Vec::new()),
        InputInversionFlag(false),
        MoveImmediate(false),
    ));
    *input_timer = InputTimer(Timer::from_seconds(0.02, TimerMode::Once));
}

fn enqueue_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_system: Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut input_timer: ResMut<InputTimer>,
    time: Res<Time>,
) {
    let (mut input_buffer, inversion_flag) = input_system.single_mut();
    if input_timer.0.just_finished() {
        input_timer.0.reset();
        if input_buffer.0.len() > BUFFER_MAX {
            return;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            if inversion_flag.0 {
                input_buffer.push(GameInput::Down(0, -1))
            } else {
                input_buffer.0.push(GameInput::Up(0, 1))
            }
        } else if keyboard_input.pressed(KeyCode::Down) {
            if inversion_flag.0 {
                input_buffer.0.push(GameInput::Up(0, 1))
            } else {
                input_buffer.0.push(GameInput::Down(0, -1))
            }
        } else if keyboard_input.pressed(KeyCode::Left) {
            if inversion_flag.0 {
                input_buffer.0.push(GameInput::Right(-1, 0))
            } else {
                input_buffer.0.push(GameInput::Left(1, 0))
            }
        } else if keyboard_input.pressed(KeyCode::Right) {
            if inversion_flag.0 {
                input_buffer.0.push(GameInput::Left(1, 0))
            } else {
                input_buffer.0.push(GameInput::Right(-1, 0))
            }
        }
    }
    input_timer.0.tick(time.delta());
}
