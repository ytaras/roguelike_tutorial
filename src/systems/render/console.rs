use ansi_term::*;
use data::components::*;
use data::structures::*;
use specs::prelude::*;

pub struct StdoutRender;

impl<'a> System<'a> for StdoutRender {
    type SystemData = (ReadStorage<'a, HasPos>, ReadStorage<'a, IsVisible>);
    fn run(&mut self, (pos, vis): Self::SystemData) {
        use specs::Join;
        for (pos, vis) in (&pos, &vis).join() {
            let (r, g, b, _) = vis.color;
            let painted = Colour::RGB(r, g, b).paint(vis.display_char.to_string());
            println!("Char {} at {:?}", painted, (pos.x, pos.y));
        }
    }
}
