use std::path::Path;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent};

mod app;
mod board;
mod info_text;
mod object;
mod tile;

pub use app::App;
pub use board::Board;
pub use info_text::InfoText;
pub use object::Object;
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

    // Create a new game and run it.
    let mut app = App::new(vec![Box::new(Board::new()), Box::new(InfoText::new())]);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, cache);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.keyboard_press(key);
        }
    }
}
