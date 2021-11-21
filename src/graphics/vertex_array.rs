use wgpu::{util::DeviceExt, Buffer, RenderPass};

use super::Vertex;

///
/// Contains one vertex and index buffer for drawing.
/// Might eventually contain several models by filling disjoint slices of the buffers with those different models.
pub struct VertexArray {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vertex_count: usize,
    index_count: usize,
}

impl VertexArray {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            vertex_count: vertices.len(),
            index_count: indices.len(),
        }
    }

    pub fn draw<'a>(&'a self, render_pass: RenderPass<'a>) {
        let mut render_pass = render_pass;
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }
}
