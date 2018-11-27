use specs::prelude::*;

use std::marker::PhantomData;

#[derive(Default)]
pub struct AssertUnique<T: Component> {
    component_type: PhantomData<T>,
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
    use data::components::*;

    fn assert_unique<T: Component + Default>(r: &mut Resources) {
        let mut system: AssertUnique<T> = Default::default();
        system.run_now(r);
    }

    #[test]
    fn passes_if_unique() {
        let mut w = build_world();
        w.create_entity().with(IsPlayer).build();
        assert_unique::<IsPlayer>(&mut w.res);
    }

    #[test]
    #[should_panic]
    fn fail_on_missing() {
        let mut w = build_world();
        assert_unique::<IsPlayer>(&mut w.res);
    }

    #[test]
    #[should_panic]
    fn fail_on_many() {
        let mut w = build_world();
        w.create_entity().with(IsPlayer).build();
        w.create_entity().with(IsPlayer).build();
        assert_unique::<IsPlayer>(&mut w.res);
    }
    fn build_world() -> World {
        let mut w = World::new();
        w.register::<IsPlayer>();
        w
    }
}
