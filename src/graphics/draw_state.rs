use wgpu::{Device, Queue};

use crate::logic::{Asset, Position};

use super::{
    uniforms::{DefaultUniforms, Uniform},
    vertex_array::VertexArray,
    Vertex,
};

///
/// The state sent from the logic system so the graphics knows what to draw.
/// Contains all the business logic to convert that data to rendering (for now)
///
pub struct DrawState {
    entities: Vec<(Asset, Position)>,
    camera_offset: [f32; 2],
}

impl DrawState {
    pub fn new(entities: Vec<(Asset, Position)>, camera_offset: [f32; 2]) -> Self {
        Self {
            entities,
            camera_offset,
        }
    }

    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        uniforms: &mut Uniform<DefaultUniforms>,
    ) -> VertexArray {
        uniforms.update_uniform(|x| x.camera_offset = self.camera_offset, queue);
        let vertices: Vec<Vertex> = self
            .entities
            .iter()
            .flat_map(|(asset, pos)| {
                let (min_u, min_v, max_u, max_v) = match &asset.name[..] {
                    "player" => (0.0, 0.0, 0.25, 0.25),
                    "bush" => (0.25, 0.0, 0.5, 0.25),
                    _ => (0.0,0.0,1.0,1.0),
                };

                [
                    Vertex {
                        position: [pos.x, pos.y, 0.0],
                        tex_coords: [min_u, max_v],
                    },
                    Vertex {
                        position: [pos.x + 64.0, pos.y, 0.0],
                        tex_coords: [max_u, max_v],
                    },
                    Vertex {
                        position: [pos.x + 64.0, pos.y + 64.0, 0.0],
                        tex_coords: [max_u, min_v],
                    },
                    Vertex {
                        position: [pos.x, pos.y + 64.0, 0.0],
                        tex_coords: [min_u, min_v],
                    },
                ]
            })
            .collect();
        let indices: Vec<u16> = self
            .entities
            .iter()
            .enumerate()
            .flat_map(|(i, _)| {
                [
                    (i * 4) as u16,
                    (i * 4 + 1) as u16,
                    (i * 4 + 2) as u16,
                    (i * 4) as u16,
                    (i * 4 + 2) as u16,
                    (i * 4 + 3) as u16,
                ]
            })
            .collect();
        VertexArray::new(device, &vertices, &indices)
    }
}
