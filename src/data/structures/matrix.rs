use std::iter::Enumerate;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::Iter;

use specs::prelude::*;
use specs_derive::*;

pub type DimIndex = u16;
type InternalIndex = usize;

#[derive(Debug, PartialEq, Clone, Component)]
pub struct Pos {
    pub x: DimIndex,
    pub y: DimIndex,
}

#[derive(Debug, Clone, Default)]
pub struct Matrix<T> {
    width: DimIndex,
    height: DimIndex,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn max_pos(&self) -> InternalIndex {
        (self.width * self.height).into()
    }

    fn to_pos(&self, i: InternalIndex) -> Pos {
        assert!(i < self.max_pos());
        let x: DimIndex = (i % self.width as InternalIndex) as DimIndex;
        let y: DimIndex = (i / self.width as InternalIndex) as DimIndex;
        Pos { x, y }
    }
    fn to_index(&self, pos: &Pos) -> InternalIndex {
        assert!(self.is_valid(&pos));
        (pos.x + pos.y * self.width).into()
    }

    pub fn is_valid(&self, p: &Pos) -> bool {
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
        let data_size: InternalIndex = (width * height).into();
        let mut data = Vec::new();
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

impl<'a, T> Index<&'a Pos> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: &Pos) -> &T {
        let i = self.to_index(pos);
        &self.data[i]
    }
}

impl<'a, T> IndexMut<&'a Pos> for Matrix<T> {
    fn index_mut(&mut self, p: &Pos) -> &mut T {
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
mod test {
    use quickcheck::*;

    use super::*;

    impl<T: 'static + Default + Clone + Send> Arbitrary for Matrix<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let x = DimIndex::arbitrary(g);
            let y = DimIndex::arbitrary(g);
            Matrix::new(x, y)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let shrinker = (self.width, self.height).shrink();
            let iter = shrinker.map(|(x, y)| Matrix::new(x, y));
            Box::new(iter)
        }
    }

    impl Arbitrary for Pos {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let (x, y) = <(DimIndex, DimIndex)>::arbitrary(g);
            Pos { x, y }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let shrinker = (self.x, self.y).shrink();
            let iter = shrinker.map(|(x, y)| Pos { x, y });
            Box::new(iter)
        }
    }

    quickcheck! {
        fn no_overflow(m: Matrix<bool>) -> TestResult {
            let _ = m.max_pos();
            TestResult::passed()
        }

        fn pos_to_index(pos: Pos, m: Matrix<bool>) -> TestResult {
            if m.is_valid(&pos) {
                TestResult::from_bool(
                    pos == m.to_pos(m.to_index(&pos))
                )
            } else {
                TestResult::discard()
            }
        }

        fn index_by_pos(pos: Pos, m: Matrix<bool>) -> TestResult {
            if m.is_valid(&pos) {
                TestResult::from_bool(
                    m[&pos] == bool::default()
                )
            } else {
                TestResult::discard()
            }
        }

        fn mut_index_by_pos(pos: Pos, m: Matrix<bool>) -> TestResult {
            if m.is_valid(&pos) {
                let mut m = m;
                let new_value = !bool::default();
                m[&pos] = new_value;
                TestResult::from_bool(
                    m[&pos] == new_value
                )
            } else {
                TestResult::discard()
            }
        }

        fn iter(m: Matrix<bool>) -> () {
            let expected_pairs = iproduct!(0..m.height, 0..m.width)
            .map(|(y, x)| Pos {x ,y }).collect::<Vec<_>>();
            let real_pairs = m.iter().map(|(p, _)| p).collect::<Vec<_>>();
            assert_eq!(expected_pairs ,real_pairs);
        }
    }
}
