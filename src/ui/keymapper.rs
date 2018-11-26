use data::structures::*;
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
}
