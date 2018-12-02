use data::structures::generation::*;
use data::structures::*;

type Dim = Pos;

const MAP_DIM: Dim = Dim { x: 90, y: 45 };

pub fn level_one() -> LevelInfo {
    let room1 = Room::new(
        Pos { x: 20, y: 15 },
        Pos {
            x: 20 + 10,
            y: 15 + 15,
        },
    );
    let room2 = Room::new(
        Pos { x: 35, y: 15 },
        Pos {
            x: 35 + 10,
            y: 15 + 15,
        },
    );
    let mut res = LevelInfo::new(MAP_DIM.x, MAP_DIM.y);
    room1.dig(&mut res);
    room2.dig(&mut res);
    res
}
