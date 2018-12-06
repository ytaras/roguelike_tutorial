use std::iter::Enumerate;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::slice::Iter;

pub use super::pos::*;

type InternalIndex = usize;

#[derive(Debug, Clone, Default)]
pub struct Matrix<T> {
    width: DimIndex,
    height: DimIndex,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn max_pos(&self) -> InternalIndex {
        self.width as InternalIndex * self.height as InternalIndex
    }

    fn to_pos(&self, i: InternalIndex) -> Pos {
        assert!(i < self.max_pos());
        let x: DimIndex = (i % self.width as InternalIndex) as DimIndex;
        let y: DimIndex = (i / self.width as InternalIndex) as DimIndex;
        Pos { x, y }
    }
    fn to_index(&self, pos: Pos) -> InternalIndex {
        assert!(self.is_valid(pos));
        pos.x as InternalIndex + pos.y as InternalIndex * self.width as InternalIndex
    }

    pub fn is_valid(&self, p: Pos) -> bool {
        p.x < self.width && p.y < self.height
    }

    pub fn height(&self) -> DimIndex {
        self.height
    }
    pub fn width(&self) -> DimIndex {
        self.width
    }

    pub fn iter(&self) -> MatrixIter<T> {
        let inner = self.data.iter().enumerate();
        MatrixIter {
            inner,
            matrix: self,
        }
    }

    pub fn dim(&self) -> Dim {
        Dim {
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(width: DimIndex, height: DimIndex) -> Self {
        let data_size: InternalIndex = width as InternalIndex * height as InternalIndex;
        Matrix {
            width,
            height,
            data: vec![T::default(); data_size],
        }
    }
}

impl<'a, T> Index<Pos> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &T {
        let i = self.to_index(pos);
        &self.data[i]
    }
}

impl<'a, T> IndexMut<Pos> for Matrix<T> {
    fn index_mut(&mut self, p: Pos) -> &mut T {
        let i = self.to_index(p);
        &mut self.data[i]
    }
}

#[derive(Debug)]
pub struct MatrixIter<'a, T: 'a> {
    inner: Enumerate<Iter<'a, T>>,
    matrix: &'a Matrix<T>,
}

impl<'a, T> Iterator for MatrixIter<'a, T> {
    type Item = (Pos, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(i, t)| {
            let pos = self.matrix.to_pos(i as InternalIndex);
            (pos, t)
        })
    }
}

impl<T> PosCollection for Matrix<T> {
    type Iter = <Range<Pos> as PosCollection>::Iter;

    fn iter_pos(&self) -> <Self as PosCollection>::Iter {
        let from = Pos { x: 0, y: 0 };
        let to = Pos {
            x: self.width,
            y: self.height,
        };

        (from..to).iter_pos()
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use itertools::*;
    use proptest::prelude::*;

    use super::*;

    impl<T: Debug + Default + Clone> Arbitrary for Matrix<T> {
        type Parameters = ();
        fn arbitrary_with(_args: <Self as Arbitrary>::Parameters) -> <Self as Arbitrary>::Strategy {
            matrix(DimIndex::max_value(), DimIndex::max_value())
        }
        type Strategy = BoxedStrategy<Matrix<T>>;
    }

    fn matrix<T: Debug + Default + Clone>(
        max_width: DimIndex,
        max_height: DimIndex,
    ) -> BoxedStrategy<Matrix<T>> {
        (1..max_width, 1..max_height)
            .prop_map(|(w, h)| Matrix::new(w, h))
            .boxed()
    }

    prop_compose! {
        fn pos(max_x: DimIndex, max_y: DimIndex)(x in 0..max_x, y in 0..max_y) -> Pos {
            Pos { x, y }
        }
    }

    fn matrix_and_pos<T: 'static + Debug + Default + Clone>() -> BoxedStrategy<(Matrix<T>, Pos)> {
        matrix_and_pos_limit(DimIndex::max_value(), DimIndex::max_value())
    }

    fn matrix_and_pos_limit<T: 'static + Debug + Default + Clone>(
        max_width: DimIndex,
        max_height: DimIndex,
    ) -> BoxedStrategy<(Matrix<T>, Pos)> {
        matrix(max_width, max_height)
            .prop_flat_map(|m| {
                let pos = pos(m.width, m.height);
                (Just(m), pos)
            })
            .boxed()
    }

    proptest! {
        #[test]
        fn max_pos_no_overflow(m: Matrix<bool>) {
            let _ = m.max_pos();
        }

        #[test]
        fn pos_to_index((m, pos) in matrix_and_pos::<bool>()) {
            prop_assert_eq!(pos, m.to_pos(m.to_index(pos)))
        }

        #[test]
        fn index_by_pos((m, pos) in matrix_and_pos::<bool>()) {
            prop_assert_eq!(bool::default(), m[pos])
        }


        #[test]
        fn mut_index_by_pos((m, pos) in matrix_and_pos::<bool>())  {
            let mut m = m;
            let new_value = !bool::default();
            m[pos] = new_value;
            prop_assert_eq!(new_value, m[pos])
        }

        #[test]
        fn iter(m: Matrix<bool>)  {
            let expected_pairs =
                iproduct!(0..m.height, 0..m.width)
                    .map(|(y, x)| Pos {x , y})
                    .collect::<Vec<_>>();
            let real_pairs = m.iter()
                .map(|(p, _)| p)
                .collect::<Vec<_>>();
            prop_assert_eq!(expected_pairs, real_pairs);
        }

        #[test]
        fn iter_pos_has_only_unique(m: Matrix<bool>) {
            let positions_count = m.iter_pos().count();
            let expected_count = m.width as InternalIndex * m.height as InternalIndex;
            prop_assert_eq!(expected_count, positions_count);
            let expected_pairs = iproduct!(0..m.width, 0..m.height) .map(|(x, y)| Pos {x , y});
            assert_equal(expected_pairs, m.iter_pos());
        }

        #[test]
        fn iter_pos_returs_only_valid(m: Matrix<bool>) {
            for p in m.iter_pos() {
                prop_assert!(m.is_valid(p));
            }
        }

    }
}
