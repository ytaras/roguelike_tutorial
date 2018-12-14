use crate::data::components::IsFighter;
use specs::Entities;
use specs::LazyUpdate;
use specs::Read;
use specs::ReadStorage;
use specs::System;

pub struct Clean;

impl<'a> System<'a> for Clean {
    type SystemData = (
        Read<'a, LazyUpdate>,
        Entities<'a>,
        ReadStorage<'a, IsFighter>,
    );

    fn run(&mut self, (lazy, entity, hp): <Self as System<'a>>::SystemData) {
        use specs::Join;
        for (entity, hp) in (&entity, &hp).join() {
            if hp.is_dead() {
                log::info!("Entity {:?} - {:?} is dead, removing", entity, hp);
                lazy.exec_mut(move |w| {
                    log::trace!("Removing entity {:?}", entity);
                    if let Err(e) = w.delete_entity(entity) {
                        log::error!("Error removing entity {:?}", e);
                    }
                });
            }
        }
    }
}
