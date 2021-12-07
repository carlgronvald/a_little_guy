use std::collections::HashMap;

use wgpu::Device;
use winit::dpi::PhysicalSize;

use crate::logic::{Asset, Position};

use super::{
    model::Model, texture::TextureIdentifier, uniforms::DefaultUniforms, vertex_array::VertexArray,
    Vertex,
};

pub struct DrawPackage {
    pub vertex_array: VertexArray,
    pub uniforms: DefaultUniforms,
    pub texture: TextureIdentifier,
}

///
/// The state sent from the logic system so the graphics knows what to draw.
/// Contains all the business logic to convert that data to rendering (for now)
///
pub struct DrawState {
    entities: Vec<(Asset, Position)>,
    camera_offset: [f32; 2],
    time: f32,
}

impl DrawState {
    pub fn new(entities: Vec<(Asset, Position)>, camera_offset: [f32; 2], time: f32) -> Self {
        let mut entities = entities;
        entities.sort_by(|x, y| y.1.y.partial_cmp(&x.1.y).unwrap());
        Self {
            entities,
            camera_offset,
            time,
        }
    }

    pub fn render(
        &self,
        device: &Device,
        uniforms: &DefaultUniforms,
        models: &HashMap<String, Model>,
        screen_size: PhysicalSize<u32>,
    ) -> Vec<DrawPackage> {
        let background_vertex_array = {
            let vertices: Vec<Vertex> = self
                .entities
                .iter()
                .filter(|(asset, _)| models[&asset.name].texture() == "background")
                .flat_map(|(asset, pos)| {
                    let model = &models[&asset.name];

                    model.vertices(
                        *pos,
                        asset.animation,
                        self.time - asset.animation_start_time,
                    )
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
        };

        let atlas_vertex_array = {
            let vertices: Vec<Vertex> = self
                .entities
                .iter()
                .filter(|(asset, _)| models[&asset.name].texture() == "atlas")
                .flat_map(|(asset, pos)| {
                    let model = &models[&asset.name];

                    model.vertices(
                        *pos,
                        asset.animation,
                        self.time - asset.animation_start_time,
                    )
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
        };

        if self
            .entities
            .iter()
            .filter(|(asset, _)| {
                models[&asset.name].texture() != "atlas"
                    && models[&asset.name].texture() != "background"
            })
            .count()
            > 0
        {
            panic!("Trying to draw from a non-existant texture! Right now only the background and atlas texture are implemented.")
        }

        vec![
            DrawPackage {
                vertex_array: background_vertex_array,
                uniforms: DefaultUniforms {
                    camera_offset: [
                        self.camera_offset[0] + (screen_size.width as f32) / 2.0,
                        self.camera_offset[1] + (screen_size.height as f32) / 2.0,
                    ],
                    ..*uniforms
                },
                texture: TextureIdentifier::new("background".into()),
            },
            DrawPackage {
                vertex_array: atlas_vertex_array,
                uniforms: DefaultUniforms {
                    camera_offset: [
                        self.camera_offset[0] + (screen_size.width as f32) / 2.0,
                        self.camera_offset[1] + (screen_size.height as f32) / 2.0,
                    ],
                    ..*uniforms
                },
                texture: TextureIdentifier::new("atlas".into()),
            },
        ]
    }
}
