use std::{
    cmp::{max, min},
    f32::consts::PI,
    mem::swap,
};

use bevy::{math::vec3, prelude::*, utils::HashMap};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickingCameraBundle;

use crate::{
    block::{spawn_meshes, Block, BlockMesh},
    buffered_input::{InputBuffer, MoveImmediate},
    network::NetworkChannel,
    player::{PlayerInfo, PlayerState},
    utils::{shuffle, string_to_board},
};

const INITIAL_BOARD_SIZE: usize = 4;
const BLOCK_MOVE_TIME: f32 = 0.3;

#[derive(Resource)]
pub struct BoardSize(pub usize);

#[derive(Resource, Default)]
pub struct MoveTimer(pub Timer);

pub enum GameError {
    BufFull,
    AbnormalInput,
    InvalidInput,
}

pub struct GamePlugin;

#[derive(Default, Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct GameState {
    pub size: usize,
    pub x: i32,
    pub z: i32,
    pub board: Board,
    pub is_shuffled: bool,
}

#[derive(Default, Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct Board(pub Vec<Vec<Option<Block>>>);

#[derive(SystemLabel)]
pub enum GameStages {
    UpdateBlock,
    CheckClear,
}

#[derive(Default, Resource)]
pub struct UrlReqestInfo(Option<String>);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardSize(INITIAL_BOARD_SIZE))
            .add_startup_system(setup)
            .add_system(update_block.label(GameStages::UpdateBlock))
            .add_system_set(
                SystemSet::on_update(PlayerState::Solving).with_system(
                    check_clear
                        .after(GameStages::UpdateBlock)
                        .label(GameStages::CheckClear),
                ),
            );
    }
}

impl GameState {
    /// init board
    pub fn init(&mut self, size: usize, mesh_entities: &HashMap<(usize, usize), Entity>) {
        for x in 0..size {
            self.board.0.push(Vec::new());
            for z in 0..size {
                self.board.0[x].push(if x != size - 1 || z != size - 1 {
                    Some(Block {
                        entity: mesh_entities.get(&(x, z)).unwrap().clone(),
                        goal: (z * size + x + 1) as i32 % (size * size) as i32,
                        moving: None,
                    })
                } else {
                    None
                });
            }
        }
        self.size = size;
        self.x = size as i32 - 1;
        self.z = size as i32 - 1;
    }

    /// swap `self.board.0[x0][z0]` and `self.board.0[x1][z1]`
    pub fn swap(&mut self, x0: usize, z0: usize, x1: usize, z1: usize) {
        if x0 == x1 {
            let arr = &mut self.board.0[x0];
            arr.swap(z0, z1);
        } else {
            let (a1, a2) = self.board.0.split_at_mut(max(x0, x1));
            swap(&mut a1[min(x0, x1)][z0], &mut a2[0][z1]);
        }
    }

    /// Move block.
    pub fn move_block(
        &mut self,
        dx: i32,
        dz: i32,
        immediate: bool,
        move_timer: &mut MoveTimer,
        transforms: &mut Query<&mut Transform>,
    ) {
        if self.x + dx < 0
            || self.x + dx >= self.size as i32
            || self.z + dz < 0
            || self.z + dz >= self.size as i32
            || !move_timer.0.finished()
        {
            return;
        }

        // translate block
        let (x0, z0) = (self.x as usize, self.z as usize);
        let (x1, z1) = ((self.x + dx) as usize, (self.z + dz) as usize);
        let block = self.board.0[x1][z1].as_mut().unwrap();

        // next transform of block
        let mut transform = transforms.get_mut(block.entity).unwrap();
        let prev_transform = transform.clone();
        let mut next_transform = prev_transform;
        next_transform.translation += vec3(-dx as f32, 0.0, -dz as f32);
        next_transform.rotate_x(-dz as f32 * PI * 0.5);
        next_transform.rotate_z(dx as f32 * PI * 0.5);

        if immediate {
            *transform = next_transform;
        } else {
            block.moving = Some((prev_transform, next_transform));
            *move_timer = MoveTimer(Timer::from_seconds(BLOCK_MOVE_TIME, TimerMode::Once));
        }

        self.swap(x0, z0, x1, z1);
        self.x += dx;
        self.z += dz;
    }

    pub fn reset(&mut self, move_timer: &mut MoveTimer, transforms: &mut Query<&mut Transform>) {
        if !move_timer.0.finished() {
            return;
        }
        for x in 0..self.size {
            for z in 0..self.size {
                loop {
                    let (x1, z1) = match &self.board.0[x][z] {
                        Some(block) => {
                            let (goal_x, goal_z) = (
                                (block.goal as usize - 1) % self.size,
                                (block.goal as usize - 1) / self.size,
                            );
                            let mut transform = transforms.get_mut(block.entity).unwrap();
                            transform.translation = vec3(goal_x as f32, 0.0, goal_z as f32);
                            transform.rotation = Quat::IDENTITY;
                            (goal_x, goal_z)
                        }
                        None => (self.size - 1, self.size - 1),
                    };
                    if x == x1 && z == z1 {
                        break;
                    }
                    self.swap(x, z, x1, z1);
                }
            }
        }
        self.x = self.size as i32 - 1;
        self.z = self.size as i32 - 1;
        self.is_shuffled = false;
    }

