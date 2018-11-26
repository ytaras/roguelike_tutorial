use data::structures::GameCommand;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct KeyMapper {
    game_commands: HashMap<String, GameCommand>,
}
impl KeyMapper {
    pub fn new() -> Self {
        let mut game_commands = HashMap::new();
        game_commands.insert("Escape".to_string(), GameCommand::Exit);
        KeyMapper { game_commands }
    }

    pub fn commands(&self) -> Iter<String, GameCommand> {
        self.game_commands.iter()
    }
}
