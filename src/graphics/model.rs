use crate::logic::Position;

use super::Vertex;

//TODO
pub struct Model {
    texture: String,
    animations: Vec<Animation>,
    ///
    /// # of indices across each axis in the model atlas.
    ///
    indices_on_axis: usize,
    /// Size of the model when drawn in pixels
    size: f32,
}

pub struct Animation {
    indices: Vec<usize>,
    time_per_frame: f32,
    one_off: bool,
}

impl Model {
    ///
    /// Creates a new model with the given animations.
    ///
    /// The first animation passed becomes the idle animation.
    ///
    pub fn new(
        texture: String,
        indices_on_axis: usize,
        size: f32,
        animations: Vec<Animation>,
    ) -> Self {
        Self {
            texture,
            indices_on_axis,
            animations,
            size,
        }
    }

    pub fn texture(&self) -> &str {
        &self.texture
    }

    pub fn animation(&self, index: usize) -> &Animation {
        &self.animations[index]
    }

    pub fn indices_on_axis(&self) -> usize {
        self.indices_on_axis
    }

    fn uv_indices(
        &self,
        animation_index: usize,
        animation_time_elapsed: f32,
    ) -> (f32, f32, f32, f32) {
        let animation = self.animation(animation_index);
        let index = animation.index((animation_time_elapsed / animation.time_per_frame()) as usize);
        let uv_mod = 1.0 / (self.indices_on_axis() as f32);

        let u_index = (index % self.indices_on_axis()) as f32;
        let v_index = (index / self.indices_on_axis()) as f32;

        (
            uv_mod * u_index,
            uv_mod * v_index,
            uv_mod * (u_index + 1.0),
            uv_mod * (v_index + 1.0),
        )
    }

    pub fn vertices(
        &self,
        center_position: Position,
        animation_index: usize,
        animation_time_elapsed: f32,
    ) -> [Vertex; 4] {
        let (min_u, min_v, max_u, max_v) = self.uv_indices(animation_index, animation_time_elapsed);
        [
            Vertex {
                position: [
                    center_position.x - self.size / 2.0,
                    center_position.y - self.size / 2.0,
                    0.0,
                ],
                tex_coords: [min_u, max_v],
            },
            Vertex {
                position: [
                    center_position.x + self.size / 2.0,
                    center_position.y - self.size / 2.0,
                    0.0,
                ],
                tex_coords: [max_u, max_v],
            },
            Vertex {
                position: [
                    center_position.x + self.size / 2.0,
                    center_position.y + self.size / 2.0,
                    0.0,
                ],
                tex_coords: [max_u, min_v],
            },
            Vertex {
                position: [
                    center_position.x - self.size / 2.0,
                    center_position.y + self.size / 2.0,
                    0.0,
                ],
                tex_coords: [min_u, min_v],
            },
        ]
    }
}

impl Animation {
    pub fn new(indices: Vec<usize>, time_per_frame: f32, one_off: bool) -> Self {
        Self {
            indices,
            time_per_frame,
            one_off,
        }
    }

    pub fn steps(&self) -> usize {
        self.indices.len()
    }

    pub fn index(&self, index: usize) -> usize {
        self.indices[if self.one_off {
            std::cmp::min(index, self.steps() - 1)
        } else {
            index % self.steps()
        }]
    }

    pub fn time_per_frame(&self) -> f32 {
        self.time_per_frame
    }
}
