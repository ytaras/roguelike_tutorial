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

const CONSOLE_DIM: Dim = Dim {
    width: 90,
    height: 50,
};

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {
    pretty_env_logger::init_timed();
    let mut root = RootInitializer::new()
        .font("static/terminal_12x12.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(CONSOLE_DIM.width.into(), CONSOLE_DIM.height.into())
        .title("Rust/libtcod tutorial")
        .init();

    // FXIME - Extrqct to world generation
    let mut world = World::new();
    world.register::<HasPos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();
    world.register::<TakesWholeTile>();

    let mut rng = rand::thread_rng();
    // FXIME Extract to script
    let (level, room) = level_1(&mut rng);
    let player_pos = room.center();

    world.add_resource(level);

    world
        .create_entity()
        .is_player()
        .with_actor_components('@', RED, player_pos)
        .build();

    tcod::system::set_fps(LIMIT_FPS);

    let mut game = Game::new(world);

    while !root.window_closed() {
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
