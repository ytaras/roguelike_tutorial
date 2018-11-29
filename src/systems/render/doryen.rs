use doryen_rs::Console;
use specs::prelude::*;

use data::components::*;
use data::structures::*;

use super::Renderable;

pub struct DoryenRenderer<'a> {
    pub doryen_api: &'a mut Console,
}

impl<'a> System<'a> for DoryenRenderer<'a> {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, IsVisible>,
        Read<'a, LevelInfo>,
    );
    fn run(&mut self, (pos, vis, li): Self::SystemData) {
        use specs::Join;
        let con = &mut self.doryen_api;
        for (pos, vis) in li.all_cells() {
            render(*con, &pos, vis);
        }
        for (pos, vis) in (&pos, &vis).join() {
            render(*con, &pos, vis);
        }
    }
}

fn render<T: Renderable>(con: &mut Console, pos: &Pos, r: &T) {
    con.ascii(i32::from(pos.x), i32::from(pos.y), r.display_char() as u16);
    con.fore(i32::from(pos.x), i32::from(pos.y), r.color());
}
