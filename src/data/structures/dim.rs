use crate::data::structures::DimIndex;
use crate::data::structures::Pos;

#[derive(Debug, Copy, Clone)]
pub struct Dim {
    pub width: DimIndex,
    pub height: DimIndex,
}

impl Dim {
    pub fn max_pos(self) -> Pos {
        Pos {
            x: self.width - 1,
            y: self.height - 1,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::data::structures::pos::test::*;
    use proptest::prelude::*;

    pub fn nonzero_dim(max_dim: Dim) -> BoxedStrategy<Dim> {
        nonzero_pos_in_dim(max_dim)
            .prop_map(|p| Dim {
                width: p.x + 1,
                height: p.y + 1,
            })
            .boxed()
    }
    fn dim(max_dim: Dim) -> BoxedStrategy<Dim> {
        (0..=max_dim.width, 0..=max_dim.height)
            .prop_map(|(width, height)| Dim { width, height })
            .boxed()
    }
}
