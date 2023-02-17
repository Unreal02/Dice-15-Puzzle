use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

use crate::{
    game::{GameError, GameState},
    local_storage::LocalStorage,
    player::{PlayLog, PlayerState},
};

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_input).add_system_set(
            SystemSet::new()
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

    fn try_push(
        &mut self,
        value: GameInput,
        input_timer: &mut ResMut<InputTimer>,
        playlog: &mut PlayLog,
    ) -> Result<(), GameError> {
        if let Some(last_input) = self.last_input {
            if last_input == value {
                if input_timer.0.finished() {
                    self.push(value.clone());
                    playlog.add_log(value);
                    input_timer.0.reset();
                    Ok(())
                } else {
                    Err(GameError::AbnormalInput)
                }
            } else {
                self.push(value.clone());
                playlog.add_log(value);
                Ok(())
            }
        } else {
            self.push(value.clone());
            playlog.add_log(value);
            Ok(())
        }
    }
}

pub fn setup_input(mut commands: Commands, mut input_timer: ResMut<InputTimer>) {
    commands.spawn((
        Name::new("InputSystem"),
        InputBuffer::new(),
        InputInversionFlag(LocalStorage::get_input_inversion().unwrap_or(false)),
        MoveImmediate(LocalStorage::get_move_immediate().unwrap_or(false)),
    ));
    *input_timer = InputTimer(Timer::from_seconds(0.03, TimerMode::Once));
}

fn just_pressed(keyboard_input: &Res<Input<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::Down)
        || keyboard_input.just_pressed(KeyCode::Left)
        || keyboard_input.just_pressed(KeyCode::Right)
        || keyboard_input.just_pressed(KeyCode::Z)
        || keyboard_input.just_pressed(KeyCode::X)
}

fn input_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_system: Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut input_timer: ResMut<InputTimer>,
    mut play_log: Query<&mut PlayLog>,
    time: Res<Time>,
    player_state: Res<State<PlayerState>>,
) {
    if *player_state.current() == PlayerState::ModeSelectionPopup
        || *player_state.current() == PlayerState::StatisticsPopup
    {
        return;
    }
    let (mut input_buffer, inversion_flag) = input_system.single_mut();
    let mut play_log = play_log.single_mut();
    if just_pressed(&keyboard_input) {
        if keyboard_input.just_pressed(KeyCode::Up) {
            InputHandler::direction(
                0,
                1,
                inversion_flag.0,
                &mut input_buffer,
                &mut play_log,
                &mut input_timer,
            );
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            InputHandler::direction(
                0,
                -1,
                inversion_flag.0,
                &mut input_buffer,
                &mut play_log,
                &mut input_timer,
            );
        } else if keyboard_input.just_pressed(KeyCode::Left) {
            InputHandler::direction(
                1,
                0,
                inversion_flag.0,
                &mut input_buffer,
                &mut play_log,
                &mut input_timer,
            );
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            InputHandler::direction(
                -1,
                0,
                inversion_flag.0,
                &mut input_buffer,
                &mut play_log,
                &mut input_timer,
            );
        } else if keyboard_input.just_pressed(KeyCode::Z) {
            InputHandler::undo(
                inversion_flag.0,
                &mut input_buffer,
                &mut play_log,
                &mut input_timer,
            )
        } else if keyboard_input.just_pressed(KeyCode::X) {
            InputHandler::redo(
                inversion_flag.0,
                &mut input_buffer,
                &mut play_log,
                &mut input_timer,
            )
        }
    }
    input_timer.0.tick(time.delta());
}

