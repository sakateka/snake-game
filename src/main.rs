mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::Game;

use std::env;
use std::io;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
fn main() -> io::Result<()> {
    let map = env::args().nth(1)
        .unwrap_or("maps/1.txt".to_string());
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    game.load_map(&map)?;

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
    Ok(())
}
