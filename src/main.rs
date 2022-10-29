use std::{
    cmp::{max, min},
    f32::consts::PI,
    mem::swap,
};

use bevy::{prelude::*, utils::HashMap, DefaultPlugins};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

const BLOCK_MOVE_TIME: f32 = 0.3;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayState {
    Playing,
    GameClear,
}

fn main() {
    App::new()
        .init_resource::<Timer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<GameState>()
        .register_inspectable::<Board>()
        .add_state(PlayState::Playing)
        .add_system_set(SystemSet::on_enter(PlayState::Playing).with_system(setup))
        .add_system_set(SystemSet::on_update(PlayState::Playing).with_system(update_block))
        .run();
}

#[derive(Default, Inspectable, Component)]
struct GameState {
    x: i32,
    z: i32,
    board: Board,
}

#[derive(Default, Inspectable, Component)]
struct Board(Vec<Vec<Option<Block>>>);

impl GameState {
    pub fn move_block(&mut self, dx: i32, dz: i32, mut move_timer: ResMut<Timer>) {
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

#[derive(Inspectable, Component)]
struct Block {
    entity: Entity,
    number: i32,
    moving: Option<(i32, i32)>, // (dx, dz)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut move_timer: ResMut<Timer>,
) {
    let mut new_game = GameState::default();

    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    let uv_modified = vec![
        // +z
        [1.0 / 4.0, 3.0 / 3.0],
        [2.0 / 4.0, 3.0 / 3.0],
        [2.0 / 4.0, 2.0 / 3.0],
        [1.0 / 4.0, 2.0 / 3.0],
        // -z
        [1.0 / 4.0, 1.0 / 3.0],
        [2.0 / 4.0, 1.0 / 3.0],
        [2.0 / 4.0, 0.0 / 3.0],
        [1.0 / 4.0, 0.0 / 3.0],
        // +x
        [3.0 / 4.0, 1.0 / 3.0],
        [2.0 / 4.0, 1.0 / 3.0],
        [2.0 / 4.0, 2.0 / 3.0],
        [3.0 / 4.0, 2.0 / 3.0],
        // -x
        [0.0 / 4.0, 2.0 / 3.0],
        [1.0 / 4.0, 2.0 / 3.0],
        [1.0 / 4.0, 1.0 / 3.0],
        [0.0 / 4.0, 1.0 / 3.0],
        // +y
        [2.0 / 4.0, 1.0 / 3.0],
        [1.0 / 4.0, 1.0 / 3.0],
        [1.0 / 4.0, 2.0 / 3.0],
        [2.0 / 4.0, 2.0 / 3.0],
        // -y
        [3.0 / 4.0, 2.0 / 3.0],
        [4.0 / 4.0, 2.0 / 3.0],
        [4.0 / 4.0, 1.0 / 3.0],
        [3.0 / 4.0, 1.0 / 3.0],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_modified);
    let cube_mesh = meshes.add(mesh);

    let mut mesh_entities = HashMap::new();
    for x in 0..4 {
        for z in 0..4 {
            if x != 3 || z != 3 {
                let texture =
                    asset_server.load(format!("images/image{}.png", x + z * 4 + 1).as_str());
                mesh_entities.insert(
                    (x, z),
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: cube_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color_texture: Some(texture.clone()),
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
                );
            }
        }
    }

    // make board
    for x in 0..4 {
        new_game.board.0.push(Vec::new());
        for z in 0..4 {
            new_game.board.0[x].push(if x != 3 || z != 3 {
                Some(Block {
                    entity: mesh_entities.get(&(x, z)).unwrap().clone(),
                    number: (x * 4 + z + 1) as i32 % 16,
                    moving: None,
                })
            } else {
                None
            });
        }
    }
    new_game.x = 3;
    new_game.z = 3;
    *move_timer = Timer::from_seconds(0.1, false);

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
        game.move_block(0, 1, move_timer);
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.move_block(0, -1, move_timer);
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.move_block(1, 0, move_timer);
    } else if keyboard_input.pressed(KeyCode::Right) {
        game.move_block(-1, 0, move_timer);
    }
}
