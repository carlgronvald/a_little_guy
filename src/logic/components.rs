use glm::Vec2;

//
// All Component structs
//

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        Self { x : vec.x, y : vec.y}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}
impl From<Vec2> for Velocity {
    fn from(vec: Vec2) -> Self {
        Self { dx : vec.x, dy : vec.y}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    pub name : String
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimedLife {
    pub seconds_left : f32,
}