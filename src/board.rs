use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Key, RenderArgs, UpdateArgs};

use crate::{Object, Tile};

struct Position {
    x: i32,
    y: i32,
}

pub struct Board {
    inner: [[Tile; 4]; 4],
    pos: Position,
}

impl Board {
    pub fn new() -> Self {
        let arr: [[Tile; 4]; 4] = [
            [
                Tile::new([0, 0], 1),
                Tile::new([1, 0], 2),
                Tile::new([2, 0], 3),
                Tile::new([3, 0], 4),
            ],
            [
                Tile::new([0, 1], 5),
                Tile::new([1, 1], 6),
                Tile::new([2, 1], 7),
                Tile::new([3, 1], 8),
            ],
            [
                Tile::new([0, 2], 9),
                Tile::new([1, 2], 10),
                Tile::new([2, 2], 11),
                Tile::new([3, 2], 12),
            ],
            [
                Tile::new([0, 3], 13),
                Tile::new([1, 3], 14),
                Tile::new([2, 3], 15),
                Tile::new([3, 3], 0),
            ],
        ];
        Board {
            inner: arr,
            pos: Position { x: 3, y: 3 },
        }
    }

    fn move_tile(&mut self, dx: i32, dy: i32) {
        let (x, y) = (self.pos.x, self.pos.y);
        if x + dx >= 0 && x + dx < 4 && y + dy >= 0 && y + dy < 4 {
            let (x0, y0) = (x as usize, y as usize);
            let (x1, y1) = ((x + dx) as usize, (y + dy) as usize);
            self.inner[x0][y0].number = self.inner[x1][y1].number;
            self.inner[x1][y1].number = 0;
            self.pos.x += dx;
            self.pos.y += dy;
        }
    }

    fn reset(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.inner[i][j].number = ((i * 4 + j + 1) % 16) as i32;
            }
            self.pos = Position { x: 3, y: 3 };
        }
    }

    fn shuffle(&mut self) {}
}

impl Object for Board {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, cache: &mut GlyphCache) {
        for x in self.inner.iter() {
            for square in x.iter() {
                square.render(args, gl, cache);
            }
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {}

    fn keyboard_press(&mut self, key: Key) {
        match key {
            Key::Up => self.move_tile(-1, 0),
            Key::Down => self.move_tile(1, 0),
            Key::Left => self.move_tile(0, -1),
            Key::Right => self.move_tile(0, 1),
            Key::R => self.reset(),
            Key::S => self.shuffle(),
            _ => (),
        }
    }
}
