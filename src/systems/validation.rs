use specs::Read;
use specs::ReadStorage;

use common::validations::Validation;
use data::components::*;
use data::structures::{CellObject, Dir, LevelInfo};
use data::structures::Pos;

#[derive(Debug, Default)]
pub struct MoveValidation;

impl<'a> Validation<'a> for MoveValidation {
    type Input = Dir;
    type Output = Option<Dir>;
    type SD = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, IsPlayer>,
        ReadStorage<'a, TakesWholeTile>,
        Read<'a, LevelInfo>,
    );

    fn run(&self, i: Dir, (pos_storage, pl, tile, level): Self::SD) -> Self::Output {
        use specs::Join;
        // TODO Create helpers for working with unique values
        let mut iter = (&pos_storage, &pl).join();
        if let Some((pos, _)) = iter.next() {
            let new_pos: Pos = pos + i;
            println!(
                "player in {:?}, going to {:?} in {:?}",
                pos, new_pos, *level
            );
            if !level.is_valid(&new_pos) || !level[&new_pos].is_walkable() {
                return None;
            }
            for (other_pos, _) in (&pos_storage, &tile).join() {
                if other_pos == &new_pos {
                    return None
                }
            }
            return Some(i);
        }
        panic!("Player not found");
    }
}

#[cfg(test)]
mod tests {
    use specs::{Builder, World};

    use data::components::*;
    use data::structures::{E, S};
    use data::structures::LevelInfo;
    use data::structures::TileType::*;
    use systems::render::YELLOW;

    use super::*;

    fn create_world(add_wall: bool) -> World {
        let mut w = World::new();
        let mut level = LevelInfo::new(1, 2);
        if add_wall {
            level[&Pos { x: 0, y: 1 }] = WALL;
        }
        w.add_resource(level);
        w.register::<IsPlayer>();
        w.register::<IsVisible>();
        w.register::<TakesWholeTile>();
        w.register::<Pos>();
        w.create_entity()
            .with(IsPlayer)
            .with(Pos { x: 0, y: 0 })
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
    fn dont_allow_to_walk_into_someone() {
        let mut w = create_world(false);
        w.create_entity()
            .with_actor_components('@', YELLOW, Pos { x: 0, y: 1 })
            .build();
        let result = MoveValidation.exec(S, &mut w);
        assert!(result.is_none());
    }

    #[test]
    fn allow_to_walk_on_the_ground() {
        let mut w = create_world(false);
        let result = MoveValidation.exec(S, &mut w);
        assert_eq!(Some(S), result);
    }
}
