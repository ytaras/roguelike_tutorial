use std::ops::Range;
use std::ops::RangeInclusive;

pub type DimIndex = u8;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: DimIndex,
    pub y: DimIndex,
}

#[derive(Debug)]
pub struct Dim {
    pub width: DimIndex,
    pub height: DimIndex,
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
        let iter =
            iproduct!(self.from.x..=self.to.x, self.from.y..=self.to.y).map(|(x, y)| Pos { x, y });
        iter
    }

    fn iter(self) -> impl Iterator<Item = Pos> {
        let iter =
            iproduct!(self.from.x..self.to.x, self.from.y..self.to.y).map(|(x, y)| Pos { x, y });
        iter
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
mod test {
    use itertools::*;
    use proptest::prelude::*;

    use super::*;

    impl Arbitrary for Pos {
        type Parameters = ();

        fn arbitrary_with(_args: <Self as Arbitrary>::Parameters) -> <Self as Arbitrary>::Strategy {
            arb_pos().boxed()
        }

        type Strategy = BoxedStrategy<Pos>;
    }
    fn pos_in_dim(d: Dim) -> BoxedStrategy<Pos> {
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
