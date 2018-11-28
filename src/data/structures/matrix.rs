use std::ops::Index;
use std::ops::IndexMut;

pub type DimIndex = u16;
type InternalIndex = usize;

#[derive(Debug, PartialEq, Clone)]
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
        let x: DimIndex = (i % self.width as InternalIndex) as DimIndex;
        let y: DimIndex = (i / self.width as InternalIndex) as DimIndex;
        Pos { x, y }
    }
    fn to_index(&self, pos: &Pos) -> InternalIndex {
        assert!(self.is_valid(&pos));
        (pos.x + pos.y * self.width).into()
    }

    fn is_valid(&self, p: &Pos) -> bool {
        p.x < self.width && p.y < self.height
    }

    pub fn height(&self) -> DimIndex {
        self.height
    }
    pub fn width(&self) -> DimIndex {
        self.width
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

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::*;

    impl<T: 'static + Default + Clone + Send> Arbitrary for Matrix<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let x = DimIndex::arbitrary(g);
            let y = DimIndex::arbitrary(g);
            Matrix::new(x, y)
        }
    }
    impl Arbitrary for Pos {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let (x, y) = <(DimIndex, DimIndex)>::arbitrary(g);
            Pos { x, y }
        }
    }

    fn changing() {
        let mut m: Matrix<bool> = Matrix::new(2, 3);
        let pos = Pos { x: 1, y: 2 };
        assert_eq!(bool::default(), m[&pos]);
        m[&pos] = !bool::default();
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
    }
}
