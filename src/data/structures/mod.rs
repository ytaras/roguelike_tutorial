use std::convert::From;

pub use self::level_data::*;
pub use self::matrix::{Dim, DimIndex, HasDim, Pos};

pub mod dim;
mod level_data;
pub mod matrix;
pub mod pos;

#[derive(Debug)]
pub enum GameCommand {
    Exit,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ActorCommand {
    Move(Dir),
}

#[derive(Debug)]
pub enum Command {
    GameCommand(GameCommand),
    PlayerCommand(ActorCommand),
}

impl Command {
    pub fn exit() -> Command {
        Command::GameCommand(GameCommand::Exit)
    }
    fn move_dir(dir: Dir) -> Command {
        Command::PlayerCommand(ActorCommand::Move(dir))
    }
    pub fn west() -> Command {
        Command::move_dir(W)
    }
    pub fn east() -> Command {
        Command::move_dir(E)
    }
    pub fn south() -> Command {
        Command::move_dir(S)
    }
    pub fn north() -> Command {
        Command::move_dir(N)
    }
}

// TODO - Better syntax
pub const N: Dir = Dir {
    ns: MoveDir::Minus,
    ew: MoveDir::Zero,
};
pub const S: Dir = Dir {
    ns: MoveDir::Plus,
    ew: MoveDir::Zero,
};
pub const W: Dir = Dir {
    ns: MoveDir::Zero,
    ew: MoveDir::Minus,
};
pub const E: Dir = Dir {
    ns: MoveDir::Zero,
    ew: MoveDir::Plus,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir {
    pub ns: MoveDir,
    pub ew: MoveDir,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveDir {
    Minus,
    Zero,
    Plus,
}

impl MoveDir {
    pub fn apply(self, i: DimIndex) -> DimIndex {
        match self {
            MoveDir::Minus => i - 1,
            MoveDir::Plus => i + 1,
            MoveDir::Zero => i,
        }
    }
    pub fn to_int(self) -> i8 {
        self.to_num()
    }
    pub fn to_num<T: From<i8>>(self) -> T {
        match self {
            MoveDir::Minus => T::from(-1),
            MoveDir::Plus => T::from(1),
            MoveDir::Zero => T::from(0),
        }
    }
}
