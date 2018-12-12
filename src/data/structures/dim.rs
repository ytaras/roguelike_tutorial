use crate::data::structures::DimIndex;
use crate::data::structures::Pos;

#[derive(Debug, Copy, Clone)]
pub struct Dim {
    pub width: DimIndex,
    pub height: DimIndex,
}

pub trait HasDim {
    fn width(&self) -> DimIndex;
    fn height(&self) -> DimIndex;

    fn max_pos(&self) -> Pos {
        Pos {
            x: self.width() - 1,
            y: self.height() - 1,
        }
    }

    fn is_valid(&self, p: Pos) -> bool {
        p.x < self.width() && p.y < self.height()
    }
}

impl HasDim for Dim {
    fn width(&self) -> DimIndex {
        self.width
    }
    fn height(&self) -> DimIndex {
        self.height
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

    impl Arbitrary for Dim {
        type Parameters = ();

        fn arbitrary_with(_args: <Self as Arbitrary>::Parameters) -> <Self as Arbitrary>::Strategy {
            dim(MAX_DIM)
        }

        type Strategy = BoxedStrategy<Dim>;
    }
}
