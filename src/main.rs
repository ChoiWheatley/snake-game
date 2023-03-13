use piston_window::{clear, color, rectangle, PistonWindow, WindowSettings};

extern crate piston_window;
extern crate rand;

mod draw;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello, Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .expect("window build failed");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                color::RED,
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
