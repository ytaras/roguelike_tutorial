use std::marker::PhantomData;

use specs::prelude::*;

use data::components::*;
use data::structures::*;

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

pub struct ExecuteCommands;

impl<'a> System<'a> for ExecuteCommands {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, HasPos>,
        WriteStorage<'a, PlansExecuting>,
        Read<'a, LazyUpdate>,
    );
    fn run(&mut self, (e, mut pos, mut plan_storage, lu): Self::SystemData) {
        use specs::Join;

        for (e, mut pos, plan) in (&e, &mut pos, &mut plan_storage).join() {
            let mut pos = &mut pos.0;
            match plan.0 {
                ActorCommand::Move(ref dir) => {
                    pos += dir;
                }
            }
            lu.remove::<PlansExecuting>(e);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    mod assert_unique {
        use super::*;

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

    mod execute_planned_commands {
        use super::*;

        #[test]
        fn move_moves() {
            let mut w = World::new();
            w.register::<HasPos>();
            w.register::<PlansExecuting>();

            let e = w
                .create_entity()
                .with(HasPos(Pos { x: 1, y: 1 }))
                .with(PlansExecuting::new(ActorCommand::Move(S)))
                .build();

            ExecuteCommands.run_now(&w.res);

            w.maintain();

            assert_eq!(
                w.read_storage::<HasPos>().get(e),
                Some(&HasPos(Pos { x: 1, y: 2 }))
            );
            assert_eq!(w.read_storage::<PlansExecuting>().get(e), None);
        }
    }
}
