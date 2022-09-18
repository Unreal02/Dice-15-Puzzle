use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Key, RenderArgs, UpdateArgs};

pub trait Object {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, cache: &mut GlyphCache);
    fn update(&mut self, args: &UpdateArgs);
    fn keyboard_press(&mut self, key: Key);
}
