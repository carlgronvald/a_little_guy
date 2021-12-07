use glm::Vec2;

use super::AABB;

pub struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2,
    surrounding_aabb: AABB,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
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
            surrounding_aabb: AABB {
                min_x,
                min_y,
                max_x,
                max_y,
            },
        }
    }

    pub fn surrounding_aabb(&self) -> AABB {
        self.surrounding_aabb
    }

    pub fn is_colliding(&self, aabb: &AABB) -> bool {
        if !self.surrounding_aabb.is_colliding(aabb) {
            return false;
        }

        //TODO: TRIANGLE INTERSECTION PART

        true
    }
}
