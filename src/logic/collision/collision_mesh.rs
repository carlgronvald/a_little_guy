use super::Aabb;

pub struct CollisionMesh {
    pub aabb: Aabb,
}

impl CollisionMesh {
    pub fn new(aabb: Aabb) -> Self {
        Self { aabb }
    }

    pub fn is_colliding(&self, mesh: &CollisionMesh) -> bool {
        self.aabb.is_colliding(&mesh.aabb)
    }

    pub fn transform(&self, translation: [f32; 2], multiplication: f32) -> CollisionMesh {
        let aabb = self.aabb * multiplication + translation;
        CollisionMesh { aabb }
    }

    pub fn closeest_intersection_vector(&self, mesh: &CollisionMesh) -> glm::Vec2 {
        self.aabb.closest_intersection_vector(&mesh.aabb)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CollisionMeshIdentifier {
    index: usize,
}

pub struct CollisionMeshManager {
    meshes: Vec<CollisionMesh>,
}

impl CollisionMeshManager {
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    pub fn add_collision_mesh(&mut self, mesh: CollisionMesh) -> CollisionMeshIdentifier {
        let identifier = CollisionMeshIdentifier {
            index: self.meshes.len(),
        };
        self.meshes.push(mesh);

        identifier
    }

    pub fn get_collision_mesh(&self, identifier: CollisionMeshIdentifier) -> &CollisionMesh {
        &self.meshes[identifier.index]
    }
}
