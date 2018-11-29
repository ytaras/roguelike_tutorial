use specs::Read;
use specs::ReadStorage;

use common::validations::Validation;
use data::components::*;
use data::structures::{CellObject, Dir, LevelInfo};
use data::structures::Pos;

#[derive(Debug, Default)]
struct MoveValidation;

impl<'a> Validation<'a> for MoveValidation {
    type Input = Dir;
    type Output = Option<Pos>;
    type SD = (ReadStorage<'a, Pos>,
               ReadStorage<'a, IsPlayer>,
               Read<'a, LevelInfo>,
    );

    fn run(&self, i: Dir, (pos, pl, level): Self::SD) -> Self::Output {
        use specs::Join;
        for (pos, _) in (&pos, &pl).join() {
            let new_pos: Pos = pos + i;
            println!("player in {:?}, going to {:?} in {:?}", pos, new_pos, *level);
            if !level.is_valid(&new_pos) || !level[&new_pos].is_walkable() {
                return None;
            }
            return Some(new_pos);
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
    use data::structures::TileType::WALL;

    use super::*;

    fn create_world() -> World {
        let mut w = World::new();
        let mut level = LevelInfo::new(1, 2);
        level[&Pos { x: 0, y: 1 }] = WALL;
        w.add_resource(level);
        w.register::<IsPlayer>();
        w.register::<Pos>();
        w.create_entity()
            .with(IsPlayer)
            .with(Pos { x: 0, y: 0 })
            .build();
        w
    }

    #[test]
    fn dont_allow_to_walk_into_wall() {
        let mut w = create_world();
        let result = MoveValidation.exec(S, &mut w);
        assert!(result.is_none());
    }

    #[test]
    fn dont_allow_to_walk_out_of_bounds() {
        let mut w = create_world();
        let result = MoveValidation.exec(E, &mut w);
        assert!(result.is_none());
    }
}