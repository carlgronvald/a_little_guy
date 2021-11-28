use super::controls::{Control, ControlConfig};
use super::state_input_event::StateInputEvent;
use super::Direction;
use crate::graphics::ExternalEvent;
use glm::Vec2;
use std::{collections::HashMap, sync::mpsc};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

/// Handles external events and produces state input events.
pub struct ExternalEventHandler {
    /// The state of each keyboard key.
    key_state: HashMap<VirtualKeyCode, bool>,
    /// The state of each mouse button.
    button_state: HashMap<MouseButton, bool>,
    /// The state events generated this tick.
    tick_state_events: Vec<StateInputEvent>,
    /// Configuration for controls.
    control_config: ControlConfig,
}

impl ExternalEventHandler {
    pub fn new(control_config: ControlConfig) -> ExternalEventHandler {
        ExternalEventHandler {
            key_state: HashMap::new(),
            button_state: HashMap::new(),
            tick_state_events: Vec::new(),
            control_config,
        }
    }

    fn key_state(&self, key_code: VirtualKeyCode) -> bool {
        *self.key_state.get(&key_code).unwrap_or(&false)
    }

    fn button_state(&self, mouse_button: MouseButton) -> bool {
        *self.button_state.get(&mouse_button).unwrap_or(&false)
    }

    fn control_state(&self, control: Control) -> bool {
        match control {
            Control::Mouse { mouse_button } => self.button_state(mouse_button),
            Control::Keyboard { key_code } => self.key_state(key_code),
        }
    }

    /// Empties the channel of new events and handles them.
    pub fn handle_inputs(
        &mut self,
        input_event_receiver: &mpsc::Receiver<crate::graphics::ExternalEvent>,
    ) {
        loop {
            match input_event_receiver.try_recv() {
                Ok(event) => self.handle_event(event),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => panic!("Event channel disconnected!"),
            }
        }
    }

    /// Handles the ExternalEvent by turning it into the right StateInputEvents.
    fn handle_event(&mut self, event: ExternalEvent) {
        match event {
            ExternalEvent::MouseMotion { delta: _ } => {}
            ExternalEvent::KeyboardInput { key_code, state } => {
                if !self.key_state(key_code) {
                    let control = Control::Keyboard { key_code };
                    self.handle_control_press(control);
                    // Handling of key presses should happen here, as the if avoids repeated presses from holding down the button.
                } else if state == ElementState::Released {
                    let control = Control::Keyboard { key_code };
                    self.handle_control_release(control);
                }
                self.key_state
                    .insert(key_code, state == ElementState::Pressed);
            }
            ExternalEvent::MouseInput { button, state } => {
                self.button_state
                    .insert(button, state == ElementState::Pressed);
                if state == ElementState::Pressed {
                    let control = Control::Mouse {
                        mouse_button: button,
                    };
                    self.handle_control_press(control);
                } else if state == ElementState::Released {
                    let control = Control::Mouse {
                        mouse_button: button,
                    };
                    self.handle_control_release(control);
                };
            }
        }
    }

    fn handle_control_press(&mut self, control: Control) {
        if control == self.control_config.jump {
            self.tick_state_events.push(StateInputEvent::Jump)
        }
    }

    fn handle_control_release(&mut self, control: Control) {
        if control == self.control_config.shoot_right {
            self.tick_state_events
                .push(StateInputEvent::Shoot(Direction::Right));
        }
        if control == self.control_config.shoot_up {
            self.tick_state_events
                .push(StateInputEvent::Shoot(Direction::Up));
        }
        if control == self.control_config.shoot_left {
            self.tick_state_events
                .push(StateInputEvent::Shoot(Direction::Left));
        }
        if control == self.control_config.shoot_down {
            self.tick_state_events
                .push(StateInputEvent::Shoot(Direction::Down));
        }
    }

    /// Returns and clears the current event buffer.
    pub fn tick_events(&mut self) -> Vec<StateInputEvent> {
        let mut state_result = std::mem::replace(&mut self.tick_state_events, Vec::new());
        let mut move_vector = Vec2::new(0., 0.);
        if self.control_state(self.control_config.move_forward) {
            move_vector += Vec2::new(0., 1.);
        }
        if self.control_state(self.control_config.strafe_right) {
            move_vector += Vec2::new(1., 0.);
        }
        if self.control_state(self.control_config.move_back) {
            move_vector += Vec2::new(0., -1.);
        }
        if self.control_state(self.control_config.strafe_left) {
            move_vector += Vec2::new(-1., 0.);
        }
        if move_vector != Vec2::new(0., 0.) {
            state_result.push(StateInputEvent::MovePlayerRelative { delta: move_vector });
        }

        let mut shoot_direction = None;
        if self.control_state(self.control_config.shoot_right) {
            shoot_direction = Some(Direction::Right);
        }
        if self.control_state(self.control_config.shoot_up) {
            shoot_direction = Some(Direction::Up);
        }
        if self.control_state(self.control_config.shoot_left) {
            shoot_direction = Some(Direction::Left);
        }
        if self.control_state(self.control_config.shoot_down) {
            shoot_direction = Some(Direction::Down);
        }
        if let Some(shoot_direction) = shoot_direction {
            state_result.push(StateInputEvent::Charge(shoot_direction));
        }
        state_result
    }
}
