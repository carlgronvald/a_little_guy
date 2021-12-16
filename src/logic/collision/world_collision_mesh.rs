use glm::Vec2;

use super::{Triangle, Aabb};

pub struct WorldCollisionMesh {
    triangles : Vec<Triangle>
}

impl WorldCollisionMesh {
    pub fn new(triangles : Vec<Triangle>) -> Self {
        Self { triangles }
    }

    pub fn find_collision(&self, aabb : &Aabb ) -> Option<Vec2> {
        let mut intersection_vector = None;
        let mut intersection_dist = 0.0;
        for triangle in self.triangles.iter() {
            if triangle.is_colliding(aabb) {
                let test_intersection_vector = triangle.closest_intersection_vector(aabb);
                if test_intersection_vector.magnitude_squared() > intersection_dist {
                    intersection_dist = test_intersection_vector.magnitude_squared();
                    intersection_vector = Some(test_intersection_vector);
                }
            }
        }
        intersection_vector
    }
}