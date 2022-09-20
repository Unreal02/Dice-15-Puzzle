use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, PressEvent};
use std::path::Path;

mod board;
mod info_text;
mod tile;

pub use board::Board;
pub use info_text::InfoText;
pub use tile::Tile;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("dice-15-puzzle", [300, 370])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let font_path = Path::new("assets/Quicksand-Bold.ttf");
    let settings = TextureSettings::new();
    let ref mut cache = GlyphCache::new(font_path, (), settings).unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut board = Board::new();
    let info_text = InfoText::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            graphics::clear(BLACK, &mut gl);
            board.render(&args, &mut gl, cache);
            info_text.render(&args, &mut gl, cache);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            board.keyboard_press(key);
        }
    }
}
