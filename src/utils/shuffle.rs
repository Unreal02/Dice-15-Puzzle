use bevy::prelude::*;
use rand::random;
use std::f32::consts::PI;

use crate::network::BoardString;

const SHUFFLE_NUMBER: i32 = 1000;

pub fn shuffle(size: usize) -> BoardString {
    #[derive(Clone, Copy, Debug)]
    struct Block {
        pub goal: i32,
        pub transform: Transform,
    }

    // shuffle settings
    let mut board = Vec::new();
    let mut x = size as i32 - 1;
    let mut z = size as i32 - 1;

    for x in 0..size {
        board.push(Vec::new());
        for z in 0..size {
            board[x].push(if x != size - 1 || z != size - 1 {
                Some(Block {
                    goal: (z * size + x + 1) as i32,
                    transform: Transform::IDENTITY,
                })
            } else {
                None
            });
        }
    }

    // move_block function
    let mut move_block = |dx: i32, dz: i32| {
        if x + dx < 0 || x + dx >= size as i32 || z + dz < 0 || z + dz >= size as i32 {
            return;
        }

        let x0 = x as usize;
        let z0 = z as usize;
        let x1 = (x + dx) as usize;
        let z1 = (z + dz) as usize;

        let mut block = board[x1][z1].unwrap();
        block.transform.rotate_x(-dz as f32 * PI * 0.5);
        block.transform.rotate_z(dx as f32 * PI * 0.5);
        board[x1][z1] = Some(block);

        let t = board[x0][z0];
        board[x0][z0] = board[x1][z1];
        board[x1][z1] = t;

        x += dx;
        z += dz;
    };

    // shuffle
    for _ in 0..SHUFFLE_NUMBER {
        match random::<u32>() % 4 {
            0 => move_block(0, 1),
            1 => move_block(0, -1),
            2 => move_block(1, 0),
            3 => move_block(-1, 0),
            _ => unreachable!(),
        }
    }

    // convert to BoardString
    let mut board_string = BoardString::new(size);

    for z in 0..size {
        (0..size).for_each(|x| {
            if let Some(block) = board[x][z] {
                let i = block.goal as usize;
                let rotation = block.transform.rotation;

                let rotation_byte = rotation.to_array().iter().fold(0_u8, |cur, &i| {
                    (cur << 2)
                        | if i > 0.3 {
                            1
                        } else if i < -0.3 {
                            3
                        } else {
                            0
                        }
                });

                board_string.0[i].0 = (z * size + x + 1) as u8;
                board_string.0[i].1 = rotation_byte;
            } else {
                board_string.0[0].0 = (z * size + x + 1) as u8;
            }
        });
    }

    board_string
}
