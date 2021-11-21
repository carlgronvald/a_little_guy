use wgpu::Device;

use crate::logic::Position;

use super::{vertex_array::VertexArray, Vertex};

pub struct DrawState {
    positions: Vec<Position>,
}

impl DrawState {
    pub fn new(positions: Vec<Position>) -> Self {
        Self { positions }
    }

    pub fn create_vertex_array(&self, device: &Device) -> VertexArray {
        let vertices: Vec<Vertex> = self
            .positions
            .iter()
            .flat_map(|pos| {
                [
                    Vertex {
                        position: [pos.x, pos.y, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [pos.x + 1.0, pos.y, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [pos.x + 1.0, pos.y + 1.0, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [pos.x, pos.y + 1.0, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                ]
            })
            .collect();
        let indices: Vec<u16> = self
            .positions
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
