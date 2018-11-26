extern crate rogue_tutorial;
extern crate specs;

use rogue_tutorial::data::components::*;
use rogue_tutorial::systems::render::*;
use specs::prelude::*;

fn main() {
    let mut world = World::new();
    world.register::<HasPos>();
    world.register::<IsVisible>();

    world
        .create_entity()
        .with(HasPos { x: 1, y: 10 })
        .with(IsVisible {
            color: RED,
            display_char: '@',
        }).build();

    let mut renderer = render_console();

    use specs::RunNow;

    renderer.run_now(&world.res);
}
