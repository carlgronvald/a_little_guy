use glm::Vec2;

use super::collision::{CollisionMeshIdentifier, CollisionMeshManager};

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
        Self { x: vec.x, y: vec.y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl From<Vec2> for Velocity {
    fn from(vec: Vec2) -> Self {
        Self {
            dx: vec.x,
            dy: vec.y,
        }
    }
}

impl From<Velocity> for Vec2 {
    fn from(vel: Velocity) -> Self {
        glm::vec2(vel.dx, vel.dy)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    pub name: String,
    pub animation: usize,
    pub animation_start_time: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimedLife {
    pub seconds_left: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Friction {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collider {
    pub collision_mesh: CollisionMeshIdentifier,
    pub size: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Team {
    PLAYER,
    ENEMY,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub collides_with_own_team: bool,
    pub team: Team,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            collides_with_own_team: false,
            team: Team::ENEMY,
        }
    }
}

impl Collider {
    pub fn is_colliding(
        pos1: &Position,
        col1: &Collider,
        pos2: &Position,
        col2: &Collider,
        collision_meshes: &CollisionMeshManager,
    ) -> bool {
        let sz1 = col1.size;
        let sz2 = col2.size;

        let col1 = collision_meshes.get_collision_mesh(col1.collision_mesh);
        let col2 = collision_meshes.get_collision_mesh(col2.collision_mesh);
        col1.transform([pos1.x, pos1.y], sz1)
            .is_colliding(&col2.transform([pos2.x, pos2.y], sz2))
    }

    pub fn closest_intersection_vector(
        pos1: &Position,
        col1: &Collider,
        pos2: &Position,
        col2: &Collider,
        collision_meshes: &CollisionMeshManager,
    ) -> glm::Vec2 {
        let sz1 = col1.size;
        let sz2 = col2.size;

        let col1 = collision_meshes.get_collision_mesh(col1.collision_mesh);
        let col2 = collision_meshes.get_collision_mesh(col2.collision_mesh);
        col1.transform([pos1.x, pos1.y], sz1)
            .closeest_intersection_vector(&col2.transform([pos2.x, pos2.y], sz2))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AiRandomWalk {
    pub speed: f32,
}
