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

fn dig<M, P>(map: &mut M, positions: &P)
where
    P: PosCollection,
    M: IndexMut<Pos> + Index<Pos, Output = TileType>,
{
    for p in positions.iter_pos() {
        map[p] = TileType::Ground;
    }
}

fn put_walls<M, P>(map: &mut M, stru: &P)
where
    P: HasWall,
    M: IndexMut<Pos> + Index<Pos, Output = TileType>,
{
    for p in stru.walls() {
        map[p] = TileType::RoomWall;
    }
}

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
    let room1 = rooms.0[0];
    for room in rooms.0 {
        dig(&mut level, &room);
        put_walls(&mut level, &room);
    }
    for corridor in rooms.1 {
        dig(&mut level, &corridor);
    }
    (level, room1)
}

#[derive(Debug, Clone, Copy)]
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

impl<'a> HasWall for Room {
    type Iter = Box<dyn ExactSizeIterator<Item = Pos>>;

    fn walls(&self) -> <Self as HasWall>::Iter {
        let mut walls = Vec::new();
        for x in (self.from.x - 1)..=(self.to.x + 1) {
            walls.push(Pos {
                x,
                y: self.from.y - 1,
            });
            walls.push(Pos {
                x,
                y: self.to.y + 1,
            });
        }

        for y in (self.from.y)..=(self.to.y) {
            walls.push(Pos {
                y,
                x: self.from.x - 1,
            });
            walls.push(Pos {
                y,
                x: self.to.x + 1,
            });
        }
        Box::new(walls.into_iter())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RoomGenStrategy {
    min_dim: Dim,
    max_dim: Dim,
    max_pos: Pos,
    min_pos: Pos,
}

impl Gen for Room {
    type Param = RoomGenStrategy;

    fn create<G>(rng: &mut G, param: <Self as Gen>::Param) -> Self
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

    pub fn center(&self) -> Pos {
        Pos {
            x: (self.from.x / 2) + (self.to.x / 2),
            y: (self.from.y / 2) + (self.to.y / 2),
        }
    }

    fn contains(&self, p: Pos) -> bool {
        self.from.x <= p.x && self.to.x >= p.x && self.from.y <= p.y && self.to.y >= p.y
    }

    fn contains_or_touches(&self, p: Pos) -> bool {
        self.from.x - 1 <= p.x
            && self.to.x + 1 >= p.x
            && self.from.y - 1 <= p.y
            && self.to.y + 1 >= p.y
    }

    fn intersects(&self, other: &Room) -> bool {
        (self.to.x >= other.from.x - 1)
            && (other.to.x >= self.from.x - 1)
            && (self.to.y >= other.from.y - 1)
            && (other.to.y >= self.from.y - 1)
    }

    fn width(&self) -> DimIndex {
        self.to.x - self.from.x
    }

    fn height(&self) -> DimIndex {
        self.to.y - self.from.y
    }

    fn nw_corner(&self) -> Pos {
        self.from
    }

    fn sw_corner(&self) -> Pos {
        Pos {
            x: self.from.x,
            y: self.to.y,
        }
    }

    fn ne_corner(&self) -> Pos {
        Pos {
            x: self.to.x,
            y: self.from.y,
        }
    }

    fn se_corner(&self) -> Pos {
        self.to
    }

    fn corners(&self) -> [Pos; 4] {
        [
            self.ne_corner(),
            self.nw_corner(),
            self.se_corner(),
            self.sw_corner(),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RoomsGenStrategy {
    room_strategy: RoomGenStrategy,
    max_rooms: usize,
}

pub struct Rooms(Vec<Room>, Vec<LCorridor>);

impl Gen for Rooms {
    type Param = RoomsGenStrategy;

    fn create<G>(rng: &mut G, param: <Self as Gen>::Param) -> Self
    where
        G: Rng,
    {
        let mut res: Vec<Room> = Vec::new();
        for _ in 0..param.max_rooms {
            let room = Room::create(rng, param.room_strategy);
            let conflicts = any(res.iter(), |r| r.intersects(&room));
            if !conflicts {
                res.push(room);
            }
        }
        res.sort_by(|a, b| a.center().cmp(&b.center()));

        let mut corridors = Vec::new();
        for i in 1..res.len() {
            let room1 = res[i - 1];
            let room2 = res[i];
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

        Rooms(res, corridors)
    }
}

#[derive(Debug)]
pub struct LCorridor {
    from: Pos,
    to: Pos,
    horizontal_first: bool,
}

impl LCorridor {
    pub fn new(from: Pos, to: Pos, horizontal_first: bool) -> Self {
        assert_ne!(from, to);
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
        fn range(p1: DimIndex, p2: DimIndex) -> RangeInclusive<DimIndex> {
            (min(p1, p2))..=(max(p1, p2))
        }
        let mut vec = range(self.from.x, self.to.x)
            .map(|x| Pos { x, y: self.from.y })
            .collect::<Vec<_>>();
        vec.extend(range(self.from.y, self.to.y).map(|y| Pos { x: self.to.x, y }));
        vec.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::{any, iproduct};
    use proptest::prelude::*;
    use proptest::{
        prop_assert, prop_assert_eq, prop_assert_ne, prop_compose, proptest, proptest_helper,
    };
    use std::collections::HashSet;

    const MAX_DIM: Dim = Dim {
        width: DimIndex::max_value(),
        height: DimIndex::max_value(),
    };

    const SMALL_DIM: Dim = Dim {
        width: 100,
        height: 100,
    };

    fn room(level_dim: Dim) -> BoxedStrategy<Room> {
        println!("{:?}", level_dim);
        ((1..level_dim.width - 1), (1..level_dim.height - 1))
            .prop_flat_map(move |(x, y)| {
                let dims = (1..(level_dim.width - x), (1..(level_dim.height - y)));
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

    fn pos(start: Pos, end: Pos) -> BoxedStrategy<Pos> {
        (start.x..=end.x, start.y..=end.y)
            .prop_map(|(x, y)| Pos { x, y })
            .boxed()
    }

    fn pos_in(d: Dim) -> BoxedStrategy<Pos> {
        pos(
            Pos { x: 0, y: 0 },
            Pos {
                x: d.width - 1,
                y: d.height - 1,
            },
        )
    }

    fn nonzero_pos_in(d: Dim) -> BoxedStrategy<Pos> {
        pos(
            Pos { x: 1, y: 1 },
            Pos {
                x: d.width - 1,
                y: d.height - 1,
            },
        )
    }

    fn nonzero_dim(max_dim: Dim) -> BoxedStrategy<Dim> {
        nonzero_pos_in(max_dim)
            .prop_map(|p| Dim {
                width: p.x + 1,
                height: p.y + 1,
            })
            .boxed()
    }
    fn dim(max_dim: Dim) -> BoxedStrategy<Dim> {
        (0..=max_dim.width, 0..=max_dim.height)
            .prop_map(|(width, height)| Dim { width, height })
            .boxed()
    }

    fn level_and_corridor() -> BoxedStrategy<(LevelInfo, LCorridor)> {
        (4..DimIndex::max_value(), 4..DimIndex::max_value())
            .prop_map(|(h, w)| LevelInfo::new(h, w))
            .prop_flat_map(|m| {
                let pos_gen = pos_in(m.dim());
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

    fn room_gen_strategy() -> BoxedStrategy<RoomGenStrategy> {
        (nonzero_pos_in(MAX_DIM), nonzero_pos_in(MAX_DIM))
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

    fn rooms_gen_strategy() -> impl Strategy<Value = RoomsGenStrategy> {
        (room_gen_strategy(), 1..30).prop_map(|(room_strategy, max_rooms)| RoomsGenStrategy {
            room_strategy,
            max_rooms: max_rooms as usize,
        })
    }

    proptest! {

            #[test]
            fn room_center_inside_room(room: Room) {
                let nw = Pos {
                    x: room.from.x - 1,
                    y: room.from.y - 1,
                };
                let se = Pos {
                    x: room.to.x + 1,
                    y: room.to.y + 1,
                };

                prop_assert!(room.contains(room.center()));
                prop_assert!(room.contains(room.from));
                prop_assert!(room.contains(room.to));
                prop_assert!(!room.contains(nw));
                prop_assert!(!room.contains(se));

                prop_assert!(room.contains_or_touches(room.center()));
                prop_assert!(room.contains_or_touches(room.from));
                prop_assert!(room.contains_or_touches(room.to));
                prop_assert!(room.contains_or_touches(nw));
                prop_assert!(room.contains_or_touches(se));
            }

            #[test]
            fn room_positions_and_walls_are_distinct(room: Room) {
                let all_inside = room.iter_pos().collect::<HashSet<_>>();
                let all_walls = room.walls().collect::<HashSet<_>>();
                prop_assert!(all_inside.is_disjoint(&all_walls));
            }

            #[test]
            fn room_positions_inside_room(room: Room) {
                for pos in room.iter_pos() {
                    prop_assert!(room.contains(pos));
    //                prop_assert!(!all_walls.contains(&pos), "{:?} is also a wall", pos);
                }
                prop_assert_ne!(room.walls().len(), 0);
    //            prop_assert_eq!(room.walls().unique().collect_vec().len(),
    //                            room.walls().len());
                for pos in room.walls() {
                    prop_assert!(room.contains_or_touches(pos));
                    prop_assert!(!room.contains(pos));
    //                prop_assert!(!all_inside.contains(&pos), "{:?} is also a ground tile", pos);
                }

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

            #[test]
            fn dims_preserved(p in nonzero_pos_in(SMALL_DIM), d in nonzero_dim(SMALL_DIM)) {
                let room = Room::new(p, d);
                prop_assert_eq!(room.width(), d.width);
                prop_assert_eq!(room.height(), d.height);
            }

            #[test]
            fn room_gen_generates_room_in_bounds(rgs in room_gen_strategy()) {
                let mut rng = rand::thread_rng();
                let room = Room::create(&mut rng, rgs);
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
            fn room_intersection(r1: Room, r2: Room) {
                if r1.intersects(&r2) {
                    prop_assert!(any(r1.iter_pos(), |pos1| r2.contains_or_touches(pos1)));
                    prop_assert!(any(r2.iter_pos(), |pos2| r1.contains_or_touches(pos2)));
                } else {
                    for pos1 in r1.iter_pos() {
                        prop_assert!(!r2.contains_or_touches(pos1), "{:?} from {:?} was in {:?}", pos1, r1, r2);
                    }
                    for pos2 in r2.iter_pos() {
                        prop_assert!(!r1.contains_or_touches(pos2), "{:?} from {:?} was in {:?}", pos2, r2, r1);
                    }
                }
            }

            #[test]
            fn rooms_gen_generates_room_in_bounds(rgs in rooms_gen_strategy()) {
                let mut rng = rand::thread_rng();
                let rooms = Rooms::create(&mut rng, rgs);
                prop_assert!(rooms.0.len() <= rgs.max_rooms);
                let rgs = rgs.room_strategy;
                for room in rooms.0.clone() {
                    prop_assert!(room.from.x   >= rgs.min_pos.x);
                    prop_assert!(room.from.y   >= rgs.min_pos.y);
                    prop_assert!(room.to.x     <= rgs.max_pos.x);
                    prop_assert!(room.to.y     <= rgs.max_pos.y);
                    prop_assert!(room.width()  >= rgs.min_dim.width);
                    prop_assert!(room.height() >= rgs.min_dim.height);
                    prop_assert!(room.width()  <= rgs.max_dim.width);
                    prop_assert!(room.height() <= rgs.max_dim.height);
                }

                for ((i1, r1), (i2, r2)) in iproduct!(rooms.0.clone().iter().enumerate(),
                                                      rooms.0.iter().enumerate()) {
                    if i1 != i2 {
                        prop_assert!(!r1.intersects(&r2), "{:?} intersects with {:?}", r1, r2);
                    }
                }
            }
        }
}
