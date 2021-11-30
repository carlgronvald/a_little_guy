use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPass, RenderPipeline};

use super::{
    texture::{Texture, TextureIdentifier},
    uniforms::Uniform,
    Vertex,
};

pub struct Pipeline {
    render_pipeline: RenderPipeline,
    texture_bind_groups: HashMap<TextureIdentifier, BindGroup>,
    uniform_bind_group: Option<BindGroup>,
    bind_group_layouts: Vec<BindGroupLayout>,
}

impl Pipeline {
    ///
    /// Creates a basic pipeline that uses the default shader & rendering system.
    ///
    pub fn new(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: Vec<BindGroupLayout>,
    ) -> Self {
        // Bind group layout references
        let bind_group_l_refs: Vec<&BindGroupLayout> = bind_group_layouts.iter().collect();

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &bind_group_l_refs,
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Self {
            render_pipeline,
            texture_bind_groups: HashMap::new(),
            uniform_bind_group: None,
            bind_group_layouts,
        }
    }

    ///
    /// Binds all bind groups
    ///
    pub fn bind_uniforms<'a: 'b, 'b>(
        &'a self,
        render_pass: &mut RenderPass<'b>,
        texture: &TextureIdentifier,
    ) {
        render_pass.set_bind_group(0, &self.texture_bind_groups[texture], &[]);
        render_pass.set_bind_group(1, self.uniform_bind_group.as_ref().unwrap(), &[]);
    }

    pub fn set<'a: 'b, 'b>(&'a self, render_pass: &mut RenderPass<'b>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }

    pub fn create_texture_bind_group(
        &mut self,
        device: &Device,
        texture: &Texture,
        identifier: &TextureIdentifier,
    ) {
        self.texture_bind_groups.insert(
            identifier.clone(),
            texture.create_bind_group(device, &self.bind_group_layouts[0]),
        );
    }

    pub fn set_uniform_bind_group<T: Pod + Zeroable>(
        &mut self,
        device: &Device,
        uniform: &Uniform<T>,
    ) {
        self.uniform_bind_group =
            Some(uniform.create_bind_group(device, &self.bind_group_layouts[1]));
    }
}
