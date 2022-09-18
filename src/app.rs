use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::{
    input::{RenderArgs, UpdateArgs},
    Key,
};

use crate::Object;

pub struct App {
    gl: GlGraphics,
    objects: Vec<Box<dyn Object>>,
}

impl App {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        let opengl = OpenGL::V3_2;
        App {
            gl: GlGraphics::new(opengl),
            objects,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, cache: &mut GlyphCache) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        graphics::clear(BLACK, &mut self.gl);
        for obj in &self.objects {
            obj.render(args, &mut self.gl, cache);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for obj in &mut self.objects {
            obj.update(args);
        }
    }

    pub fn keyboard_press(&mut self, key: Key) {
        for obj in &mut self.objects {
            obj.keyboard_press(key);
        }
    }
}
