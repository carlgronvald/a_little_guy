use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min_x : f32,
    pub min_y : f32,
    pub max_x : f32,
    pub max_y : f32
}

impl AABB {
    pub fn is_colliding(&self, aabb : &AABB) -> bool {
        self.min_x < aabb.max_x && self.min_y < aabb.max_y && self.max_x > aabb.min_x && self.max_y > aabb.min_y
    }

    pub fn surrounding_aabb(aabbs : &[AABB]) -> AABB {
        let min_x = aabbs.iter().map(|x| x.min_x).reduce(|x, y| if x < y {x} else {y}).unwrap();
        let min_y = aabbs.iter().map(|x| x.min_y).reduce(|x, y| if x < y {x} else {y}).unwrap();
        let max_x = aabbs.iter().map(|x| x.max_x).reduce(|x, y| if x > y {x} else {y}).unwrap();
        let max_y = aabbs.iter().map(|x| x.max_y).reduce(|x, y| if x > y {x} else {y}).unwrap();
        AABB {
            min_x, min_y, max_x, max_y
        }
    }
}

impl Add<[f32;2]> for AABB {
    type Output = AABB;

    fn add(self, rhs: [f32;2]) -> Self::Output {
        AABB {
            min_x : self.min_x + rhs[0],
            min_y : self.min_y + rhs[1],
            max_x : self.max_x + rhs[0],
            max_y : self.max_y + rhs[1]
        }
    }   
}

impl Mul<f32> for AABB {
    type Output = AABB;

    fn mul(self, rhs : f32) -> Self::Output {
        AABB {
            min_x : self.min_x * rhs,
            min_y : self.min_y * rhs,
            max_x : self.max_x * rhs,
            max_y : self.max_y * rhs
        }
    }
}