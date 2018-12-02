use std::ops::{Index, IndexMut};

use data::structures::matrix::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    WALL,
    GROUND,
}

pub trait CellObject {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
}

impl CellObject for TileType {
    fn is_walkable(&self) -> bool {
        match self {
            TileType::WALL => false,
            _ => true,
        }
    }

    fn blocks_sight(&self) -> bool {
        unimplemented!()
    }
}

impl Default for TileType {
    fn default() -> Self {
        TileType::WALL
    }
}

#[derive(Debug, Default)]
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

impl LevelInfo {
    pub fn new(width: DimIndex, height: DimIndex) -> Self {
        LevelInfo {
            data: Matrix::new(width, height),
        }
    }

    pub fn with_dim(dim: Pos) -> Self {
        Self::new(dim.x, dim.y)
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
}
