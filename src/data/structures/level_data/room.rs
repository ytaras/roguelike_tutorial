use std::ops::RangeInclusive;

use crate::data::structures::level_data::HasWall;
use crate::data::structures::pos::PosCollection;
use crate::data::structures::Dim;
use crate::data::structures::DimIndex;
use crate::data::structures::Pos;

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub from: Pos,
    pub to: Pos,
}

impl PosCollection for Room {
    type Iter = <RangeInclusive<Pos> as PosCollection>::Iter;
    fn iter_pos(&self) -> Self::Iter {
        (self.from..=self.to).iter_pos()
    }
}

impl HasWall for Room {
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

impl Room {
    pub fn new(from: Pos, dim: Dim) -> Self {
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

    pub fn contains(&self, p: Pos) -> bool {
        self.from.x <= p.x && self.to.x >= p.x && self.from.y <= p.y && self.to.y >= p.y
    }

    pub fn contains_or_touches(&self, p: Pos) -> bool {
        self.from.x - 1 <= p.x
            && self.to.x + 1 >= p.x
            && self.from.y - 1 <= p.y
            && self.to.y + 1 >= p.y
    }

    pub fn intersects(&self, other: &Room) -> bool {
        (self.to.x >= other.from.x - 1)
            && (other.to.x >= self.from.x - 1)
            && (self.to.y >= other.from.y - 1)
            && (other.to.y >= self.from.y - 1)
    }

    pub fn width(&self) -> DimIndex {
        self.to.x - self.from.x
    }

    pub fn height(&self) -> DimIndex {
        self.to.y - self.from.y
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashSet;

    use itertools::any;
    use proptest::prelude::*;
    use proptest::{prop_assert, prop_assert_eq, prop_assert_ne, proptest, proptest_helper};

    use crate::data::structures::dim::test::*;
    use crate::data::structures::pos::test::*;

    use super::*;

    pub fn room_in(level_dim: Dim) -> BoxedStrategy<Room> {
        println!("{:?}", level_dim);
        ((1..level_dim.width - 1), (1..level_dim.height - 1))
            .prop_flat_map(move |(x, y)| {
                let dims = (1..(level_dim.width - x), (1..(level_dim.height - y)));
                (Just(Pos { x, y }), dims)
            })
            .prop_map(|(pos, (width, height))| Room::new(pos, Dim { width, height }))
            .boxed()
    }

    impl Arbitrary for Room {
        type Parameters = ();

        fn arbitrary_with(_args: <Self as Arbitrary>::Parameters) -> <Self as Arbitrary>::Strategy {
            room_in(MAX_DIM)
        }

        type Strategy = BoxedStrategy<Room>;
    }

    proptest! {

        #[test]
        fn dims_preserved(p in nonzero_pos_in_dim(SMALL_DIM), d in nonzero_dim(SMALL_DIM)) {
            let room = Room::new(p, d);
            prop_assert_eq!(room.width(), d.width);
            prop_assert_eq!(room.height(), d.height);
        }

        #[test]
        fn room_center_inside_room(room: Room) {
            prop_assert!(room.contains(room.center()));
            prop_assert!(room.contains(room.from));
            prop_assert!(room.contains(room.to));
            prop_assert!(!room.contains(room.from.n().w()));
            prop_assert!(!room.contains(room.to.s().e()));

            prop_assert!(room.contains_or_touches(room.center()));
            prop_assert!(room.contains_or_touches(room.from));
            prop_assert!(room.contains_or_touches(room.to));
            prop_assert!(room.contains_or_touches(room.from.n().w()));
            prop_assert!(room.contains_or_touches(room.to.s().e()));
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
            }
            prop_assert_ne!(room.walls().len(), 0);
            for pos in room.walls() {
                prop_assert!(room.contains_or_touches(pos));
                prop_assert!(!room.contains(pos));
            }

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
    }
}
