use glm::Vec2;
use serde::{Deserialize, Serialize};

use super::Direction;

/// Represents the event of something happening outside of state that the state might need to react to.
/// Examples are player actions and game closing.
/// Examples do _not_ include mouse clicks or button presses.
/// These events should be abstracted away before-hand.
/// Stuff like saving and loading should be handled by logic around the state.
/// These events are only for events to be sent into the game world.
#[derive(Debug, Serialize, Deserialize)]
pub enum StateInputEvent {
    /// Makes the player move in the direction given in view coordinates.
    MovePlayerRelative {
        delta: Vec2,
    },
    Jump,
    Shoot (Direction),
    Charge (Direction)
}
/*
/// Represents the entire history of input events.
#[derive(Serialize, Deserialize)]
pub struct InputEventHistory {
    input_events: Vec<Vec<StateInputEvent>>,
}

impl InputEventHistory {
    /// Creates a new history with no events stored.
    pub fn new() -> InputEventHistory {
        InputEventHistory {
            input_events: Vec::new(),
        }
    }

    /// Receive the events for the next tick.
    pub fn receive_tick_events(&mut self, events: Vec<StateInputEvent>) {
        self.input_events.push(events)
    }

    /// Gets all events stored for the specific tick.
    /// Returns None if the history hasn't reached the given tick number yet.
    ///
    /// # Arguments
    ///
    /// `tick_num` - The tick to get events for.
    pub fn get_events(&'_ self, tick_num: usize) -> Option<&'_ [StateInputEvent]> {
        self.input_events.get(tick_num).map(|vec| &vec[..])
    }

    /// Returns the events for the latest tick.
    pub fn cur_tick_events(&'_ self) -> Option<&'_ [StateInputEvent]> {
        self.input_events.last().map(|vec| &vec[..])
    }

    /// Returns the current tick number.
    pub fn cur_tick_num(&self) -> usize {
        self.input_events.len()
    }
}
*/
