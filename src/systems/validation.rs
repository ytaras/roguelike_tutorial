use specs::prelude::*;

use crate::common::query::*;
use crate::common::validations::Validation;
use crate::data::components::*;
use crate::data::structures::*;
use crate::data::structures::{CellObject, Dir, LevelInfo};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MoveValidation;

impl<'a> Validation<'a> for MoveValidation {
    type Input = Dir;
    type Output = Option<ActorCommand>;
    type SD = (
        ReadStorage<'a, HasPos>,
        ReadStorage<'a, IsPlayer>,
        ReadStorage<'a, TakesWholeTile>,
        Entities<'a>,
        Read<'a, LevelInfo>,
    );

    fn run(&self, move_dir: Dir, (pos_storage, pl, tile, entity, level): Self::SD) -> Self::Output {
        let target_pos: Pos = singleton((&pos_storage, &pl))
            .map(|(ref mut player_pos, _)| player_pos.0 + move_dir)
            .unwrap();
        if !level.is_valid(target_pos) || !level[target_pos].is_walkable() {
            return None;
        }

        for (e, pos, _) in (&entity, &pos_storage, &tile).join() {
            if pos.0 == target_pos {
                return Some(ActorCommand::MeleeAttack {
                    pos: target_pos,
                    target: e,
                });
            }
        }
        Some(ActorCommand::Move(move_dir))
    }
}

#[cfg(test)]
mod tests {
    use specs::{Builder, World};

    use super::*;
    use crate::data::structures::LevelInfo;
    use crate::data::structures::Pos;
    use crate::data::structures::TileType::*;
    use crate::data::structures::{E, S};
    use tcod::colors::YELLOW;

    fn create_world(add_wall: bool) -> World {
        let mut w = World::new();
        let mut level = LevelInfo::new(1, 2);
        level[Pos { x: 0, y: 0 }] = Ground;
        level[Pos { x: 0, y: 1 }] = Ground;
        if add_wall {
            level[Pos { x: 0, y: 1 }] = Wall;
        }
        w.add_resource(level);
        w.register::<IsPlayer>();
        w.register::<IsVisible>();
        w.register::<TakesWholeTile>();
        w.register::<HasPos>();
        w.create_entity()
            .with(IsPlayer)
            .with(HasPos(Pos { x: 0, y: 0 }))
            .build();
        w
    }

    #[test]
    fn dont_allow_to_walk_into_wall() {
        let mut w = create_world(true);
        let result = MoveValidation.exec(S, &mut w);
        assert!(result.is_none());
    }

    #[test]
    fn dont_allow_to_walk_out_of_bounds() {
        let mut w = create_world(false);
        let result = MoveValidation.exec(E, &mut w);
        assert!(result.is_none());
    }

    #[test]
    fn converts_move_into_someone_to_atack() {
        let mut w = create_world(false);
        let target_pos = Pos { x: 0, y: 1 };
        let e = w
            .create_entity()
            .with_actor_components('@', YELLOW, target_pos)
            .build();
        let result = MoveValidation.exec(S, &mut w).unwrap();

        assert_eq!(
            result,
            ActorCommand::MeleeAttack {
                pos: target_pos,
                target: e
            }
        );
    }

    #[test]
    fn allow_to_walk_on_the_ground() {
        let mut w = create_world(false);
        let result = MoveValidation.exec(S, &mut w);
        assert_eq!(Some(ActorCommand::Move(S)), result);
    }
}
