use doryen_rs::{DoryenApi, Engine};
use super::config::{WINDOW_WIDTH, WINDOW_HEIGHT};
use super::entities::Character;

pub struct Game {
    player: Character,
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
            self.player.pos.0 = (self.player.pos.0 - 1).max(1);
        }

        if input.key("ArrowRight") {
            self.player.pos.0 = (self.player.pos.0 + 1).min(WINDOW_WIDTH as i32 - 2);
        }

        if input.key("ArrowUp") {
            self.player.pos.1 = (self.player.pos.1 - 1).max(1);
        }

        if input.key("ArrowDown") {
            self.player.pos.1 = (self.player.pos.1 + 1).min(WINDOW_HEIGHT as i32 - 2);
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

        con.ascii(self.player.pos.0, self.player.pos.1, '@' as u16);
        con.fore(self.player.pos.0, self.player.pos.1, (255, 255, 255, 255));
    }

    fn resize(&mut self, _api: &mut DoryenApi) {}
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Character {
                pos: ((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32),
                health: 100,
            },
        }
    }
}
