use serde::{Deserialize, Serialize};
use winit::event::{MouseButton, VirtualKeyCode};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Control {
    Mouse { mouse_button: MouseButton },
    Keyboard { key_code: VirtualKeyCode },
}

#[derive(Serialize, Deserialize)]
pub struct ControlConfig {
    #[serde(default = "move_forward_default")]
    pub move_forward: Control,
    #[serde(default = "move_back_default")]
    pub move_back: Control,
    #[serde(default = "strafe_right_default")]
    pub strafe_right: Control,
    #[serde(default = "strafe_left_default")]
    pub strafe_left: Control,
    #[serde(default = "jump_default")]
    pub jump: Control,
    #[serde(default = "save_default")]
    pub save: Control,
    #[serde(default = "load_default")]
    pub load: Control,
    #[serde(default = "player_interact_1_default")]
    pub player_interact_1: Control,
    #[serde(default = "player_interact_2_default")]
    pub player_interact_2: Control,
    #[serde(default = "shoot_right_default")]
    pub shoot_right: Control,
    #[serde(default = "shoot_up_default")]
    pub shoot_up: Control,
    #[serde(default = "shoot_left_default")]
    pub shoot_left: Control,
    #[serde(default = "shoot_down_default")]
    pub shoot_down: Control,
}

impl Default for ControlConfig {
    fn default() -> Self {
        ControlConfig {
            move_forward: move_forward_default(),
            move_back: move_back_default(),
            strafe_right: strafe_right_default(),
            strafe_left: strafe_left_default(),
            jump: jump_default(),
            save: save_default(),
            load: load_default(),
            player_interact_1: player_interact_1_default(),
            player_interact_2: player_interact_2_default(),
            shoot_right : shoot_right_default(),
            shoot_up : shoot_up_default(),
            shoot_left : shoot_left_default(),
            shoot_down : shoot_down_default()
        }
    }
}

fn move_forward_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::W,
    }
}

fn move_back_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::S,
    }
}

fn strafe_right_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::D,
    }
}

fn strafe_left_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::A,
    }
}

fn jump_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::Space,
    }
}

fn save_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::S,
    }
}
fn load_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::L,
    }
}
fn player_interact_1_default() -> Control {
    Control::Mouse {
        mouse_button: MouseButton::Left,
    }
}
fn player_interact_2_default() -> Control {
    Control::Mouse {
        mouse_button: MouseButton::Right,
    }
}
fn shoot_right_default() -> Control {
    Control::Keyboard {
        key_code : VirtualKeyCode::Right
    }
}
fn shoot_up_default() -> Control {
    Control::Keyboard {
        key_code : VirtualKeyCode::Up
    }
}
fn shoot_left_default() -> Control {
    Control::Keyboard {
        key_code : VirtualKeyCode::Left
    }
}
fn shoot_down_default() -> Control {
    Control::Keyboard {
        key_code : VirtualKeyCode::Down
    }
}
