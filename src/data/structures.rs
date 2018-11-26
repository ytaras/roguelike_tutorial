#[derive(Debug, Default)]
pub struct LevelInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum GameCommand {
    Exit,
}
