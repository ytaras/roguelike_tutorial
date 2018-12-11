use std::ops::{Index, IndexMut};

use crate::data::structures::matrix::*;

pub mod corridor;
pub mod room;

pub use self::corridor::*;
pub use self::room::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Ground,
    RoomWall,
}

pub trait CellObject {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
}

impl CellObject for TileType {
    fn is_walkable(&self) -> bool {
        match self {
            TileType::Wall => false,
            TileType::RoomWall => false,
            _ => true,
        }
    }

    fn blocks_sight(&self) -> bool {
        match self {
            TileType::Wall => true,
            TileType::RoomWall => true,
            _ => false,
        }
    }
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Wall
    }
}

#[derive(Debug, Default, Clone)]
pub struct LevelInfo {
    data: Matrix<TileType>,
}

impl<'a> Index<Pos> for LevelInfo {
    type Output = TileType;
    fn index(&self, i: Pos) -> &TileType {
        &self.data[i]
    }
}

impl<'a> IndexMut<Pos> for LevelInfo {
    fn index_mut(&mut self, i: Pos) -> &mut TileType {
        &mut self.data[i]
    }
}

impl PosCollection for LevelInfo {
    type Iter = <Matrix<TileType> as PosCollection>::Iter;

    fn iter_pos(&self) -> Self::Iter {
        self.data.iter_pos()
    }
}

pub trait HasWall {
    type Iter: Iterator<Item = Pos>;

    fn walls(&self) -> Self::Iter;
}

impl LevelInfo {
    pub fn new(width: DimIndex, height: DimIndex) -> Self {
        Self::from_matrix(Matrix::new(width, height))
    }

    pub fn from_matrix(data: Matrix<TileType>) -> Self {
        LevelInfo { data }
    }

    pub fn with_dim(dim: Dim) -> Self {
        Self::new(dim.width, dim.height)
    }

    pub fn width(&self) -> DimIndex {
        self.data.width()
    }

    pub fn height(&self) -> DimIndex {
        self.data.height()
    }

    pub fn all_cells(&self) -> MatrixIter<TileType> {
        self.data.iter()
    }

    pub fn is_valid(&self, p: Pos) -> bool {
        self.data.is_valid(p)
    }

    pub fn dim(&self) -> Dim {
        self.data.dim()
    }

    pub fn max_pos(&self) -> Pos {
        self.dim().max_pos()
    }
}
