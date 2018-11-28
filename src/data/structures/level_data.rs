use data::structures::matrix::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub enum TileType {
    WALL,
    GROUND,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::GROUND
    }
}

#[derive(Debug, Default)]
pub struct LevelInfo {
    data: Matrix<TileType>,
}

impl<'a> Index<&'a Pos> for LevelInfo {
    type Output = TileType;
    fn index(&self, i: &Pos) -> &TileType {
        &self.data[i]
    }
}
impl<'a> IndexMut<&'a Pos> for LevelInfo {
    fn index_mut(&mut self, i: &Pos) -> &mut TileType {
        &mut self.data[i]
    }
}

impl LevelInfo {
    pub fn new(width: DimIndex, height: DimIndex) -> Self {
        LevelInfo {
            data: Matrix::new(width, height),
        }
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
}
