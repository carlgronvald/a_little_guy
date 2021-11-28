mod components;
pub use components::*;

mod resources;
use glm::Vec2;
pub use resources::Time;

mod systems;
pub use systems::{update_positions_system, update_velocities_system};

mod controls;
mod external_event_handler;
mod state_input_event;

mod logic;
pub use logic::start_logic_thread;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => glm::vec2(0.0, 1.0),
            Direction::Right => glm::vec2(1.0, 0.0),
            Direction::Down => glm::vec2(0.0, -1.0),
            Direction::Left => glm::vec2(-1.0, 0.0),
        }
    }
}

impl Direction {
    pub fn lowercase(self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Right => "right",
            Direction::Left => "left",
            Direction::Down => "down",
        }
    }
}
