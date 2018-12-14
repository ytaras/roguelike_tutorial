use specs::World;

pub struct Goals {
    goal_stack: Vec<AiGoal>,
}

impl Goals {
    pub fn bored() -> Self {
        Goals {
            goal_stack: vec![AiGoal::Bored],
        }
    }
}

pub enum AiGoal {
    Bored,
}

impl Goal for AiGoal {}

pub trait Goal {}

impl AiGoal {}

#[derive(Debug)]
pub enum BrainDecision {
    Undecided,
}
