use glm::Vec2;

use super::{aabb::AabbCorner, Aabb};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum TriangleCorner {
    A,
    B,
    C,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
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
            .reduce(|k, l| if k < l { k } else { l })
            .unwrap();
        let max_y = *[a.y, b.y, c.y]
            .iter()
            .reduce(|k, l| if k < l { k } else { l })
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

    pub fn surrounding_aabb(&self) -> Aabb {
        self.surrounding_aabb
    }

    pub fn approx_is_colliding(&self, aabb: &Aabb) -> bool {
        if !self.surrounding_aabb.is_colliding(aabb) {
            return false;
        }

        for side in TriangleSide::iter() {
            let vector_to_side = self.vector_to_side(side);
            let distance_to_side = self.distance_to_side(side, &vector_to_side);

            for corner in AabbCorner::iter() {
                let corner = aabb.get_corner(corner);
                let projected_distance = (corner - self.center_point).dot(&vector_to_side);
                if projected_distance > 0.0 && projected_distance < distance_to_side {
                    return true;
                }
            }
        }

        for corner in TriangleCorner::iter() {
            if aabb.is_inside(self.get_corner(corner)) {
                return true;
            }
        }

        // NB: THIS DOES NOT CATCH ALL INTERSECTIONS! IF THE TRIANGLE AND RECTANGLE ARE OVERLAPPING WITHOUT ANY CORNERS INSIDE EACH OTHER, IT'S MISSED!
        // BUT THIS REQUIRES SWIFT MOVEMENT FROM EITHER THE TRIANGLE OR THE RECTANGLE; THIS ALGORITHM IS NOT MADE TO CATCH THAT

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
