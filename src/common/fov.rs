use crate::data::structures::pos::PosCollection;
use crate::data::structures::*;
use std::ops::Index;
use tcod::map::*;

fn into_fov_map<M, C>(m: &M) -> Map
where
    C: CellObject,
    M: Index<Pos, Output = C> + HasDim,
{
    let mut map = Map::new(m.width().into(), m.height().into());
    for p in (Pos::default()..=m.max_pos()).iter_pos() {
        map.set(
            p.x.into(),
            p.y.into(),
            !m[p].blocks_sight(),
            m[p].is_walkable(),
        )
    }
    map
}
