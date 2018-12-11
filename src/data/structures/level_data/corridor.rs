use crate::data::structures::pos::PosCollection;
use crate::data::structures::DimIndex;
use crate::data::structures::Pos;
use std::cmp::max;
use std::cmp::min;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct LCorridor {
    pub from: Pos,
    pub to: Pos,
    pub horizontal_first: bool,
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
pub mod test {}
