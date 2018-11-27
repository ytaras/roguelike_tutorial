use data::components::*;
use data::structures::*;
use doryen_rs::DoryenApi;
use specs::prelude::*;

pub struct DoryenRenderer<'a> {
    pub doryen_api: &'a mut DoryenApi,
}

impl<'a> System<'a> for DoryenRenderer<'a> {
    type SystemData = (
        ReadStorage<'a, HasPos>,
        ReadStorage<'a, IsVisible>,
        Read<'a, LevelInfo>,
    );
    fn run(&mut self, (pos, vis, li): Self::SystemData) {
        use specs::Join;
        let con = self.doryen_api.con();
        con.area(
            0,
            0,
            li.width,
            li.height,
            Some(WHITE),
            None,
            Some('.' as u16),
        );
        for (pos, vis) in (&pos, &vis).join() {
            con.ascii(pos.x as i32, pos.y as i32, vis.display_char as u16);
            con.fore(pos.x as i32, pos.y as i32, vis.color);
        }
    }
}
