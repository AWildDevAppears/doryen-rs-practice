extern crate doryen_rs;

mod game;
mod config;
mod entities;

use doryen_rs::{App, AppOptions};
use config::{SCREEN_WIDTH, SCREEN_HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT};

fn main() {
    let mut app = App::new(AppOptions {
        console_width: WINDOW_WIDTH,
        console_height: WINDOW_HEIGHT,
        screen_width: SCREEN_WIDTH,
        screen_height: SCREEN_HEIGHT,
        window_title: "Roguelike".to_owned(),
        font_path: "terminal_10x16.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(game::Game::new()));
    app.run();
}
