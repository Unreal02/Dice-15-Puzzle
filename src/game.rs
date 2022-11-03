use std::{
    cmp::{max, min},
    f32::consts::PI,
    mem::swap,
};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    block::{spawn_meshes, Block, BlockState},
    player::PlayerState,
};

const BLOCK_MOVE_TIME: f32 = 0.3;

pub struct GamePlugin;

#[derive(Default, Inspectable, Component)]
pub struct GameState {
    x: i32,
    z: i32,
    board: Board,
}

#[derive(Default, Inspectable, Component)]
struct Board(Vec<Vec<Option<Block>>>);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(PlayerState::Playing).with_system(setup))
            .add_system_set(SystemSet::on_update(PlayerState::Playing).with_system(update_block));
    }
}

impl GameState {
    pub fn move_block(
        &mut self,
        dx: i32,
        dz: i32,
        mut move_timer: ResMut<Timer>,
        direction: KeyCode,
    ) {
        if self.x + dx < 0
            || self.x + dx > 3
            || self.z + dz < 0
            || self.z + dz > 3
            || !move_timer.finished()
        {
            return;
        }

        // translate block
        let (x0, z0) = (self.x as usize, self.z as usize);
        let (x1, z1) = ((self.x + dx) as usize, (self.z + dz) as usize);
        let block = self.board.0[x1][z1].as_mut().unwrap();
        block.moving = Some((dx, dz));
        block.state = block.state.transition(direction);
        *move_timer = Timer::from_seconds(BLOCK_MOVE_TIME, false);

        // swap self.board.0[x0][z0] and self.board.0[x1][z1]
        if x0 == x1 {
            let asdf = &mut self.board.0[x0];
            asdf.swap(z0, z1);
        } else {
            let (a1, a2) = self.board.0.split_at_mut(max(x0, x1));
            swap(&mut a1[min(x0, x1)][z0], &mut a2[0][z0]);
        }
        self.x += dx;
        self.z += dz;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut move_timer: ResMut<Timer>,
) {
    let mut new_game = GameState::default();

    let mesh_entities = spawn_meshes(&mut commands, meshes, materials, asset_server);

    // make board
    for x in 0..4 {
        new_game.board.0.push(Vec::new());
        for z in 0..4 {
            new_game.board.0[x].push(if x != 3 || z != 3 {
                Some(Block {
                    entity: mesh_entities.get(&(x, z)).unwrap().clone(),
                    goal: (z * 4 + x + 1) as i32 % 16,
                    moving: None,
                    state: BlockState::default(),
                })
            } else {
                None
            });
        }
    }
    new_game.x = 3;
    new_game.z = 3;
    *move_timer = Timer::from_seconds(0.1, false);

    // Spawn Game
    commands
        .spawn()
        .insert_bundle(SpatialBundle::default())
        .insert(new_game)
        .insert(Name::new("GAME"))
        .push_children(
            mesh_entities
                .iter()
                .map(|(_, v)| *v)
                .collect::<Vec<Entity>>()
                .as_slice(),
        );

    // light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
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
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(1.5, 5.0, 6.5).looking_at(
            Vec3 {
                x: 1.5,
                y: 0.0,
                z: 1.5,
            },
            Vec3::Y,
        ),
        ..default()
    });
}

fn update_block(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
    mut move_timer: ResMut<Timer>,
    mut game_query: Query<&mut GameState>,
) {
    let mut game = game_query.single_mut();

    let timer_finished = move_timer.tick(time.delta()).just_finished();
    let elapsed_secs = move_timer.elapsed_secs();
    for arr in game.board.0.iter_mut() {
        for elem in arr {
            if let Some(block) = elem {
                if let Some((dx, dz)) = block.moving {
                    let mut transform = transforms.get_mut(block.entity).unwrap();
                    let move_dist =
                        (2.0 as f32).sqrt() * 0.25 * PI * time.delta_seconds() / BLOCK_MOVE_TIME;
                    let move_angle = PI * 0.5 * (0.5 - elapsed_secs / BLOCK_MOVE_TIME);
                    transform.translation += Vec3 {
                        x: -dx as f32 * move_dist * move_angle.cos(),
                        y: move_dist * move_angle.sin(),
                        z: -dz as f32 * move_dist * move_angle.cos(),
                    };
                    transform
                        .rotate_x(-dz as f32 * PI * 0.5 * time.delta_seconds() / BLOCK_MOVE_TIME);
                    transform
                        .rotate_z(dx as f32 * PI * 0.5 * time.delta_seconds() / BLOCK_MOVE_TIME);
                    if timer_finished {
                        transform.translation =
                            Vec3::from_array(transform.translation.to_array().map(|i| i.round()));
                        transform.rotation =
                            Quat::from_array(transform.rotation.to_array().map(|i: f32| {
                                // rotation의 값은 0, 0.5, sqrt(2), 1 중 하나
                                let abs = i.abs();
                                let sgn = i.signum();
                                if abs < 0.25 {
                                    0.0
                                } else if abs < 0.6 {
                                    0.5 * sgn
                                } else if abs < 0.85 {
                                    (2.0 as f32).sqrt() * 0.5 * sgn
                                } else {
                                    1.0 * sgn
                                }
                            }));
                        block.moving = None;
                    }
                }
            }
        }
    }

    if keyboard_input.pressed(KeyCode::Up) {
        game.move_block(0, 1, move_timer, KeyCode::Up);
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.move_block(0, -1, move_timer, KeyCode::Down);
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.move_block(1, 0, move_timer, KeyCode::Left);
    } else if keyboard_input.pressed(KeyCode::Right) {
        game.move_block(-1, 0, move_timer, KeyCode::Right);
    }
}
