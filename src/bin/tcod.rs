#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rogue_tutorial;
extern crate specs;
extern crate tcod;

use specs::{Builder, System, World};
use tcod::colors::RED;
use tcod::*;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::levels::level_1;
use rogue_tutorial::systems::render::Renderer;
use rogue_tutorial::ui::Game;

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
    root.as_specs_system().setup(&mut world.res);
    let mut game = Game::new(world);
    let mut rng = rand::thread_rng();
    // FXIME Extract to script
    let (level_info, level) = level_1(&mut rng);

    game.world.add_resource(level_info);

    game.world
        .create_entity()
        .is_player()
        .with_actor_components('@', RED, level.player_pos)
        .build();

    for (monster, pos) in level.monsters {
        game.world.create_entity().is_monster(&monster, pos).build();
    }

    tcod::system::set_fps(LIMIT_FPS);

    while !root.window_closed() {
        game.update();
        game.render_on(&mut root);

        root.flush();
        let x = root.wait_for_keypress(true);
        if let Some(c) = game.key_mapper.command(x) {
            trace!("{:?} - {:?}", x, c);
            game.game_command_handler.exec(c, &mut game.world);
        }
    }
}
