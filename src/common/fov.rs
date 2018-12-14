use crate::data::structures::matrix::Matrix;
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

fn from_fov_map(fov_map: Map) -> Matrix<bool> {
    let (width, height) = fov_map.size();
    let dim = Dim {
        width: width as DimIndex,
        height: height as DimIndex,
    };
    Matrix::tabulate(dim, |pos| fov_map.is_in_fov(pos.x.into(), pos.y.into()))
}

pub fn calculate_fov<M, C>(m: &M, pos: Pos, sight_radius: DimIndex) -> Matrix<bool>
where
    C: CellObject,
    M: Index<Pos, Output = C> + HasDim,
{
    // TODO(#24) - There's a possibility to have smaller memory pressure to calculate fov only
    // for what's in radius
    let mut fov_map = into_fov_map(m);
    fov_map.compute_fov(
        pos.x.into(),
        pos.y.into(),
        sight_radius.into(),
        true,
        FovAlgorithm::Basic,
    );

    from_fov_map(fov_map)
}
