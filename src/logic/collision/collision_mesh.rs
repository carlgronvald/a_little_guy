use super::AABB;

pub struct CollisionMesh {
    pub aabbs : Vec<AABB>,
    pub surrounding_aabb : AABB
}

impl CollisionMesh {
    pub fn new(aabbs : Vec<AABB>) -> Self {
        Self { surrounding_aabb : AABB::surrounding_aabb(&aabbs), aabbs }
    }

    pub fn is_colliding(&self, mesh : &CollisionMesh) -> bool {
        if !self.surrounding_aabb.is_colliding(&mesh.surrounding_aabb) {
            return false;
        }
        for aabb in &mesh.aabbs {
            for aabb2 in &self.aabbs {
                if aabb.is_colliding(aabb2) {
                    return true
                }
            }
        }

        false
    }

    pub fn transform(&self, translation : [f32;2], multiplication : f32) -> CollisionMesh{
        let aabbs : Vec<AABB> = self.aabbs.iter().map(|x| (*x*multiplication) + translation).collect();
        let surrounding_aabb = self.surrounding_aabb*multiplication + translation;
        CollisionMesh {
            aabbs,
            surrounding_aabb
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CollisionMeshIdentifier {
    index : usize
}

pub struct CollisionMeshManager {
    meshes : Vec<CollisionMesh>,
}

impl CollisionMeshManager {
    pub fn new() -> Self {
        Self {
            meshes : Vec::new(),
        }
    }

    pub fn add_collision_mesh(&mut self, mesh : CollisionMesh) -> CollisionMeshIdentifier {
        let identifier = CollisionMeshIdentifier { index : self.meshes.len()};
        self.meshes.push(mesh);

        identifier
    }

    pub fn get_collision_mesh(&self, identifier : CollisionMeshIdentifier) -> &CollisionMesh {
        &self.meshes[identifier.index]
    }

}

