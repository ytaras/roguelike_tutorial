extern crate doryen_rs;
extern crate rogue_tutorial;
extern crate specs;

use doryen_rs::{App, AppOptions};
use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::systems::render::*;
use rogue_tutorial::ui::GameWorld;
use specs::prelude::*;

const CONSOLE_WIDTH: u16 = 80;
const CONSOLE_HEIGHT: u16 = 45;

fn main() {
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();

    let mut level = LevelInfo::new(CONSOLE_WIDTH, CONSOLE_HEIGHT);

    for (x, y) in &[(30, 29), (30, 30), (30, 31)] {
        level[&Pos { x: *x, y: *y }] = TileType::WALL;
    }

    world.add_resource(level);

    world
        .create_entity()
        .with(IsPlayer)
        .with(Pos { x: 1, y: 10 })
        .with(IsVisible {
            color: RED,
            display_char: '@',
        }).build();

    let mut renderer = render_console();

    use specs::RunNow;

    renderer.run_now(&world.res);
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH.into(),
        console_height: CONSOLE_HEIGHT.into(),
        screen_width: (CONSOLE_WIDTH * 8).into(),
        screen_height: (CONSOLE_HEIGHT * 8).into(),
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
