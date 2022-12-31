use bevy::prelude::*;

use crate::game::GameState;

#[derive(Default, Debug)]
pub struct BoardString([(u8, u8); 16]);

pub fn board_to_string(transforms: &Query<&mut Transform>, game: &mut GameState) -> BoardString {
    let mut board_string = BoardString::default();

    for z in 0..4 {
        for x in 0..4 {
            if let Some(block) = &game.board.0[x][z] {
                let i = z * 4 + x;
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

                board_string.0[i].0 = block.goal as u8;
                board_string.0[i].1 = rotation_byte;
            }
        }
    }

    board_string
}
