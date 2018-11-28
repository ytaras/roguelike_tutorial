use std::ops::Index;

type DimIndex = u16;
type InternalIndex = usize;

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    x: DimIndex,
    y: DimIndex,
}

#[derive(Debug, Clone)]
struct Matrix<T> {
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

    quickcheck! {
        fn no_owerflow(m: Matrix<bool>) -> TestResult {
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
    }
}
