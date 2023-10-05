extern crate piston_window;

use piston_window::types::Color;
use piston_window::*;

const BG_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    let window_settings = WindowSettings::new("Rust Snake", [800, 600]).exit_on_esc(true);

    let mut window: PistonWindow = window_settings.build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |_, g, _| clear(BG_COLOR, g));
    }

    println!("Hello, world!");
}
