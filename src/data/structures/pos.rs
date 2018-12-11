use itertools::iproduct;
use std::cmp::Ordering;
use std::ops::Range;
use std::ops::RangeInclusive;

pub type DimIndex = u8;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Default)]
pub struct Pos {
    pub x: DimIndex,
    pub y: DimIndex,
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_w: u16 = u16::from(self.x) + u16::from(self.y);
        let other_w: u16 = u16::from(other.x) + u16::from(other.y);
        self_w.partial_cmp(&other_w)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_w: u16 = u16::from(self.x) + u16::from(self.y);
        let other_w: u16 = u16::from(other.x) + u16::from(other.y);
        self_w.cmp(&other_w)
    }
}

impl Pos {
    pub fn n(self) -> Pos {
        Pos {
            y: self.y - 1,
            ..self
        }
    }
    pub fn s(self) -> Pos {
        Pos {
            y: self.y + 1,
            ..self
        }
    }
    pub fn w(self) -> Pos {
        Pos {
            x: self.x - 1,
            ..self
        }
    }
    pub fn e(self) -> Pos {
        Pos {
            x: self.x + 1,
            ..self
        }
    }
}

pub struct PosRange {
    from: Pos,
    to: Pos,
}

impl PosRange {
    fn new(p1: Pos, p2: Pos) -> Self {
        let from = Pos {
            x: DimIndex::min(p1.x, p2.x),
            y: DimIndex::min(p1.y, p2.y),
        };
        let to = Pos {
            x: DimIndex::max(p1.x, p2.x),
            y: DimIndex::max(p1.y, p2.y),
        };

        PosRange { from, to }
    }

    fn iter_inclusive(self) -> impl Iterator<Item = Pos> {
        iproduct!(self.from.x..=self.to.x, self.from.y..=self.to.y).map(|(x, y)| Pos { x, y })
    }

    fn iter(self) -> impl Iterator<Item = Pos> {
        iproduct!(self.from.x..self.to.x, self.from.y..self.to.y).map(|(x, y)| Pos { x, y })
    }
}

impl IntoIterator for PosRange {
    type Item = Pos;
    type IntoIter = Box<dyn Iterator<Item = Pos>>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let iter =
            iproduct!(self.from.x..=self.to.x, self.from.y..=self.to.y).map(|(x, y)| Pos { x, y });
        Box::new(iter)
    }
}

pub trait PosCollection {
    type Iter: Iterator<Item = Pos>;

    fn iter_pos(&self) -> Self::Iter;
}

impl PosCollection for Range<Pos> {
    type Iter = Box<dyn Iterator<Item = Pos>>;

    fn iter_pos(&self) -> Self::Iter {
        Box::new(PosRange::new(self.start, self.end).iter())
    }
}

impl PosCollection for RangeInclusive<Pos> {
    type Iter = <PosRange as IntoIterator>::IntoIter;

    fn iter_pos(&self) -> Self::Iter {
        Box::new(PosRange::new(*self.start(), *self.end()).iter_inclusive())
    }
}

#[cfg(test)]
pub mod test {
    use itertools::*;
    use proptest::prelude::*;
    use proptest::{prop_assert, prop_assert_eq, prop_compose, proptest, proptest_helper};

    use super::*;
    use crate::data::structures::Dim;

    impl Arbitrary for Pos {
        type Parameters = ();

        fn arbitrary_with(_args: <Self as Arbitrary>::Parameters) -> <Self as Arbitrary>::Strategy {
            arb_pos().boxed()
        }

        type Strategy = BoxedStrategy<Pos>;
    }

    pub const MAX_DIM: Dim = Dim {
        width: DimIndex::max_value(),
        height: DimIndex::max_value(),
    };

    pub const SMALL_DIM: Dim = Dim {
        width: 100,
        height: 100,
    };
    pub fn pos(start: Pos, end: Pos) -> BoxedStrategy<Pos> {
        (start.x..=end.x, start.y..=end.y)
            .prop_map(|(x, y)| Pos { x, y })
            .boxed()
    }
    pub fn nonzero_pos_in_dim(d: Dim) -> BoxedStrategy<Pos> {
        pos(
            Pos { x: 1, y: 1 },
            Pos {
                x: d.width - 1,
                y: d.height - 1,
            },
        )
    }

    pub fn pos_in_dim(d: Dim) -> BoxedStrategy<Pos> {
        (0..d.width, 0..d.height)
            .prop_map(|(x, y)| Pos { x, y })
            .boxed()
    }

    prop_compose! {
        fn arb_pos()(x: DimIndex, y: DimIndex) -> Pos {
            Pos {x, y}
        }
    }

    proptest! {
        #[test]
        fn pos_range_inclusive_unique_only(from: Pos, to: Pos) {
            let range = from..=to;
            let real_count = range.iter_pos().count();
            let unique_count = range.iter_pos().unique().count();
            prop_assert_eq!(real_count, unique_count);
        }

        #[test]
        fn pos_range_inclusive_holds(from: Pos, to: Pos) {
            let range = from..=to;
            let real_pairs = range
                .iter_pos()
                .collect::<Vec<_>>();

            let x_range = DimIndex::min(from.x, to.x)..=DimIndex::max(from.x, to.x);
            let y_range = DimIndex::min(from.y, to.y)..=DimIndex::max(from.y, to.y);

             prop_assert_eq!(x_range.len() * y_range.len(), real_pairs.len());

            let values_are_valid = range.iter_pos().all(|p| {
                p.x >= *x_range.start() && p.x <= *x_range.end() &&
                    p.y >= *y_range.start() && p.y <= *y_range.end()
            });
            prop_assert!(values_are_valid, "{:?} doesn't hold x or y invariant for {:?}",
                &real_pairs,
                 range);
        }


        #[test]
        fn pos_range_unique_only(from: Pos, to: Pos) {
            let range = from..to;
            let real_count = range.iter_pos().count();
            let unique_count = range.iter_pos().unique().count();
            prop_assert_eq!(real_count, unique_count);
        }

        #[test]
        fn pos_range_holds(from: Pos, to: Pos) {
            let range = from..to;
            let real_pairs = range
                .iter_pos()
                .collect::<Vec<_>>();

            let x_range: Range<DimIndex> = DimIndex::min(from.x, to.x)..DimIndex::max(from.x, to.x);
            let y_range: Range<DimIndex> = DimIndex::min(from.y, to.y)..DimIndex::max(from.y, to.y);

             prop_assert_eq!(x_range.len() * y_range.len(), real_pairs.len());

            let values_are_valid = range.iter_pos().all(|p| {
                p.x >= x_range.start && p.x < x_range.end &&
                    p.y >= y_range.start && p.y < y_range.end
            });
            prop_assert!(values_are_valid, "{:?} doesn't hold x or y invariant for {:?}",
                &real_pairs,
                 range);
        }
    }
}
