use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{RenderArgs, UpdateArgs};

use crate::Object;

pub struct Tile {
    pub position: [f64; 2],
    pub rotation: f64,
    pub number: i32,
}

impl Tile {
    pub fn new(pos: [i32; 2], number: i32) -> Self {
        Tile {
            position: pos.map(|i| i as f64 * 75.0 + 37.5),
            rotation: 0.0,
            number,
        }
    }
}

impl Object for Tile {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, cache: &mut GlyphCache) {
        use graphics::*;

        let color: [f32; 4] = if self.number <= 4 {
            [1.0, 0.0, 0.0, 1.0]
        } else if self.number <= 8 {
            [1.0, 1.0, 0.0, 1.0]
        } else if self.number <= 12 {
            [0.0, 1.0, 0.0, 1.0]
        } else {
            [0.2, 0.2, 1.0, 1.0]
        };
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 60.0);
        let mut text = graphics::Text::new(30);
        text.color = BLACK;

        let rotation = self.rotation;
        let (x, y) = (self.position[0], self.position[1]);

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            let transform = c.transform.trans(x, y).rot_rad(rotation);

            if self.number > 0 {
                rectangle(color, square, transform.trans(-30.0, -30.0), gl);

                let text_trans_x = if self.number < 10 { -10.0 } else { -15.0 };
                text.draw(
                    self.number.to_string().as_str(),
                    cache,
                    &c.draw_state,
                    transform.trans(text_trans_x, 13.0),
                    gl,
                )
                .unwrap();
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
    fn keyboard_press(&mut self, _key: piston::Key) {}
}
