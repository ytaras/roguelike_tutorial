use crate::data::structures::matrix::*;
use crate::data::structures::*;
use std::cmp::max;
use std::cmp::min;
use std::ops::{Index, IndexMut, RangeInclusive};

fn dig<M, P>(map: &mut M, positions: &P)
where
    P: PosCollection,
    M: IndexMut<Pos> + Index<Pos, Output = TileType>,
{
    for p in positions.iter_pos() {
        map[p] = TileType::GROUND;
    }
}
const MAP_DIM: Dim = Dim {
    width: 90,
    height: 45,
};

pub fn level_1() -> LevelInfo {
    let mut level = LevelInfo::with_dim(MAP_DIM);
    let room1 = Room::new(
        Pos { x: 20, y: 15 },
        Dim {
            width: 10,
            height: 15,
        },
    );
    let room2 = Room::new(
        Pos { x: 35, y: 15 },
        Dim {
            width: 10,
            height: 15,
        },
    );
    dig(&mut level, &room1);
    dig(&mut level, &room2);
    dig(
        &mut level,
        &LCorridor::new(room1.center(), room2.center(), true),
    );
    level
}

#[derive(Debug, Clone)]
pub struct Room {
    from: Pos,
    to: Pos,
}

impl PosCollection for Room {
    type Iter = <RangeInclusive<Pos> as PosCollection>::Iter;
    fn iter_pos(&self) -> Self::Iter {
        (self.from..=self.to).iter_pos()
    }
}

impl Room {
    fn new(from: Pos, dim: Dim) -> Self {
        assert!(from.x > 0);
        assert!(from.y > 0);
        assert!(dim.width > 0);
        assert!(dim.height > 0);
        Room {
            from,
            to: Pos {
                x: from.x + dim.width,
                y: from.y + dim.height,
            },
        }
    }

    fn center(&self) -> Pos {
        Pos {
            x: ((self.from.x as u16 + self.to.x as u16) / 2) as DimIndex,
            y: ((self.from.y as u16 + self.to.y as u16) / 2) as DimIndex,
        }
    }

    fn contains(&self, p: Pos) -> bool {
        self.from.x <= p.x && self.to.x >= p.x && self.from.y <= p.y && self.to.y >= p.y
    }
}

#[derive(Debug)]
pub struct LCorridor {
    from: Pos,
    to: Pos,
    horizontal_first: bool,
}

impl LCorridor {
    pub fn new(pos1: Pos, pos2: Pos, horizontal_first: bool) -> Self {
        assert_ne!(pos1, pos2);
        let from = Pos {
            x: min(pos1.x, pos2.x),
            y: min(pos1.y, pos2.y),
        };
        let to = Pos {
            x: max(pos1.x, pos2.x),
            y: max(pos1.y, pos2.y),
        };
        LCorridor {
            from,
            to,
            horizontal_first,
        }
    }
}

impl PosCollection for LCorridor {
    type Iter = <Vec<Pos> as IntoIterator>::IntoIter;

    fn iter_pos(&self) -> <Self as PosCollection>::Iter {
        let mut vec = (self.from.x..=self.to.x)
            .map(|x| Pos { x, y: self.from.y })
            .collect::<Vec<_>>();
        vec.extend((self.from.y..=self.to.y).map(|y| Pos { x: self.to.x, y }));
        vec.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use proptest::{prop_assert, prop_assert_eq, prop_compose, proptest, proptest_helper};

    const MAX_DIM: Dim = Dim {
        width: DimIndex::max_value(),
        height: DimIndex::max_value(),
    };

    fn room(level_dim: Dim) -> BoxedStrategy<Room> {
        println!("{:?}", level_dim);
        ((1..level_dim.width - 1), (1..level_dim.height - 1))
            .prop_flat_map(move |(x, y)| {
                let dims = (1..(level_dim.width - x), (1..(level_dim.height - y)));
                println!("{:?}", dims);
                (Just(Pos { x, y }), dims)
            })
            .prop_map(|(pos, (width, height))| Room::new(pos, Dim { width, height }))
            .boxed()
    }

    fn level_and_room() -> BoxedStrategy<(LevelInfo, Room)> {
        (4..DimIndex::max_value(), 4..DimIndex::max_value())
            .prop_map(|(h, w)| LevelInfo::new(h, w))
            .prop_flat_map(|m| {
                let gen = room(m.dim());
                (Just(m), gen)
            })
            .boxed()
    }

    fn pos(d: Dim) -> BoxedStrategy<Pos> {
        (0..d.width, 0..d.height)
            .prop_map(|(x, y)| Pos { x, y })
            .boxed()
    }

    fn level_and_corridor() -> BoxedStrategy<(LevelInfo, LCorridor)> {
        (4..DimIndex::max_value(), 4..DimIndex::max_value())
            .prop_map(|(h, w)| LevelInfo::new(h, w))
            .prop_flat_map(|m| {
                let pos_gen = pos(m.dim());
                (Just(m), pos_gen.clone(), pos_gen)
            })
            .prop_filter("positions must be ne", |(_, from, to)| from != to)
            .prop_map(|(l, from, to)| (l, LCorridor::new(from, to, true)))
            .boxed()
    }

    impl Arbitrary for Room {
        type Parameters = ();

        fn arbitrary_with(_args: <Self as Arbitrary>::Parameters) -> <Self as Arbitrary>::Strategy {
            room(MAX_DIM)
        }

        type Strategy = BoxedStrategy<Room>;
    }

    proptest! {

        #[test]
        fn room_center_inside_room(room: Room) {
            prop_assert!(room.contains(room.center()))
        }
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
                for x in corridor.from.x..=corridor.to.x {
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
                for y in corridor.from.y..=corridor.to.y {
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
