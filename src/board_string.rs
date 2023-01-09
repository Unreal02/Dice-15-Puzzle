use bevy::prelude::*;

use crate::game::GameState;

#[derive(Default, Debug)]
/// (position, rotation)
/// index: number written on block (0 means empty)
pub struct BoardString([(u8, u8); 16]);

pub fn board_to_string(transforms: &Query<&mut Transform>, game: &mut GameState) -> BoardString {
    let mut board_string = BoardString::default();

    for z in 0..4 {
        for x in 0..4 {
            if let Some(block) = &game.board.0[x][z] {
                let i = block.goal as usize;
                let rotation = transforms.get(block.entity).unwrap().rotation;

                let rotation_byte = rotation.to_array().iter().fold(0 as u8, |cur, &i| {
                    (cur << 2)
                        | if i > 0.3 {
                            1
                        } else if i < -0.3 {
                            3
                        } else {
                            0
                        }
                });

                board_string.0[i].0 = (z * 4 + x + 1) as u8;
                board_string.0[i].1 = rotation_byte;
            } else {
                board_string.0[0].0 = (z * 4 + x + 1) as u8;
            }
        }
    }

    board_string
}

pub fn string_to_board(
    board_string: BoardString,
    transforms: &mut Query<&mut Transform>,
    game: &mut GameState,
) {
    for z in 0..4 {
        for x in 0..4 {
            if let Some(block) = &game.board.0[x][z] {
                let (position, rotation) = board_string.0[block.goal as usize];
                let mut transform = transforms.get_mut(block.entity).unwrap();
                transform.translation = Vec3::new(
                    ((position - 1) % 4) as f32,
                    0 as f32,
                    ((position - 1) / 4) as f32,
                );
                let mut rotation_arr = [0.0; 4];
                for i in 0..4 {
                    let bits = rotation >> ((3 - i) * 2); // 0 ~ 3
                    let num = (bits & 1) as f32;
                    let sign = -((bits & 2) as f32) + 1.0; // 1 or -1
                    rotation_arr[i] = sign * num;
                }
                transform.rotation = Quat::from_vec4(Vec4::from(rotation_arr).normalize());
            } else {
                let position = board_string.0[0].0;
                game.x = ((position - 1) % 4) as i32;
                game.z = ((position - 1) / 4) as i32;
            }
        }
    }

    game.is_shuffled = true;
}
