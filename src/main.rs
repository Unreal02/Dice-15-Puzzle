use std::{
    cmp::{max, min},
    mem::swap,
};

use bevy::{input::keyboard, prelude::*, scene::SceneBundle, DefaultPlugins};

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
}

impl Game {
    pub fn move_block(&mut self, dx: i32, dz: i32, mut transforms: Query<&mut Transform>) {
        if self.x + dx < 0 || self.x + dx > 3 || self.z + dz < 0 || self.z + dz > 3 {
            return;
        }

        // translate block
        let (x0, z0) = (self.x as usize, self.z as usize);
        let (x1, z1) = ((self.x + dx) as usize, (self.z + dz) as usize);
        let block = self.board[x1][z1].as_ref().unwrap();
        let mut asdf = transforms.get_mut(block.entity).unwrap();
        asdf.translation += Vec3 {
            x: -dx as f32,
            y: 0.0,
            z: -dz as f32,
        };

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
                                    red: z as f32 / 4.0,
                                    green: x as f32 / 4.0,
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
                })
            } else {
                None
            });
        }
    }
    game.x = 3;
    game.z = 3;

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
    mut game: ResMut<Game>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
) {
    // for arr in game.board.iter() {
    //     for block in arr {
    //         transforms
    //             .get_mut(block.entity)
    //             .unwrap()
    //             .rotate_y(block.number as f32 / 100.0);
    //     }
    // }
    if keyboard_input.just_pressed(KeyCode::Up) {
        game.move_block(0, -1, transforms);
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        game.move_block(0, 1, transforms);
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        game.move_block(-1, 0, transforms);
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        game.move_block(1, 0, transforms);
    }
}
