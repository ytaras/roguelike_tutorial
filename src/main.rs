extern crate doryen_rs;
extern crate rogue_tutorial;
extern crate specs;

use doryen_rs::{App, AppOptions};
use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::systems::render::*;
use rogue_tutorial::ui::GameWorld;
use specs::prelude::*;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

fn main() {
    let mut world = World::new();
    world.register::<HasPos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();
    world.add_resource(LevelInfo {
        width: CONSOLE_WIDTH,
        height: CONSOLE_HEIGHT,
    });

    world
        .create_entity()
        .with(IsPlayer)
        .with(HasPos { x: 1, y: 10 })
        .with(IsVisible {
            color: RED,
            display_char: '@',
        }).build();

    let mut renderer = render_console();

    use specs::RunNow;

    renderer.run_now(&world.res);
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "my roguelike".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(GameWorld::new(world)));
    app.run();
}
