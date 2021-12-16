use glm::Vec2;

use super::{aabb::AabbCorner, Aabb};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2,
    surrounding_aabb: Aabb,
    center_point: Vec2,
}

/// Sides of a triangle
/// A to B, B to C, and C to A
#[derive(Debug, EnumIter, Clone, Copy)]
pub enum TriangleSide {
    Ab,
    Bc,
    Ca,
}

impl TriangleSide {
    pub fn get_start_corner(self) -> TriangleCorner {
        match self {
            Self::Ab => TriangleCorner::A,
            Self::Bc => TriangleCorner::B,
            Self::Ca => TriangleCorner::C,
        }
    }

    pub fn get_end_corner(self) -> TriangleCorner {
        match self {
            Self::Ab => TriangleCorner::B,
            Self::Bc => TriangleCorner::C,
            Self::Ca => TriangleCorner::A,
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum TriangleCorner {
    A,
    B,
    C,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        let mut b = b;
        let mut c = c;
        if (a.x * b.y - b.x * a.y) + (b.x * c.y - c.x * b.y) + (c.x * a.y - c.y * a.x) <= 0.0 {
            println!("Triangles must be counter clockwise!");
            std::mem::swap(&mut b, &mut c);
        }
        // TODO: MAKE SURE TRIANGLES ARE ALWAYS GIVEN COUNTERCLOCKWISE
        let min_x = *[a.x, b.x, c.x]
            .iter()
            .reduce(|k, l| if k < l { k } else { l })
            .unwrap();
        let min_y = *[a.y, b.y, c.y]
            .iter()
            .reduce(|k, l| if k < l { k } else { l })
            .unwrap();
        let max_x = *[a.x, b.x, c.x]
            .iter()
            .reduce(|k, l| if k > l { k } else { l })
            .unwrap();
        let max_y = *[a.y, b.y, c.y]
            .iter()
            .reduce(|k, l| if k > l { k } else { l })
            .unwrap();
        Self {
            a,
            b,
            c,
            surrounding_aabb: Aabb {
                min_x,
                min_y,
                max_x,
                max_y,
            },
            center_point: (a + b + c) / 3.0,
        }
    }

    pub fn closest_intersection_vector(&self, aabb: &Aabb) -> Vec2 {
        let mut min_distance = f32::INFINITY;
        let mut closest_vector = None;
        for side in TriangleSide::iter() {
            let vector_to_side = self.vector_to_side(side);
            let distance_to_side = self.distance_to_side(side, &vector_to_side);

            for corner in AabbCorner::iter() {
                let corner = aabb.get_corner(corner);
                if self.is_point_inside(&corner) {
                    let projected_distance =
                        distance_to_side - (corner - self.center_point).dot(&vector_to_side);
                    if projected_distance >= -1.0 && projected_distance < min_distance {
                        min_distance = projected_distance;
                        if projected_distance != 0.0 {
                            let projected_distance = if projected_distance > 0.0 {
                                projected_distance
                            } else {
                                0.0
                            };
                            closest_vector = Some(vector_to_side * projected_distance);
                        } else {
                            closest_vector = Some(glm::vec2(0.0, 0.0));
                        }
                    }
                }
            }
        }
        if let Some(closest_vector) = closest_vector {
            return closest_vector;
        }

        aabb.closest_intersection_vector(&self.surrounding_aabb)
    }

    pub fn is_point_inside(&self, point: &Vec2) -> bool {
        for side in TriangleSide::iter() {
            let start_point = self.get_corner(side.get_start_corner());
            let end_point = self.get_corner(side.get_end_corner());

            if (end_point.y - start_point.y) * (point.x - start_point.x)
                + (start_point.x - end_point.x) * (point.y - start_point.y)
                >= 1.0
            {
                return false;
            }
        }
        true
    }

    pub fn is_colliding(&self, aabb: &Aabb) -> bool {
        if !self.surrounding_aabb.is_colliding(aabb) {
            return false;
        }

        for side in TriangleSide::iter() {
            let start_point = self.get_corner(side.get_start_corner());
            let end_point = self.get_corner(side.get_end_corner());
            let direction_vector = end_point - start_point;

            {
                // Left side of rectangle
                let t = (aabb.min_x - start_point.x) / (direction_vector.x);
                if t >= 0.0 && t <= 1.0 {
                    let collision_y = start_point.y + t * direction_vector.y;
                    if collision_y >= aabb.min_y && collision_y <= aabb.max_y {
                        return true;
                    }
                }
            }
            {
                // Right side of rectangle
                let t = (aabb.max_x - start_point.x) / (direction_vector.x);
                if t >= 0.0 && t <= 1.0 {
                    let collision_y = start_point.y + t * direction_vector.y;
                    if collision_y >= aabb.min_y && collision_y <= aabb.max_y {
                        return true;
                    }
                }
            }
            {
                // Bottom side of rectangle
                let t = (aabb.min_y - start_point.y) / (direction_vector.y);
                if t >= 0.0 && t <= 1.0 {
                    let collision_x = start_point.x + t * direction_vector.x;
                    if collision_x >= aabb.min_x && collision_x <= aabb.max_x {
                        return true;
                    }
                }
            }
            {
                // Top side of rectangle
                let t = (aabb.max_y - start_point.y) / (direction_vector.y);
                if t >= 0.0 && t <= 1.0 {
                    let collision_x = start_point.x + t * direction_vector.x;
                    if collision_x >= aabb.min_x && collision_x <= aabb.max_x {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_corner(&self, corner: TriangleCorner) -> Vec2 {
        match corner {
            TriangleCorner::A => self.a,
            TriangleCorner::B => self.b,
            TriangleCorner::C => self.c,
        }
    }

    fn distance_to_side(&self, side: TriangleSide, vector_to_side: &Vec2) -> f32 {
        match side {
            TriangleSide::Ab => (self.a - self.center_point).dot(vector_to_side).abs(),
            TriangleSide::Bc => (self.b - self.center_point).dot(vector_to_side).abs(),
            TriangleSide::Ca => (self.c - self.center_point).dot(vector_to_side).abs(),
        }
    }

    fn vector_to_side(&self, side: TriangleSide) -> Vec2 {
        let side_vector = match side {
            TriangleSide::Ab => self.b - self.a,
            TriangleSide::Bc => self.c - self.b,
            TriangleSide::Ca => self.a - self.c,
        };

        glm::vec2(side_vector.y, -side_vector.x).normalize()
    }
}