    pub fn shuffle(&mut self, transforms: &mut Query<&mut Transform>) {
        let board_string = shuffle(self.size);
        string_to_board(&board_string, transforms, self);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut move_timer: ResMut<MoveTimer>,
    board_size: Res<BoardSize>,
) {
    let mesh_entities = spawn_meshes(&mut commands, board_size.0, meshes, materials, asset_server);
    let mut new_game = GameState::default();
    new_game.init(board_size.0, &mesh_entities);

    *move_timer = MoveTimer(Timer::from_seconds(0.1, TimerMode::Once));

    // Spawn Game
    commands
        .spawn((SpatialBundle::default(), new_game, Name::new("GAME")))
        .push_children(
            mesh_entities
                .iter()
                .map(|(_, v)| *v)
                .collect::<Vec<Entity>>()
                .as_slice(),
        );

    // light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: vec3(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                (board_size.0 as f32 - 1.0) / 2.0,
                board_size.0 as f32 * 1.25,
                (board_size.0 as f32 - 1.0) / 2.0 + board_size.0 as f32 * 1.25,
            )
            .looking_at(
                vec3(
                    (board_size.0 as f32 - 1.0) / 2.0,
                    0.0,
                    (board_size.0 as f32 - 1.0) / 2.0,
                ),
                Vec3::Y,
            ),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}

pub fn try_url_load(
    mut player_state: ResMut<State<PlayerState>>,
    network_channel: Res<NetworkChannel>,
) {
    let window = web_sys::window().unwrap();
    let query = window.location().search().map(|raw_url| {
        let url_key = raw_url.trim_start_matches('?').to_owned();
        info!("Try getting url reqeust {}", &url_key);
        url_key
    });
    match query {
        Ok(url_key) => {
            if url_key.len() > 0 {
                let _ = crate::network::Network::get_puzzle_state(
                    url_key,
                    &mut player_state,
                    &network_channel,
                );
                return;
            }
        }
        Err(_) => (),
    }

    assert_eq!(player_state.inactives().len(), 0);
    let _ = player_state.set(PlayerState::Idle);
}

fn update_block(
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<MoveTimer>,
    mut game_query: Query<&mut GameState>,
    mut input_buffer: Query<&mut InputBuffer>,
    move_immediate: Query<&MoveImmediate>,
    mut player_info: Query<&mut PlayerInfo>,
    mut player_state: ResMut<State<PlayerState>>,
) {
    let mut game = game_query.single_mut();

    let timer_finished = move_timer.0.tick(time.delta()).just_finished();
    let elapsed_secs = move_timer.0.elapsed_secs();

    let mut new_move_flag = true;
    for arr in game.board.0.iter_mut() {
        for elem in arr {
            if let Some(block) = elem {
                if let Some((prev_transform, next_transform)) = block.moving {
                    // rotate block
                    let mut transform = transforms.get_mut(block.entity).unwrap();
                    if timer_finished {
                        *transform = next_transform;
                        block.moving = None;
                    } else {
                        transform.rotation = prev_transform
                            .rotation
                            .slerp(next_transform.rotation, elapsed_secs / BLOCK_MOVE_TIME);
                        let angle = PI / 4.0 + elapsed_secs / BLOCK_MOVE_TIME * PI / 2.0;
                        transform.translation = prev_transform.translation.lerp(
                            next_transform.translation,
                            -angle.cos() * (0.5 as f32).sqrt() + 0.5,
                        );
                        transform.translation.y = angle.sin() * (0.5 as f32).sqrt() - 0.5;
                        new_move_flag = false;
                    }
                }
            }
        }
    }

    if new_move_flag {
        if let Some(input) = input_buffer.single_mut().pop() {
            let state = player_state.current();
            match state {
                PlayerState::Shuffled => {
                    player_info.single_mut().add_move_count();
                    let _ = player_state.set(PlayerState::Solving);
                }
                PlayerState::Solving => player_info.single_mut().add_move_count(),
                PlayerState::Clear => {
                    let _ = player_state.set(PlayerState::Idle);
                }
                _ => {}
            }
            game.move_block(
                input.dx(),
                input.dy(),
                move_immediate.single().0,
                &mut move_timer,
                &mut transforms,
            );
        }
    }
}

fn check_clear(
    block_transforms: Query<&Transform, With<BlockMesh>>,
    mut app_state: ResMut<State<PlayerState>>,
    game_query: Query<&GameState>,
) {
    let game = game_query.single();

    if !game.is_shuffled {
        return;
    }

    let mut is_clear = true;
    for x in 0..game.size {
        for z in 0..game.size {
            let curr = (z * game.size + x + 1) as i32 % (game.size * game.size) as i32;
            if let Some(block) = &game.board.0[x][z] {
                if (block.goal == curr) && block.moving.is_none() {
                    if let Ok(block_transform) = block_transforms.get(block.entity) {
                        if !block_transform.rotation.is_near_identity() {
                            is_clear = false;
                            break;
                        }
                    }
                } else {
                    is_clear = false;
                    break;
                }
            } else {
                if (x != game.size - 1) || (z != game.size - 1) {
                    is_clear = false;
                    break;
                }
            }
        }
    }

    if is_clear {
        let _ = app_state.set(PlayerState::Clear);
    }
}
