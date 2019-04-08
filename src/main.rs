extern crate doryen_rs;

use doryen_rs::{App, AppOptions};

mod game;

fn main() {
    let mut app = App::new(AppOptions {
        console_width: game::WINDOW_WIDTH,
        console_height: game::WINDOW_HEIGHT,
        screen_width: game::WINDOW_WIDTH * game::ICON_WIDTH,
        screen_height: game::WINDOW_HEIGHT * game::ICON_HEIGHT,
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
