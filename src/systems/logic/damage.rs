use specs::prelude::*;

use crate::data::components::HasEffectStack;

pub struct ExecuteDamage;

impl<'a> System<'a> for ExecuteDamage {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HasEffectStack>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (ent, dam, lazy): <Self as System<'a>>::SystemData) {
        use specs::Join;

        for (e, _) in (&ent, &dam).join() {
            log::info!("Removing {:?}", e);
            lazy.exec_mut(move |world| {
                world.delete_entity(e).unwrap();
            });
        }
    }
}
