use bevy::{math::vec3, prelude::*, utils::HashMap};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::{Highlighting, PickableBundle};

#[derive(Component)]
pub struct BlockMesh;

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Inspectable))]
pub struct Block {
    pub entity: Entity,
    pub moving: Option<(Transform, Transform)>, // previous and next transform
    /// (z * 4 + x + 1) as i32 % 16
    pub goal: i32,
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
                let material = materials.add(StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    base_color: match z {
                        0 => Color::hsl(0.0, 1.0, 0.6),
                        1 => Color::hsl(60.0, 1.0, 0.6),
                        2 => Color::hsl(120.0, 1.0, 0.6),
                        3 => Color::hsl(240.0, 1.0, 0.6),
                        _ => Color::BLACK,
                    },
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
