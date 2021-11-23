use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device, Queue};

pub struct Uniform<T: Pod + Zeroable> {
    buffer: Buffer,
    uniform_struct: T,
}

impl<T: Pod + Zeroable> Uniform<T> {
    pub fn new(device: &Device, uniform_struct: T) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform_struct]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        Self {
            buffer,
            uniform_struct,
        }
    }

    pub fn create_bind_group(&self, device: &Device) -> (BindGroupLayout, BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.buffer.as_entire_binding(),
            }],
            label: None,
        });

        (bind_group_layout, bind_group)
    }

    pub fn update_uniform(&mut self, change: impl FnOnce(&mut T), queue: &Queue) {
        change(&mut self.uniform_struct);
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.uniform_struct]),
        )
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefaultUniforms {
    pub x_scale: f32,
    pub y_scale: f32,
    dummy: [f32; 2],
}

impl DefaultUniforms {
    pub fn new(x_scale: f32, y_scale: f32) -> Self {
        Self {
            x_scale,
            y_scale,
            dummy: [0.0, 0.0],
        }
    }
}

unsafe impl bytemuck::Pod for DefaultUniforms {}
unsafe impl bytemuck::Zeroable for DefaultUniforms {}
