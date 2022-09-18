use graphics::Transformed;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Key, RenderArgs, UpdateArgs};

use crate::Object;

pub struct InfoText {
    text: Vec<String>,
}

impl InfoText {
    pub fn new() -> Self {
        InfoText {
            text: vec![
                String::from("R: Reset"),
                String::from("S: Shuffle"),
                String::from("Esc: Quit"),
            ],
        }
    }
}

impl Object for InfoText {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, cache: &mut GlyphCache) {
        gl.draw(args.viewport(), |c, gl| {
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            let transform = c.transform.trans(7.5, 320.0);
            let mut text = graphics::Text::new(20);
            text.color = WHITE;
            for (i, t) in self.text.iter().enumerate() {
                text.draw(
                    t,
                    cache,
                    &c.draw_state,
                    transform.trans(0.0, i as f64 * 20.0),
                    gl,
                )
                .unwrap();
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}

    fn keyboard_press(&mut self, _key: Key) {}
}
