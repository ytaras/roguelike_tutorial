use specs::Entities;
use specs::Read;
use specs::ReadStorage;
use specs::System;

use crate::data::components::*;
use crate::data::structures::*;

pub struct GetAiCommand;

impl<'a> System<'a> for GetAiCommand {
    type SystemData = (Read<'a, LevelInfo>, ReadStorage<'a, HasBrain>, Entities<'a>);
    fn run(&mut self, (_level, brain, entity): <Self as System<'a>>::SystemData) {
        use specs::Join;
        for (_brain, e) in (&brain, &entity).join() {
            log::debug!(
                "Entity {:?} thinks of what doing next and stays stubborn",
                e
            );
        }
    }
}
