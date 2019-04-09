use doryen_rs::{DoryenApi, Engine};
use super::config::{WINDOW_WIDTH, WINDOW_HEIGHT, MOVE_SPEED};
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
            self.player.move_left(MOVE_SPEED, 1);
        }

        if input.key("ArrowRight") {
            self.player.move_right(MOVE_SPEED, WINDOW_WIDTH as i32 - 2);
        }

        if input.key("ArrowUp") {
            self.player.move_up(MOVE_SPEED, 1);
        }

        if input.key("ArrowDown") {
            self.player.move_down(MOVE_SPEED, WINDOW_HEIGHT as i32 - 2);
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

        self.player.draw_sprite(con);
    }

    fn resize(&mut self, _api: &mut DoryenApi) {}
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Character {
                pos: ((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32),
                health: 100,
                sprite: '@' as u16,
            },
        }
    }
}
