use crate::data::structures::*;
use log::trace;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct KeyMapper {
    game_commands: HashMap<String, Command>,
}
impl KeyMapper {
    pub fn new() -> Self {
        let mut game_commands = HashMap::new();
        game_commands.insert("Escape".to_string(), Command::exit());
        game_commands.insert("ArrowLeft".to_string(), Command::west());
        game_commands.insert("ArrowRight".to_string(), Command::east());
        game_commands.insert("ArrowUp".to_string(), Command::north());
        game_commands.insert("ArrowDown".to_string(), Command::south());
        KeyMapper { game_commands }
    }

    pub fn commands(&self) -> Iter<String, Command> {
        self.game_commands.iter()
    }

    #[cfg(feature = "render_tcod")]
    pub fn command(&self, key: tcod::input::Key) -> Option<&Command> {
        use tcod::input::KeyCode::*;
        let mnem = match key.code {
            Left => Some("ArrowLeft"),
            Right => Some("ArrowRight"),
            Up => Some("ArrowUp"),
            Down => Some("ArrowDown"),
            Escape => Some("Escape"),
            _ => None,
        };
        trace!("Maping for {:?} {:?}", key, mnem);
        mnem.and_then(|o| self.game_commands.get(o))
    }
}
