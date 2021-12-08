use std::ops::{Add, Mul};

use strum_macros::EnumIter;

use crate::logic::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum AabbCorner {
    Sw,
    Se,
    Ne,
    Nw,
}

impl Aabb {
    pub fn is_colliding(&self, aabb: &Aabb) -> bool {
        self.min_x < aabb.max_x
            && self.min_y < aabb.max_y
            && self.max_x > aabb.min_x
            && self.max_y > aabb.min_y
    }

    pub fn get_corner(&self, corner: AabbCorner) -> glm::Vec2 {
        match corner {
            AabbCorner::Sw => glm::vec2(self.min_x, self.min_y),
            AabbCorner::Se => glm::vec2(self.max_x, self.min_y),
            AabbCorner::Nw => glm::vec2(self.min_x, self.max_y),
            AabbCorner::Ne => glm::vec2(self.max_x, self.max_y),
        }
    }

    pub fn is_inside(&self, point: glm::Vec2) -> bool {
        point.x > self.min_x && point.y > self.min_y && point.x < self.max_x && point.y < self.max_y
    }

    pub fn surrounding_aabb(aabbs: &[Aabb]) -> Aabb {
        let min_x = aabbs
            .iter()
            .map(|x| x.min_x)
            .reduce(|x, y| if x < y { x } else { y })
            .unwrap();
        let min_y = aabbs
            .iter()
            .map(|x| x.min_y)
            .reduce(|x, y| if x < y { x } else { y })
            .unwrap();
        let max_x = aabbs
            .iter()
            .map(|x| x.max_x)
            .reduce(|x, y| if x > y { x } else { y })
            .unwrap();
        let max_y = aabbs
            .iter()
            .map(|x| x.max_y)
            .reduce(|x, y| if x > y { x } else { y })
            .unwrap();
        Aabb {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn closest_intersection_vector(&self, aabb: &Aabb) -> glm::Vec2 {
        let x_1 = aabb.max_x - self.min_x;
        let x_2 = aabb.min_x - self.max_x;
        let y_1 = aabb.max_y - self.min_y;
        let y_2 = aabb.min_y - self.max_y;
        let &min = [x_1, x_2, y_1, y_2]
            .iter()
            .reduce(|x, y| if x.abs() < y.abs() { x } else { y })
            .unwrap();

        let direction = if min == x_1 {
            Direction::Right
        } else if min == x_2 {
            Direction::Left
        } else if min == y_1 {
            Direction::Up
        } else if min == y_2 {
            Direction::Down
        } else {
            unreachable!()
        };

        glm::Vec2::from(direction) * (min.abs())
    }
}

impl Add<[f32; 2]> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: [f32; 2]) -> Self::Output {
        Aabb {
            min_x: self.min_x + rhs[0],
            min_y: self.min_y + rhs[1],
            max_x: self.max_x + rhs[0],
            max_y: self.max_y + rhs[1],
        }
    }
}

impl Mul<f32> for Aabb {
    type Output = Aabb;

    fn mul(self, rhs: f32) -> Self::Output {
        Aabb {
            min_x: self.min_x * rhs,
            min_y: self.min_y * rhs,
            max_x: self.max_x * rhs,
            max_y: self.max_y * rhs,
        }
    }
}
