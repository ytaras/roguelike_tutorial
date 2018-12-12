use crate::common::gen::Gen;
use crate::data::structures::matrix::*;
use crate::data::structures::*;
use log::*;
use rand::Rng;

pub mod functions;
pub mod generators;
pub mod races;

pub use self::functions::*;
use crate::levels::generators::mosters::MonsterGeneratorParam;
use crate::levels::generators::Level;
use crate::levels::generators::LevelGenStrategy;
use crate::levels::generators::RoomGenStrategy;
use crate::levels::races::ALL_MONSTERS;

const MAP_DIM: Dim = Dim {
    width: 90,
    height: 45,
};

pub fn level_1<G>(rng: &mut G) -> (LevelInfo, Level)
where
    G: Rng,
{
    let mut level = LevelInfo::with_dim(MAP_DIM);
    let strategy = LevelGenStrategy {
        room_strategy: RoomGenStrategy {
            max_dim: Dim {
                height: 10,
                width: 10,
            },
            min_dim: Dim {
                height: 6,
                width: 6,
            },
            max_pos: level.max_pos().w().n(),
            min_pos: Pos::default().e().s(),
        },
        max_rooms: 30,
        monsters: 20..30,
        monster_strategy: MonsterGeneratorParam {
            templates: ALL_MONSTERS(),
        },
    };

    let rooms = Level::create(rng, &strategy);
    for room in &rooms.rooms {
        dig(&mut level, room);
        put_walls(&mut level, room);
    }
    for corridor in &rooms.corridors {
        dig(&mut level, corridor);
    }

    (level, rooms)
}
