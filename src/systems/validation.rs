use specs::Read;
use specs::ReadStorage;

use common::query::*;
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

    fn run(&self, move_dir: Dir, (pos_storage, pl, tile, level): Self::SD) -> Self::Output {
        use specs::Join;
        // TODO Create helpers for working with unique values
        let res: Option<Dir> = unique((&pos_storage, &pl))
            .unwrap()
            .map(|(player_pos, _)| { player_pos + move_dir })
            .filter(|new_pos| level.is_valid(&new_pos) && level[&new_pos].is_walkable())
            .filter(|new_pos| {
                let existing_entities = hash(&pos_storage, &tile);
                !existing_entities.contains_key(new_pos)
            })
            .map(|_| move_dir);
        res
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
