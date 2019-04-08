pub struct Character {
    pub pos: (i32, i32),
    pub health: i32,
    pub sprite: u16,
}
extern crate doryen_rs;

use doryen_rs::Console;

impl Character {
     pub fn draw_sprite(&mut self, con: &mut Console) {
        con.ascii(self.pos.0, self.pos.1, self.sprite);
        con.fore(self.pos.0, self.pos.1, (255, 255, 255, 255));
    }
}
