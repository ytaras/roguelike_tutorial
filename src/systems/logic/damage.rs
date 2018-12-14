use specs::prelude::*;

use crate::data::components::*;

pub struct ExecuteEffects;

impl<'a> System<'a> for ExecuteEffects {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HasEffectStack>,
        WriteStorage<'a, IsFighter>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (ent, dam, mut hp, lazy): <Self as System<'a>>::SystemData) {
        use specs::Join;

        for (e, hp, dam) in (&ent, &mut hp, &dam).join() {
            log::info!("Dealing {:?} to {:?}", dam, hp);
            hp.inflict_damage(dam.attack_power);
            log::trace!("Result: {:?}", hp);
            lazy.remove::<HasEffectStack>(e);
        }
    }
}
