extern crate tcod;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rogue_tutorial;
extern crate specs;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::levels::level_1;
use rogue_tutorial::systems::render::*;
use rogue_tutorial::ui::Game;
use specs::prelude::*;
use tcod::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {
    pretty_env_logger::init_timed();
    let mut root = RootInitializer::new()
        .font("static/terminal_12x12.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    // FXIME - Extrqct to world generation
    let mut world = World::new();
    world.register::<HasPos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();
    world.register::<TakesWholeTile>();

    // FXIME Extract to script
    let level = level_1();
    let player_pos = Pos {
        x: level.width() / 2,
        y: level.height() / 2,
    };

    world.add_resource(level);

    world
        .create_entity()
        .is_player()
        .with_actor_components('@', RED, player_pos)
        .build();

    let mut npc_pos = player_pos;
    npc_pos.x -= 5;
    world
        .create_entity()
        .with_actor_components('@', YELLOW, npc_pos)
        .build();

    tcod::system::set_fps(LIMIT_FPS);

    let mut game = Game::new(world);

    while !root.window_closed() {
        use specs::RunNow;

        game.render_on(&mut root);

        root.flush();
        let x = root.wait_for_keypress(true);
        if let Some(c) = game.key_mapper.command(x) {
            trace!("{:?} - {:?}", x, c);
            game.game_command_handler.exec(c, &mut game.world);
        }
        game.update();
    }
}
