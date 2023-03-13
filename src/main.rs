use draw::to_coord;
use game::Game;
use piston_window::{
    clear, types::Color, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let width: i32 = 20;
    let height: i32 = 20;

    let mut window: PistonWindow =
        WindowSettings::new("Snake game", [to_coord(width), to_coord(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |context, g, _device| {
            clear(BACK_COLOR, g);
            game.draw(&context, g);
        });
        event.update(|arg| game.update(arg.dt));
    }
}
