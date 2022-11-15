use bevy::{math::vec3, prelude::*, utils::HashMap};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct Block {
    pub entity: Entity,
    pub moving: Option<(Transform, Transform)>, // previous and next transform
    /// (z * 4 + x + 1) as i32 % 16
    pub goal: i32,
    pub state: BlockState,
}

/// Represent the block's upper side state
/// Dice required to be rotated in stated direction to achieve goal state
#[derive(Component, Clone, Copy, Debug)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub enum BlockState {
    Goal,
    Left,
    Right,
    Up,
    Down,
    Back,
}

impl Default for BlockState {
    fn default() -> Self {
        BlockState::Goal
    }
}

impl BlockState {
    pub fn transition(self, direction: KeyCode) -> Self {
        match direction {
            KeyCode::Up => match self {
                BlockState::Goal => BlockState::Down,
                BlockState::Up => BlockState::Goal,
                BlockState::Down => BlockState::Back,
                BlockState::Back => BlockState::Up,
                _ => self,
            },
            KeyCode::Down => match self {
                BlockState::Goal => BlockState::Up,
                BlockState::Up => BlockState::Back,
                BlockState::Down => BlockState::Goal,
                BlockState::Back => BlockState::Down,
                _ => self,
            },
            KeyCode::Left => match self {
                BlockState::Goal => BlockState::Right,
                BlockState::Left => BlockState::Goal,
                BlockState::Right => BlockState::Back,
                BlockState::Back => BlockState::Left,
                _ => self,
            },
            KeyCode::Right => match self {
                BlockState::Goal => BlockState::Left,
                BlockState::Left => BlockState::Back,
                BlockState::Right => BlockState::Goal,
                BlockState::Back => BlockState::Right,
                _ => self,
            },
            _ => unreachable!("Block transition should be done with direction keyboard input"),
        }
    }
}

/// Spawn Mesh for blocks and return entity ids
/// This function must be called in game setup
pub fn spawn_meshes(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> HashMap<(usize, usize), Entity> {
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
                        .spawn(PbrBundle {
                            mesh: cube_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color_texture: Some(texture.clone()),
                                base_color: match z {
                                    0 => Color::hsl(0.0, 1.0, 0.6),
                                    1 => Color::hsl(60.0, 1.0, 0.6),
                                    2 => Color::hsl(120.0, 1.0, 0.6),
                                    3 => Color::hsl(240.0, 1.0, 0.6),
                                    _ => Color::BLACK,
                                },
                                ..default()
                            }),
                            transform: Transform::from_translation(vec3(x as f32, 0.0, z as f32)),
                            ..default()
                        })
                        .id(),
                );
            }
        }
    }

    mesh_entities
}
