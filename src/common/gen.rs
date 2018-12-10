use rand::Rng;

pub trait Gen {
    type Param;
    fn create<G>(rng: &mut G, param: Self::Param) -> Self
    where
        G: Rng;
}
