use std::{
    cmp::{max, min},
    f32::consts::PI,
    mem::swap,
};

use bevy::{prelude::*, DefaultPlugins};

const BLOCK_MOVE_TIME: f32 = 0.5;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameClear,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_block))
        .run();
}

#[derive(Default)]
struct Game {
    board: Vec<Vec<Option<Block>>>,
    x: i32,
    z: i32,
    move_timer: Timer,
}

impl Game {
    pub fn move_block(&mut self, dx: i32, dz: i32) {
        if self.x + dx < 0
            || self.x + dx > 3
            || self.z + dz < 0
            || self.z + dz > 3
            || !self.move_timer.finished()
        {
            return;
        }

        // translate block
        let (x0, z0) = (self.x as usize, self.z as usize);
        let (x1, z1) = ((self.x + dx) as usize, (self.z + dz) as usize);
        let block = self.board[x1][z1].as_mut().unwrap();
        block.moving = Some((dx, dz));
        self.move_timer = Timer::from_seconds(BLOCK_MOVE_TIME, false);

        // swap self.board[x0][z0] and self.board[x1][z1]
        if x0 == x1 {
            let asdf = &mut self.board[x0];
            asdf.swap(z0, z1);
        } else {
            let (a1, a2) = self.board.split_at_mut(max(x0, x1));
            swap(&mut a1[min(x0, x1)][z0], &mut a2[0][z0]);
        }
        self.x += dx;
        self.z += dz;
    }
}

struct Block {
    entity: Entity,
    number: i32,
    moving: Option<(i32, i32)>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    // make board
    for x in 0..4 {
        game.board.push(Vec::new());
        for z in 0..4 {
            game.board[x].push(if x != 3 || z != 3 {
                Some(Block {
                    entity: commands
                        .spawn_bundle(PbrBundle {
                            mesh: cube_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: Color::Rgba {
                                    red: z as f32 / 3.0,
                                    green: x as f32 / 3.0,
                                    blue: 0.5,
                                    alpha: 0.5,
                                },
                                ..default()
                            }),
                            transform: Transform::from_translation(Vec3::new(
                                x as f32, 0.0, z as f32,
                            )),
                            ..default()
                        })
                        .id(),
                    number: (x * 4 + z + 1) as i32 % 16,
                    moving: None,
                })
            } else {
                None
            });
        }
    }
    game.x = 3;
    game.z = 3;
    game.move_timer = Timer::from_seconds(0.1, false);

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
    mut game: ResMut<Game>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
) {
    let timer_finished = game.move_timer.tick(time.delta()).just_finished();
    let elapsed_secs = game.move_timer.elapsed_secs();
    for arr in game.board.iter_mut() {
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
                        let rotation_round = |i: f32| {
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
                        };
                        transform.translation.x = transform.translation.x.round();
                        transform.translation.y = transform.translation.y.round();
                        transform.translation.z = transform.translation.z.round();
                        transform.rotation.x = rotation_round(transform.rotation.x);
                        transform.rotation.y = rotation_round(transform.rotation.y);
                        transform.rotation.z = rotation_round(transform.rotation.z);
                        transform.rotation.w = rotation_round(transform.rotation.w);
                        block.moving = None;
                    }
                }
            }
        }
    }

    if keyboard_input.pressed(KeyCode::Up) {
        game.move_block(0, 1);
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.move_block(0, -1);
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.move_block(1, 0);
    } else if keyboard_input.pressed(KeyCode::Right) {
        game.move_block(-1, 0);
    }
}
