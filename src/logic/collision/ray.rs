use glm::Vec2;

#[derive(Debug)]
pub struct Ray {
    start_point: Vec2,
    vector: Vec2,
}

impl Ray {
    pub fn new(start_point: Vec2, vector: Vec2) -> Self {
        Self {
            start_point,
            vector,
        }
    }

    pub fn are_intersecting(ray_1: &Ray, ray_2: &Ray) -> bool {
        let p = ray_1.start_point;
        let q = ray_2.start_point;
        let v = ray_1.vector;
        let u = ray_2.vector;

        let t = u.x * u.y * (q.y + p.x - p.y - q.x) / (v.y * u.x - v.x * u.y);
        let s = (p.x - q.x + t * v.x) / u.x;
        return t >= 0.0 && t <= 1.0 && s >= 0.0 && s <= 1.0;
    }
}
