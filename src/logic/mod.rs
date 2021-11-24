mod components;
pub use components::{Asset, Player, Position, Velocity};

mod resources;
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
enum Direction {
    Up, Right, Down, Left
}