pub type DimIndex = u8;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Pos {
    pub x: DimIndex,
    pub y: DimIndex,
}
