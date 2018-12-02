extern crate doryen_rs;
extern crate rogue_tutorial;
extern crate specs;

use doryen_rs::{App, AppOptions};
use specs::prelude::*;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::levels::*;
use rogue_tutorial::systems::render::*;
use rogue_tutorial::ui::GameWorld;

type Dim = Pos;

const CONSOLE_DIM: Dim = Dim { x: 90, y: 50 };

fn main() {
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();
    world.register::<TakesWholeTile>();

    let level = level_one();

    world
        .create_entity()
        .is_player()
        .with_actor_components(
            '@',
            RED,
            Pos {
                x: level.width() / 2,
                y: level.height() / 2,
            },
        ).build();

    world
        .create_entity()
        .with_actor_components(
            '@',
            YELLOW,
            Pos {
                x: level.width() / 2 - 5,
                y: level.height() / 2,
            },
        ).build();

    world.add_resource(level);

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
