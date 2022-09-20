use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Key, RenderArgs};
use rand::{self, Rng};

use crate::Tile;

#[derive(Default)]
pub struct Board {
    inner: [[Tile; 4]; 4],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                board.inner[i][j].position = [j as f64 * 75.0 + 37.5, i as f64 * 75.0 + 37.5];
                board.inner[i][j].number = ((i * 4 + j + 1) % 16) as i32;
            }
        }
        board
    }

    fn move_tile(&mut self, dx: i32, dy: i32) {
        let (px, py): (i32, i32) = (|| {
            for i in 0..4 {
                for j in 0..4 {
                    if self.inner[i][j].number == 0 {
                        return (i as i32, j as i32);
                    }
                }
            }
            return (0, 0);
        })();
        let (x, y) = (px, py);
        if x + dx >= 0 && x + dx < 4 && y + dy >= 0 && y + dy < 4 {
            let (x0, y0) = (x as usize, y as usize);
            let (x1, y1) = ((x + dx) as usize, (y + dy) as usize);
            self.inner[x0][y0].number = self.inner[x1][y1].number;
            self.inner[x1][y1].number = 0;
        }
    }

    fn reset(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.inner[i][j].number = ((i * 4 + j + 1) % 16) as i32;
            }
        }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];

        // shuffle
        for i in 0..15 {
            let j = rng.gen_range(i..15);
            arr.swap(i, j);
        }

        // count inversion
        let mut inv = 0;
        for i in 0..15 {
            for j in i + 1..15 {
                if arr[i] > arr[j] {
                    inv += 1;
                }
            }
        }
        if inv % 2 != 0 {
            arr.swap(0, 1);
        }

        // copy to self.inner
        for i in 0..4 {
            for j in 0..4 {
                self.inner[i][j].number = arr[i * 4 + j];
            }
        }

        // randomize empty space
        let x = rng.gen_range(0..4);
        let y = rng.gen_range(0..4);
        for _ in 0..x {
            self.move_tile(-1, 0);
        }
        for _ in 0..y {
            self.move_tile(0, -1);
        }
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, cache: &mut GlyphCache) {
        for x in self.inner.iter() {
            for square in x.iter() {
                square.render(args, gl, cache);
            }
        }
    }

    pub fn keyboard_press(&mut self, key: Key) {
        match key {
            Key::Up => self.move_tile(-1, 0),
            Key::W => self.move_tile(-1, 0),
            Key::Down => self.move_tile(1, 0),
            Key::S => self.move_tile(1, 0),
            Key::Left => self.move_tile(0, -1),
            Key::A => self.move_tile(0, -1),
            Key::Right => self.move_tile(0, 1),
            Key::D => self.move_tile(0, 1),
            Key::R => self.shuffle(),
            Key::V => self.reset(),
            _ => (),
        }
    }
}
