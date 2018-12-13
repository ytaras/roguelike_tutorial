use crate::common::gen::Gen;
use crate::data::structures::pos::PosCollection;
use crate::data::structures::world_data::MonsterTemplate;
use crate::data::structures::*;
use crate::levels::generators::mosters::MonsterGeneratorParam;
use itertools::free::any;
use itertools::Itertools;
use log::{trace, warn};
use rand::seq::IteratorRandom;
use rand::Rng;
use std::collections::HashSet;
use std::ops::Range;

pub mod mosters;

#[derive(Debug, Clone, Copy)]
pub struct RoomGenStrategy {
    pub min_dim: Dim,
    pub max_dim: Dim,
    pub max_pos: Pos,
    pub min_pos: Pos,
}

impl Gen for Room {
    type Param = RoomGenStrategy;

    fn create<G>(rng: &mut G, param: &<Self as Gen>::Param) -> Self
    where
        G: Rng,
    {
        let width = rng.gen_range(param.min_dim.width, param.max_dim.width + 1);
        let height = rng.gen_range(param.min_dim.height, param.max_dim.height + 1);
        let dim = Dim { width, height };
        let x = rng.gen_range(param.min_pos.x, param.max_pos.x - width + 1);
        let y = rng.gen_range(param.min_pos.y, param.max_pos.y - height + 1);
        let pos = Pos { x, y };
        trace!("Generated {:?} with dim {:?} for {:?}", pos, dim, param);
        Room::new(pos, dim)
    }
}

#[derive(Debug, Clone)]
pub struct LevelGenStrategy {
    pub room_strategy: RoomGenStrategy,
    pub max_rooms: usize,
    pub monsters: Range<u8>,
    pub monster_strategy: MonsterGeneratorParam,
}

pub struct Level {
    pub rooms: Vec<Room>,
    pub corridors: Vec<LCorridor>,
    pub player_pos: Pos,
    pub monsters: Vec<(MonsterTemplate, Pos)>,
}

impl Gen for Level {
    type Param = LevelGenStrategy;

