    pub const WINDOW_WIDTH: u32 = 80;
pub const WINDOW_HEIGHT: u32 = 45;
pub const ICON_WIDTH: u32 = 10;
pub const ICON_HEIGHT: u32 = 16;

extern crate doryen_rs;

use doryen_rs::{DoryenApi, Engine};

pub struct Game {
    player_pos: (i32, i32),
}

impl Engine for Game {
    fn init(&mut self, api: &mut DoryenApi) {
        api.con().register_color("white", (255, 255, 255, 255));
        api.con().register_color("red", (255, 92, 92, 255));
        api.con().register_color("blue", (192, 192, 255, 255));
    }

    fn update(&mut self, api: &mut DoryenApi) {
        let input = api.input();

        if input.key("ArrowLeft") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
        }

        if input.key("ArrowRight") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(WINDOW_WIDTH as i32 - 2);
        }

        if input.key("ArrowUp") {
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        }

        if input.key("ArrowDown") {
            self.player_pos.1 = (self.player_pos.1 + 1).min(WINDOW_HEIGHT as i32 - 2);
        }
    }

    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();

        con.rectangle(
            0,
            0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            Some((128, 128, 128, 255)),
            None,
            Some('.' as u16),
        );

        con.area(
            10,
            10,
            5,
            5,
            Some((255, 64, 64, 255)),
            Some((128, 32, 32, 255)),
            Some('&' as u16),
        );

        con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
        con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
    }

    fn resize(&mut self, _api: &mut DoryenApi) {}
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_pos: ((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32),
        }
    }
}
