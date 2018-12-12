use super::*;
use crate::common::fov::*;
use log::trace;
use std::collections::HashMap;

pub struct Fov {
    fov_validity_cache: HashMap<Entity, Pos>,
}

impl<'a> System<'a> for Fov {
    type SystemData = (
        Read<'a, LevelInfo>,
        Entities<'a>,
        ReadStorage<'a, HasPos>,
        WriteStorage<'a, HasVision>,
    );

    fn run(&mut self, (level, e, pos, mut vis): <Self as System<'a>>::SystemData) {
        use specs::Join;

        let level = &*level;

        for (e, pos, vis) in (&e, &pos, &mut vis).join() {
            let known_pos = self.fov_validity_cache.get(&e);
            trace!("Pos for {:?} is {:?}, current is {:?}", e, known_pos, pos.0);
            let is_cache_valid = match known_pos {
                Some(kp) => kp == &pos.0,
                None => false,
            };

            if !is_cache_valid {
                let new_fov = calculate_fov(level, pos.0, vis.radius);
                vis.set_fov(new_fov);
                self.fov_validity_cache.insert(e, pos.0);
            }
        }
    }
}

impl Default for Fov {
    fn default() -> Self {
        Fov {
            fov_validity_cache: HashMap::default(),
        }
    }
}
