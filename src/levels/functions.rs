use crate::data::structures::pos::PosCollection;
use crate::data::structures::*;
use std::ops::Index;
use std::ops::IndexMut;

pub fn dig<M, P>(map: &mut M, positions: &P)
where
    P: PosCollection,
    M: IndexMut<Pos> + Index<Pos, Output = TileType>,
{
    for p in positions.iter_pos() {
        map[p] = TileType::Ground;
    }
}

pub fn put_walls<M, P>(map: &mut M, stru: &P)
where
    P: HasWall,
    M: IndexMut<Pos> + Index<Pos, Output = TileType>,
{
    for p in stru.walls() {
        map[p] = TileType::RoomWall;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::data::structures::pos::test::*;
    use crate::data::structures::room::test::*;
    use proptest::prelude::*;
    use proptest::{
        prop_assert, prop_assert_eq, prop_assert_ne, prop_compose, proptest, proptest_helper,
    };
    use std::cmp::{max, min};

    pub fn level_and_room() -> BoxedStrategy<(LevelInfo, Room)> {
        (4..DimIndex::max_value(), 4..DimIndex::max_value())
            .prop_map(|(h, w)| LevelInfo::new(h, w))
            .prop_flat_map(|m| {
                let gen = room_in(m.dim());
                (Just(m), gen)
            })
            .boxed()
    }

    pub fn level_and_corridor() -> BoxedStrategy<(LevelInfo, LCorridor)> {
        (4..DimIndex::max_value(), 4..DimIndex::max_value())
            .prop_map(|(h, w)| LevelInfo::new(h, w))
            .prop_flat_map(|m| {
                let pos_gen = pos_in_dim(m.dim());
                (Just(m), pos_gen.clone(), pos_gen)
            })
            .prop_filter("positions must be ne", |(_, from, to)| from != to)
            .prop_map(|(l, from, to)| (l, LCorridor::new(from, to, true)))
            .boxed()
    }

    proptest! {
        #[test]
        fn room_is_diggable((mut level, room) in level_and_room()) {
            prop_assert!(level.width() > room.to.x);
            prop_assert!(level.height() > room.to.y);
            let start_x = room.from.x;
            let start_y = room.from.y;
            let end_x = room.to.x;
            let end_y = room.to.y;
            dig(&mut level, &room);
            for p in level.iter_pos() {
                if p.x >= start_x && p.x <= end_x && p.y >= start_y && p.y <= end_y {
                    prop_assert!(level[p].is_walkable());
                    prop_assert!(!level[p].blocks_sight());
                } else {
                    prop_assert!(!level[p].is_walkable());
                    prop_assert!(level[p].blocks_sight());
                }
            }
        }

        #[test]
        fn corridor_is_diggable((mut level, corridor) in level_and_corridor()) {
            dig(&mut level, &corridor);
            // check horizontal lane
            {
                let y = if corridor.horizontal_first { corridor.from.y } else { corridor.to.y };
                let from_x = min(corridor.from.x, corridor.to.x);
                let to_x = max(corridor.from.x, corridor.to.x);
                for x in from_x..=to_x {
                        let pos = Pos{x, y};
                        prop_assert!(
                            level[pos].is_walkable(),
                            "{:?} is not walkable", pos
                        );
                }
            }
            // check vertical lane
            {
                let x = if corridor.horizontal_first { corridor.to.x } else { corridor.from.x };
                let from_y = min(corridor.from.y, corridor.to.y);
                let to_y = max(corridor.from.y, corridor.to.y);
                for y in from_y..=to_y {
                        let pos = Pos{x, y};
                        prop_assert!(
                            level[pos].is_walkable(),
                            "{:?} is not walkable", pos
                        );
                }
            }
        }

    }
}
