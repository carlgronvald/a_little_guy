//
// All Component structs
//

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

///
/// Marker struct for the player entity
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {}