fn input_click(
    mut game_query: Query<&GameState>,
    transforms: Query<&mut Transform>,
    mut input_timer: ResMut<InputTimer>,
    mut input_system: Query<(&mut InputBuffer, &InputInversionFlag)>,
    mut play_log: Query<&mut PlayLog>,
    mut events: EventReader<PickingEvent>,
    player_state: Res<State<PlayerState>>,
) {
    if *player_state.current() != PlayerState::Idle
        && *player_state.current() != PlayerState::Shuffled
        && *player_state.current() != PlayerState::Solving
        && *player_state.current() != PlayerState::Clear
    {
        return;
    }
    let game = game_query.single_mut();
    let mut play_log = play_log.single_mut();
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            let transform = transforms.get(*e).unwrap();
            let dx = transform.translation.x.round() as i32 - game.x;
            let dz = transform.translation.z.round() as i32 - game.z;
            InputHandler::direction(
                dx,
                dz,
                false,
                &mut input_system.single_mut().0,
                &mut play_log,
                &mut input_timer,
            );
        }
    }
}

pub struct InputHandler;

impl InputHandler {
    fn direction(
        dx: i32,
        dz: i32,
        inverse: bool,
        input_buffer: &mut InputBuffer,
        play_log: &mut PlayLog,
        input_timer: &mut ResMut<InputTimer>,
    ) {
        match enqueue_input(dx, dz, inverse, input_buffer, play_log, input_timer) {
            Ok(_) => play_log.clear_redo_buf(),
            Err(_) => (),
        }
    }

    pub fn undo(
        inverse: bool,
        input_buffer: &mut InputBuffer,
        play_log: &mut PlayLog,
        input_timer: &mut ResMut<InputTimer>,
    ) {
        let log = play_log.undo();
        if let Some(input) = log {
            match enqueue_input(
                input.dx(),
                input.dy(),
                !inverse,
                input_buffer,
                play_log,
                input_timer,
            ) {
                Ok(_) => {
                    let log = play_log.undo().unwrap();
                    play_log.add_redo(log);
                    play_log.undo_used = true;
                }
                Err(_) => play_log.add_log(input),
            }
        }
    }

    pub fn redo(
        inverse: bool,
        input_buffer: &mut InputBuffer,
        play_log: &mut PlayLog,
        input_timer: &mut ResMut<InputTimer>,
    ) {
        let log = play_log.redo();
        if let Some(input) = log {
            match enqueue_input(
                input.dx(),
                input.dy(),
                !inverse,
                input_buffer,
                play_log,
                input_timer,
            ) {
                Ok(_) => (),
                Err(_) => play_log.add_redo(input),
            }
        }
    }
}

fn enqueue_input(
    dx: i32,
    dz: i32,
    inverse: bool,
    input_buffer: &mut InputBuffer,
    mut play_log: &mut PlayLog,
    mut input_timer: &mut ResMut<InputTimer>,
) -> Result<(), GameError> {
    if input_buffer.buffer.len() < BUFFER_MAX {
        match (dx, dz) {
            (0, 1) => {
                if inverse {
                    input_buffer.try_push(GameInput::Down(0, -1), &mut input_timer, &mut play_log)
                } else {
                    input_buffer.try_push(GameInput::Up(0, 1), &mut input_timer, &mut play_log)
                }
            }
            (0, -1) => {
                if inverse {
                    input_buffer.try_push(GameInput::Up(0, 1), &mut input_timer, &mut play_log)
                } else {
                    input_buffer.try_push(GameInput::Down(0, -1), &mut input_timer, &mut play_log)
                }
            }
            (1, 0) => {
                if inverse {
                    input_buffer.try_push(GameInput::Right(-1, 0), &mut input_timer, &mut play_log)
                } else {
                    input_buffer.try_push(GameInput::Left(1, 0), &mut input_timer, &mut play_log)
                }
            }
            (-1, 0) => {
                if inverse {
                    input_buffer.try_push(GameInput::Left(1, 0), &mut input_timer, &mut play_log)
                } else {
                    input_buffer.try_push(GameInput::Right(-1, 0), &mut input_timer, &mut play_log)
                }
            }
            _ => Err(GameError::InvalidInput),
        }
    } else {
        Err(GameError::BufFull)
    }
}
