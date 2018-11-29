extern crate doryen_rs;
extern crate rogue_tutorial;
extern crate specs;

use doryen_rs::{App, AppOptions};
use specs::prelude::*;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::systems::render::*;
use rogue_tutorial::ui::GameWorld;

type Dim = Pos;

const MAP_DIM: Dim = Dim { x: 90, y: 45 };
const CONSOLE_DIM: Dim = Dim { x: 90, y: 50 };

fn main() {
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();

    let mut level = LevelInfo::with_dim(&MAP_DIM);

    for (x, y) in &[(30, 29), (30, 30), (30, 31)] {
        level[&Pos { x: *x, y: *y }] = TileType::WALL;
    }

    world.add_resource(level);

    world
        .create_entity()
        .with(IsPlayer)
        .with(Pos {
            x: MAP_DIM.x / 2,
            y: MAP_DIM.y / 2,
        }).with(IsVisible {
            color: RED,
            display_char: '@',
        }).build();

    world
        .create_entity()
        .with(Pos {
            x: MAP_DIM.x / 2 - 5,
            y: MAP_DIM.y / 2,
        }).with(IsVisible {
            color: YELLOW,
            display_char: '@',
        }).build();

    let mut renderer = render_console();

    use specs::RunNow;

    renderer.run_now(&world.res);
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_DIM.x.into(),
        console_height: CONSOLE_DIM.y.into(),
        screen_width: (CONSOLE_DIM.x * 8).into(),
        screen_height: (CONSOLE_DIM.y * 8).into(),
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