    fn create<G>(rng: &mut G, param: &<Self as Gen>::Param) -> Self
    where
        G: Rng,
    {
        let mut rooms: Vec<Room> = Vec::new();
        for _ in 0..param.max_rooms {
            let room = Room::create(rng, &param.room_strategy);
            let conflicts = any(rooms.iter(), |r| r.intersects(&room));
            if !conflicts {
                rooms.push(room);
            }
        }
        rooms.sort_by(|a, b| a.center().cmp(&b.center()));

        let mut corridors = Vec::new();
        for i in 1..rooms.len() {
            let room1 = rooms[i - 1];
            let room2 = rooms[i];
            let horizontal_first = rng.gen_bool(0.5);
            trace!(
                "{:?} - Connecting {:?}[{:?}] and {:?}[{:?}] with corridor",
                i,
                room1.center(),
                room1,
                room2.center(),
                room2
            );
            corridors.push(LCorridor::new(
                room1.center(),
                room2.center(),
                horizontal_first,
            ));
        }

        let player_pos = rooms[0].center();

        let mut free_pos = rooms.iter().flat_map(|r| r.iter_pos()).collect::<Vec<_>>();

        let monster_count = rng.gen_range(param.monsters.start, param.monsters.end);

        let mut monsters = Vec::with_capacity(monster_count.into());
        for _ in 0..monster_count {
            let monster_template = MonsterTemplate::create(rng, &param.monster_strategy);

            if free_pos.is_empty() {
                log::warn!("No more free cells left, level gen strategy: {:?}", param);
            } else {
                let pos_index = rng.gen_range(0, free_pos.len());
                let pos = free_pos.remove(pos_index);
                monsters.push((monster_template, pos));
            }
        }

        Level {
            rooms,
            corridors,
            player_pos,
            monsters,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::structures::pos::test::*;
    use crate::levels::races::ALL_MONSTERS;
    use itertools::iproduct;
    use proptest::prelude::*;
    use proptest::{prop_assert, proptest, proptest_helper};
    use std::cmp::max;
    use std::cmp::min;

    fn room_gen_strategy() -> BoxedStrategy<RoomGenStrategy> {
        (nonzero_pos_in_dim(MAX_DIM), nonzero_pos_in_dim(MAX_DIM))
            .prop_filter("positions must be not eq", |(p1, p2)| {
                p1.x != p2.x && p1.y != p2.y
            })
            .prop_map(|(p1, p2)| {
                (
                    Pos {
                        x: min(p1.x, p2.x),
                        y: min(p1.y, p2.y),
                    },
                    Pos {
                        x: max(p1.x, p2.x),
                        y: max(p1.y, p2.y),
                    },
                )
            })
            .prop_flat_map(|(p1, p2)| {
                let max_w = p2.x - p1.x;
                let max_h = p2.y - p1.y;
                (Just(p1), Just(p2), 0..max_w, 0..max_w, 0..max_h, 0..max_h)
            })
            .prop_map(|(min_pos, max_pos, w1, w2, h1, h2)| {
                let min_dim = Dim {
                    width: min(w1, w2) + 1,
                    height: max(h1, h2) + 1,
                };
                let max_dim = Dim {
                    width: max(w1, w2) + 1,
                    height: max(h1, h2) + 1,
                };
                RoomGenStrategy {
                    min_dim,
                    max_dim,
                    min_pos,
                    max_pos,
                }
            })
            .boxed()
    }

    fn rooms_gen_strategy() -> impl Strategy<Value = LevelGenStrategy> {
        (room_gen_strategy(), 1..30).prop_map(|(room_strategy, max_rooms)| LevelGenStrategy {
            room_strategy,
            max_rooms: max_rooms as usize,
            monsters: 20..30,
            monster_strategy: MonsterGeneratorParam {
                templates: ALL_MONSTERS(),
            },
        })
    }

    proptest! {

        #[test]
        fn room_gen_generates_room_in_bounds(rgs in room_gen_strategy()) {
            let mut rng = rand::thread_rng();
            let room = Room::create(&mut rng, &rgs);
            prop_assert!(room.from.x >= rgs.min_pos.x);
            prop_assert!(room.from.y >= rgs.min_pos.y);
            prop_assert!(room.to.x   <= rgs.max_pos.x);
            prop_assert!(room.to.y   <= rgs.max_pos.y);
            prop_assert!(room.width() >= rgs.min_dim.width);
            prop_assert!(room.height() >= rgs.min_dim.height);
            prop_assert!(room.width() <= rgs.max_dim.width);
            prop_assert!(room.height() <= rgs.max_dim.height);
        }


        #[test]
        fn rooms_gen_generates_room_in_bounds(rgs in rooms_gen_strategy()) {
            let mut rng = rand::thread_rng();
            let rooms = Level::create(&mut rng, &rgs);
            prop_assert!(rooms.rooms.len() <= rgs.max_rooms);
            let rgs = rgs.room_strategy;
            for room in rooms.rooms.clone() {
                prop_assert!(room.from.x   >= rgs.min_pos.x);
                prop_assert!(room.from.y   >= rgs.min_pos.y);
                prop_assert!(room.to.x     <= rgs.max_pos.x);
                prop_assert!(room.to.y     <= rgs.max_pos.y);
                prop_assert!(room.width()  >= rgs.min_dim.width);
                prop_assert!(room.height() >= rgs.min_dim.height);
                prop_assert!(room.width()  <= rgs.max_dim.width);
                prop_assert!(room.height() <= rgs.max_dim.height);
            }

            for ((i1, r1), (i2, r2)) in iproduct!(rooms.rooms.clone().iter().enumerate(),
                                                  rooms.rooms.iter().enumerate()) {
                if i1 != i2 {
                    prop_assert!(!r1.intersects(&r2), "{:?} intersects with {:?}", r1, r2);
                }
            }
        }
    }
}
