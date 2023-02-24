use bevy::{math::vec3, prelude::*};

use crate::{block::Block, game::GameState, network::BoardString};

pub fn board_to_string(transforms: &Query<&mut Transform>, game: &mut GameState) -> BoardString {
    let mut board_string = BoardString::new(game.size);

    for z in 0..game.size {
        for x in 0..game.size {
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

                board_string.0[i].0 = (z * game.size + x + 1) as u8;
                board_string.0[i].1 = rotation_byte;
            } else {
                board_string.0[0].0 = (z * game.size + x + 1) as u8;
            }
        }
    }

    board_string
}

pub fn string_to_board(
    board_string: &BoardString,
    transforms: &mut Query<&mut Transform>,
    game: &mut GameState,
) {
    let mut temp_vec: Vec<Option<Block>> = vec![None; game.size * game.size];

    for z in 0..game.size {
        for x in 0..game.size {
            let index = if let Some(block) = &game.board.0[x][z] {
                block.goal
            } else {
                0
            };
            temp_vec[index as usize] = game.board.0[x][z];
        }
    }

    game.x = (board_string.0[0].0 as i32 - 1) % game.size as i32;
    game.z = (board_string.0[0].0 as i32 - 1) / game.size as i32;
    game.board.0[game.x as usize][game.z as usize] = None;
    for i in 1..game.size * game.size {
        let block = temp_vec[i].expect("temp_arr wrong");
        let (position, rotation) = board_string.0[block.goal as usize];
        let x = ((position - 1) % game.size as u8) as usize;
        let z = ((position - 1) / game.size as u8) as usize;
        let mut transform = transforms.get_mut(block.entity).unwrap();
        transform.translation = vec3(x as f32, 0 as f32, z as f32);
        let mut rotation_arr = [0.0; 4];
        for i in 0..4 {
            let bits = rotation >> ((3 - i) * 2); // 0, 1, or 3
            let num = (bits & 1) as f32;
            let sign = -((bits & 2) as f32) + 1.0; // 1 or -1
            rotation_arr[i] = sign * num;
        }
        transform.rotation = Quat::from_vec4(Vec4::from(rotation_arr).normalize());
        game.board.0[x][z] = temp_vec[i];
    }

    game.is_shuffled = true;
}
