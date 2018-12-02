use data::structures::matrix::*;
use data::structures::*;
use std::cmp::{max, min};

#[derive(Clone, Debug)]
pub struct Room {
    corner1: Pos,
    corner2: Pos,
}

impl Room {
    fn dig(&self, level: &mut Matrix<TileType>) {
        for x in self.corner1.x..=(self.corner2.x) {
            for y in self.corner1.y..=(self.corner2.y) {
                level[Pos { x, y }] = TileType::GROUND;
            }
        }
    }

    fn contains(&self, p: Pos) -> bool {
        p.x >= self.corner1.x
            && p.x <= self.corner2.x
            && p.y >= self.corner1.y
            && p.y <= self.corner2.y
    }

    fn new(pos1: Pos, pos2: Pos) -> Self {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use data::structures::matrix::test::*;
    use quickcheck::*;

    impl Arbitrary for Room {
        fn arbitrary<G>(gen: &mut G) -> Self
        where
            G: Gen,
        {
            let (corner1, corner2) = <(Pos, Pos)>::arbitrary(gen);
            Room::new(corner1, corner2)
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let shrinker = (self.corner1, self.corner2).shrink();
            let iter = shrinker.map(|(corner1, corner2)| Room { corner1, corner2 });
            Box::new(iter)
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

        fn dig_in_map(m: Matrix<TileType>, room: Room) -> TestResult {
            if m.is_valid(room.corner1) && m.is_valid(room.corner2) {
                let mut m = m;
                room.dig(&mut m);
                for (pos, tile) in m.iter() {
                    let expected_type = if room.contains(pos) {
                        TileType::GROUND
                    } else {
                        TileType::WALL
                    };
                    assert_eq!(expected_type, m[pos]);
                }
                TestResult::passed()
            } else {
            TestResult::discard()
            }
        }
    }
}
