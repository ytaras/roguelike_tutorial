use crate::common::gen::Gen;
use crate::data::structures::matrix::*;
use crate::data::structures::*;
use itertools::any;
use itertools::Itertools;
use log::*;
use rand::Rng;
use std::cmp::max;
use std::cmp::min;
use std::ops::Range;
use std::ops::{Index, IndexMut, RangeInclusive};
use std::prelude::v1::Vec;

pub mod functions;
pub mod generators;

pub use self::functions::*;
use crate::levels::generators::RoomGenStrategy;
use crate::levels::generators::Rooms;
use crate::levels::generators::RoomsGenStrategy;

const MAP_DIM: Dim = Dim {
    width: 90,
    height: 45,
};

pub fn level_1<G>(rng: &mut G) -> (LevelInfo, Room)
where
    G: Rng,
{
    let mut level = LevelInfo::with_dim(MAP_DIM);
    let strategy = RoomsGenStrategy {
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
    };

    let rooms = Rooms::create(rng, strategy);
    let room1 = rooms.rooms[0];
    for room in rooms.rooms {
        dig(&mut level, &room);
        put_walls(&mut level, &room);
    }
    for corridor in rooms.corridors {
        dig(&mut level, &corridor);
    }
    (level, room1)
}
