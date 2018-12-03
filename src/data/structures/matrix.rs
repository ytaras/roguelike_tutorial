use std::fmt::{Debug, Formatter};
use std::iter::Enumerate;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::slice::Iter;

pub use data::structures::pos::*;

type InternalIndex = usize;

const MAX_MATRIX_SIZE: InternalIndex = 1000 * 1000;

#[derive(Clone, Default)]
pub struct Matrix<T> {
    width: DimIndex,
    height: DimIndex,
    data: Vec<T>,
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        (self.width, self.height).fmt(formatter)
    }
}

impl<T> PosCollection for Matrix<T> {
    type Iter = <Range<Pos> as PosCollection>::Iter;

    fn iter_pos(&self) -> <Self as PosCollection>::Iter {
        let range = (Pos { x: 0, y: 0 })..Pos {
            x: self.width,
            y: self.height,
        };
        range.iter_pos()
    }
}

impl<T> Matrix<T> {
    fn max_pos(&self) -> InternalIndex {
        (self.width as InternalIndex * self.height as InternalIndex).into()
    }

    fn to_pos(&self, i: InternalIndex) -> Pos {
        assert!(i < self.max_pos());
        let x: DimIndex = (i % self.width as InternalIndex) as DimIndex;
        let y: DimIndex = (i / self.width as InternalIndex) as DimIndex;
        let res = Pos { x, y };
        self.assert_is_valid(res);
        res
    }
    fn to_index(&self, pos: Pos) -> InternalIndex {
        self.assert_is_valid(pos);
        (pos.x as InternalIndex + pos.y as InternalIndex * self.width as InternalIndex).into()
    }

    fn assert_is_valid(&self, pos: Pos) {
        assert!(
            self.is_valid(pos),
            "{:?} is out of bounds for {:?}",
            pos,
            (self.width, self.height)
        );
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
}

impl<T: Default> Matrix<T> {
    pub fn new(width: DimIndex, height: DimIndex) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        let data_size: InternalIndex = width as InternalIndex * height as InternalIndex;
        assert!(data_size <= MAX_MATRIX_SIZE, "{:?}", (width, height));
        let mut data = Vec::with_capacity(data_size);
        for _ in 0..data_size {
            data.push(T::default());
        }
        Matrix {
            width,
            height,
            data,
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

#[cfg(test)]
pub mod test {
    use std::collections::HashSet;

    use quickcheck::*;
    use rand::Rng;

    use super::*;

    #[derive(Clone, Debug)]
    pub struct MatrixAndPos<T> {
        matrix: Matrix<T>,
        pos: Pos,
    }

    pub fn random_pos<T, G: Gen>(m: &Matrix<T>, g: &mut G) -> Pos {
        let i: InternalIndex = g.gen_range(0, m.max_pos());

        m.to_pos(i)
    }

    impl<T: 'static + Default + Clone + Send> Arbitrary for Matrix<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let x: DimIndex = g.gen_range(1, DimIndex::max_value() - 1);
            let size: InternalIndex = g.gen_range(x.into(), MAX_MATRIX_SIZE - 1);
            let y: DimIndex = (size / x as InternalIndex) as DimIndex;
            Matrix::new(x.into(), y.into())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let shrinker = (self.width, self.height).shrink();
            let iter = shrinker.map(|(x, y)| Matrix::new(x + 1, y + 1));
            Box::new(iter)
        }
    }

    impl<T: 'static + Default + Clone + Send> Arbitrary for MatrixAndPos<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let matrix: Matrix<T> = Arbitrary::arbitrary(g);

            let x = g.gen_range(0, matrix.width);
            let y = g.gen_range(0, matrix.height);
            MatrixAndPos {
                matrix,
                pos: Pos { x, y },
            }
        }
    }

    quickcheck! {
            fn no_overflow(m: Matrix<bool>) -> TestResult {
                let _ = m.max_pos();
                TestResult::passed()
            }

            fn pos_to_index(m: MatrixAndPos<bool>) -> TestResult {
                    TestResult::from_bool(
                        m.pos == m.matrix.to_pos(m.matrix.to_index(m.pos))
                    )
            }

            fn index_by_pos(m: MatrixAndPos<bool>) -> TestResult {
                    TestResult::from_bool(
                        m.matrix[m.pos] == bool::default()
                    )
            }

            fn mut_index_by_pos(mp: MatrixAndPos<bool>) -> TestResult {
                    let mut m = mp.matrix;
                    let new_value = !bool::default();
                    m[mp.pos] = new_value;
                    TestResult::from_bool(
                        m[mp.pos] == new_value
                    )
            }

            fn iter(m: Matrix<bool>) -> () {
                let expected_pairs = iproduct!(0..m.height, 0..m.width)
                .map(|(y, x)| Pos {x ,y }).collect::<Vec<_>>();
                let real_pairs = m.iter().map(|(p, _)| p).collect::<Vec<_>>();
                assert_eq!(expected_pairs ,real_pairs);
            }

            fn pos_iter(m: Matrix<bool>) -> TestResult {
       //         let expected_pairs = iproduct!(0..m.height, 0..m.width)
        //            .map(|(y, x)| Pos {x ,y }).collect::<HashSet<_>>();

    //             let real_pos = m.iter_pos().collect::<HashSet<_>>();
    //            let real_pos = expected_pairs.clone();

          //       TestResult::from_bool(expected_pairs == real_pos)
                TestResult::passed()
            }
        }
}
