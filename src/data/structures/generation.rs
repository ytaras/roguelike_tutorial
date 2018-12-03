use std::cmp::{max, min};
use std::ops::Range;
use std::ops::{Index, IndexMut};

use data::structures::matrix::*;
use data::structures::*;

#[derive(Clone, Debug)]
pub struct Room {
    corner1: Pos,
    corner2: Pos,
}

impl Room {
    pub fn dig<T>(&self, level: &mut T)
    where
        T: IndexMut<Pos> + Index<Pos, Output = TileType>,
    {
        for p in self.iter_pos() {
            level[p] = TileType::GROUND;
        }
    }

    pub fn contains(&self, p: Pos) -> bool {
        p.x >= self.corner1.x
            && p.x <= self.corner2.x
            && p.y >= self.corner1.y
            && p.y <= self.corner2.y
    }

    pub fn new(pos1: Pos, pos2: Pos) -> Self {
        assert_ne!(pos1, pos2);
        let corner1 = Pos {
            x: min(pos1.x, pos2.x),
            y: min(pos1.y, pos2.y),
        };
        let corner2 = Pos {
            x: max(pos1.x, pos2.x),
            y: max(pos1.y, pos2.y),
        };
        Room { corner1, corner2 }
    }

    fn with_dim(start: Pos, dim: Pos) -> Self {
        Room::new(
            start,
            Pos {
                x: start.x + dim.x,
                y: start.y + dim.y,
            },
        )
    }
}

impl PosCollection for Room {
    type Iter = <Range<Pos> as PosCollection>::Iter;

    fn iter_pos(&self) -> <Self as PosCollection>::Iter {
        let range = self.corner1..self.corner2;
        range.iter_pos()
    }
}

#[cfg(test)]
mod test {
    use quickcheck::*;

    use super::*;
    use data::structures::matrix::test::random_pos;
    #[derive(Clone, Debug)]
    struct LevelAndRoom {
        level: Matrix<TileType>,
        room: Room,
    }
    impl Arbitrary for LevelAndRoom {
        fn arbitrary<G>(gen: &mut G) -> Self
        where
            G: Gen,
        {
            let level: Matrix<TileType> = Arbitrary::arbitrary(gen);
            let corner1 = random_pos(&level, gen);
            let corner2 = random_pos(&level, gen);
            LevelAndRoom {
                level,
                room: Room::new(corner1, corner2),
            }
        }
    }

    #[test]
    fn check_dig() {
        let mut m: Matrix<TileType> = Matrix::new(1, 1);
        let room = Room::with_dim(Pos { x: 0, y: 0 }, Pos { x: 1, y: 1 });

        room.dig(&mut m);

        for (pos, tile) in m.iter() {
            let expected_type = if room.contains(pos) {
                TileType::GROUND
            } else {
                TileType::WALL
            };
            assert_eq!(&expected_type, tile);
        }
    }

    quickcheck! {
        fn test_new(pos1: Pos, pos2: Pos) -> bool {
            let room = Room::new(pos1, pos2);
            assert!(pos1.x == room.corner1.x || pos2.x == room.corner1.x);
            assert!(pos1.y == room.corner1.y || pos2.y == room.corner1.y);
            assert!(pos1.x == room.corner2.x || pos2.x == room.corner2.x);
            assert!(pos1.y == room.corner2.y || pos2.y == room.corner2.y);
            assert!(room.corner1.x <= room.corner2.x);
            assert!(room.corner1.y <= room.corner2.y);
            true
        }

        fn dig_in_map(lr: LevelAndRoom) -> TestResult {
                let mut m = lr.level;
                lr.room.dig(&mut m);
                for (pos, tile) in m.iter() {
                    let expected_type = if lr.room.contains(pos) {
                        TileType::GROUND
                    } else {
                        TileType::WALL
                    };
                    if &expected_type != tile {
                        return TestResult::failed();
                    }
                }
                TestResult::passed()

        }
    }
}
