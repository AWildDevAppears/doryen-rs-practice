extern crate doryen_rs;

use doryen_rs::Console;

pub struct Character {
    pub pos: (i32, i32),
    pub health: i32,
    pub sprite: u16,
}

impl Character {
    pub fn draw_sprite(&mut self, con: &mut Console) {
        con.ascii(self.pos.0, self.pos.1, self.sprite);
        con.fore(self.pos.0, self.pos.1, (255, 255, 255, 255));
    }

    pub fn move_left(&mut self, speed: i32, max: i32) {
        self.pos.0 = (self.pos.0 - speed).max(max);
    }

    pub fn move_right(&mut self, speed: i32, min: i32) {
        self.pos.0 = (self.pos.0 + speed).min(min);
    }

    pub fn move_up(&mut self, speed: i32, max: i32) {
        self.pos.1 = (self.pos.1 - speed).max(max);
    }

    pub fn move_down(&mut self, speed: i32, min: i32) {
        self.pos.1 = (self.pos.1 + speed).min(min);
    }
}
