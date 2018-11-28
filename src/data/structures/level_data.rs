#[derive(Debug)]
enum TileType {
    WALL,
    GROUND,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::GROUND
    }
}

use data::structures::matrix::*;

#[derive(Debug, Default)]
pub struct LevelInfo {
    data: Matrix<TileType>,
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
}
