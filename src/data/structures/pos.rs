use std::iter::{once, Once};
use std::ops::Range;

use itertools::*;
use specs::prelude::*;
use specs_derive::*;

pub type DimIndex = u16;

#[derive(Debug, PartialEq, Copy, Clone, Component, Eq, Hash)]
pub struct Pos {
    pub x: DimIndex,
    pub y: DimIndex,
}

pub trait PosCollection {
    type Iter: Iterator<Item = Pos>;

    fn iter_pos(&self) -> Self::Iter;
}

impl PosCollection for Pos {
    type Iter = Once<Pos>;

    fn iter_pos(&self) -> Self::Iter {
        once(*self)
    }
}

impl PosCollection for Range<Pos> {
    type Iter = Box<dyn Iterator<Item = Pos>>;

    fn iter_pos(&self) -> Self::Iter {
        let start = self.start;
        let end = self.end;
        let iter = iproduct!(start.x..end.x, start.y..end.y).map(|(x, y)| Pos { x, y });
        let res = Box::new(iter);
        res
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use quickcheck::*;

    use super::*;
    use rand::Rng;

    impl Arbitrary for Pos {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let max_pos: DimIndex = 1000;
            let x = g.gen_range(0, max_pos);
            let y = g.gen_range(0, max_pos);
            Pos { x, y }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let shrinker = (self.x, self.y).shrink();
            let iter = shrinker.map(|(x, y)| Pos { x, y });
            Box::new(iter)
        }
    }

    quickcheck! {
        fn test_iter(pos: Pos) -> bool {
            vec![pos] == pos.iter_pos().collect::<Vec<_>>()
        }

        fn test_range(pos1: Pos, pos2: Pos) -> TestResult {
            let expected = iproduct!(
                pos1.x..pos2.x,
                pos1.y..pos2.y
            ).map(|(x,y)| Pos {x, y}).collect::<HashSet<_>>();
            TestResult::from_bool(expected == (pos1..pos2).iter_pos().collect::<HashSet<_>>())
        }
    }
}
