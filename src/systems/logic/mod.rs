use specs::prelude::*;

use data::components::*;
use std::marker::PhantomData;

pub struct AssertUnique<T: Component> {
    component_type: PhantomData<T>,
}

impl<T: Component> AssertUnique<T> {
    pub fn new() -> Self {
        AssertUnique {
            component_type: PhantomData,
        }
    }
}

impl<'a, T: Component> System<'a> for AssertUnique<T> {
    type SystemData = ReadStorage<'a, T>;
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        assert_eq!(1, data.join().count());
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_if_unique() {
        use specs::RunNow;
        let mut w = build_world();
        w.create_entity().with(IsPlayer).build();
        AssertUnique::<IsPlayer>::new().run_now(&mut w.res);
    }

    #[test]
    #[should_panic]
    fn fail_on_missing() {
        let mut w = build_world();
        AssertUnique::<IsPlayer>::new().run_now(&mut w.res);
    }

    #[test]
    #[should_panic]
    fn fail_on_many() {
        use specs::RunNow;

        let mut w = build_world();
        w.create_entity().with(IsPlayer).build();
        w.create_entity().with(IsPlayer).build();
        AssertUnique::<IsPlayer>::new().run_now(&mut w.res);
    }
    fn build_world() -> World {
        let mut w = World::new();
        w.register::<IsPlayer>();
        w
    }
}
