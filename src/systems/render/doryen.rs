use super::{Renderable, WHITE};
use data::components::*;
use data::structures::*;
use doryen_rs::{Console, DoryenApi};
use specs::prelude::*;

pub struct DoryenRenderer<'a> {
    pub doryen_api: &'a mut DoryenApi,
}

impl<'a> System<'a> for DoryenRenderer<'a> {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, IsVisible>,
        Read<'a, LevelInfo>,
    );
    fn run(&mut self, (pos, vis, li): Self::SystemData) {
        use specs::Join;
        let con = self.doryen_api.con();
        for (pos, vis) in li.all_cells() {
            render(con, &pos, vis);
        }
        // con.area(
        //     0,
        //     0,
        //     li.width().into(),
        //     li.height().into(),
        //     Some(WHITE),
        //     None,
        //     Some('.' as u16),
        // );
        for (pos, vis) in (&pos, &vis).join() {
            render(con, &pos, vis);
        }
    }
}

fn render<T: Renderable>(con: &mut Console, pos: &Pos, r: &T) {
    con.ascii(pos.x as i32, pos.y as i32, r.display_char() as u16);
    con.fore(pos.x as i32, pos.y as i32, r.color());
}
