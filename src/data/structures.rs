#[derive(Debug, Default)]
pub struct LevelInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum GameCommand {
    Exit,
}
#[derive(Debug, Clone)]
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

const N: Dir = Dir {
    ns: MoveDir::Minus,
    ew: MoveDir::Zero,
};
const S: Dir = Dir {
    ns: MoveDir::Plus,
    ew: MoveDir::Zero,
};
const W: Dir = Dir {
    ns: MoveDir::Zero,
    ew: MoveDir::Plus,
};
const E: Dir = Dir {
    ns: MoveDir::Zero,
    ew: MoveDir::Minus,
};

#[derive(Debug, Clone)]
pub struct Dir {
    ns: MoveDir,
    ew: MoveDir,
}

#[derive(Debug, Clone)]
enum MoveDir {
    Minus,
    Zero,
    Plus,
}
