use bevy::{math::vec3, prelude::*, utils::HashMap};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::{Highlighting, PickableBundle};

#[derive(Component)]
pub struct BlockMesh;

#[derive(Component, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct Block {
    pub entity: Entity,
    pub moving: Option<(Transform, Transform)>, // previous and next transform
    /// (z * size + x + 1) as i32 % (size * size)
    pub goal: i32,
}

/// Spawn Mesh for blocks and return entity ids
/// This function must be called in game setup
pub fn spawn_meshes(
    commands: &mut Commands,
    size: usize,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> HashMap<(usize, usize), Entity> {
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    let uv_modified = vec![
        // +z
        [1.0 / 4.0, 1.0],
        [2.0 / 4.0, 1.0],
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
        [1.0, 2.0 / 3.0],
        [1.0, 1.0 / 3.0],
        [3.0 / 4.0, 1.0 / 3.0],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_modified);
    let cube_mesh = meshes.add(mesh);

    let mut mesh_entities = HashMap::new();
    for x in 0..size {
        for z in 0..size {
            if x != size - 1 || z != size - 1 {
                let texture =
                    asset_server.load(format!("images/image{}.png", x + z * size + 1).as_str());
                let material = materials.add(StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    base_color: Color::hsl(360.0 * z as f32 / size as f32, 1.0, 0.6),
                    ..default()
                });
                mesh_entities.insert(
                    (x, z),
                    commands
                        .spawn((
                            PbrBundle {
                                mesh: cube_mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_translation(vec3(
                                    x as f32, 0.0, z as f32,
                                )),
                                ..default()
                            },
                            PickableBundle::default(),
                            Highlighting {
                                initial: material.clone(),
                                hovered: Some(material.clone()),
                                pressed: Some(material.clone()),
                                selected: Some(material.clone()),
                            },
                            BlockMesh,
                        ))
                        .id(),
                );
            }
        }
    }

    mesh_entities
}
