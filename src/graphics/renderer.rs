use std::collections::HashMap;

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use crate::graphics::model::Animation;

use super::{
    model::Model,
    pipeline::Pipeline,
    texture::{Texture, TextureIdentifier},
    uniforms::{DefaultUniforms, Uniform},
    DrawState,
};

///
/// Contains everything needed to interact with the WGPU rendering system
///
pub struct Renderer {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,

    pipeline: Pipeline,
    default_uniforms: Uniform<DefaultUniforms>,

    models: HashMap<String, Model>,
    textures: HashMap<TextureIdentifier, Texture>,
}

impl Renderer {
    ///
    /// Initializes the WGPU rendering system
    ///
    pub async fn new(window: &winit::window::Window) -> Self {
        println!("Starting render creation.");
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        println!("{},{}", size.width, size.height);
        println!("{:?}", &device);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let textures = Self::load_textures(&device, &queue);

        let default_uniforms = Uniform::new(
            &device,
            DefaultUniforms::new(
                1.0 / (size.width as f32),
                1.0 / (size.height as f32),
                [0.0, 0.0],
            ),
        );

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        let texture_bind_group_layout = Texture::create_bind_group_layout(&device);
        let uniform_bind_group_layout =
            Uniform::<DefaultUniforms>::create_bind_group_layout(&device);
        // Create our main pipeline
        let mut pipeline = Pipeline::new(
            &device,
            &shader,
            &config,
            vec![texture_bind_group_layout, uniform_bind_group_layout],
        );
        for (name, texture) in textures.iter() {
            pipeline.create_texture_bind_group(&device, texture, name);
        }
        pipeline.set_uniform_bind_group(&device, &default_uniforms);

        let models = Self::load_models();

        println!("Returning renderer");
        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,

            default_uniforms,

            textures,
            models,
        }
    }

    fn load_models() -> HashMap<String, Model> {
        let mut models = HashMap::new();

        let player_model = Model::new(
            "atlas".into(),
            8,
            192.0,
            vec![Animation::new(vec![0], 1.0, false)],
        );
        models.insert("player".into(), player_model);

        let bush_model = Model::new(
            "atlas".into(),
            16,
            96.0,
            vec![Animation::new(vec![2], 1.0, false)],
        );
        models.insert("bush".into(), bush_model);

        let lamppost_model = Model::new(
            "atlas".into(),
            8,
            192.0,
            vec![Animation::new(vec![2], 1.0, false)],
        );
        models.insert("lamp post".into(), lamppost_model);

        let arrow_model = Model::new(
            "atlas".into(),
            8,
            192.0,
            vec![Animation::new(
                vec![8, 9, 10, 11, 12, 13, 14, 15, 63],
                0.125,
                true,
            )],
        );
        models.insert("arrow".into(), arrow_model);

        let background_model = Model::new(
            "background".into(),
            1,
            4800.0,
            vec![Animation::new(vec![0], 1.0, false)],
        );
        models.insert("background".into(), background_model);

        let firefly_model = Model::new(
            "atlas".into(),
            32,
            48.0,
            vec![Animation::new(
                vec![6, 6, 6, 6, 6, 6, 6, 6, 7, 38, 39, 39, 38, 7],
                0.2,
                false,
            )],
        );
        models.insert("firefly".into(), firefly_model);

        models
    }

    fn load_textures(device: &Device, queue: &Queue) -> HashMap<TextureIdentifier, Texture> {
        let atlas_bytes = include_bytes!("atlas.png");
        let atlas_texture = Texture::new(&device, &queue, atlas_bytes);

        let background_bytes = include_bytes!("background.png");
        let background_texture = Texture::new(&device, &queue, background_bytes);

        let mut textures = HashMap::new();
        textures.insert(TextureIdentifier::new("atlas".into()), atlas_texture);
        textures.insert(
            TextureIdentifier::new("background".into()),
            background_texture,
        );

        textures
    }

    ///
    /// Resizes the WGPU surface - needs to be called whenever the window changes size
    ///
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.default_uniforms.update_uniform(
                |x| {
                    x.x_scale = 1.0 / (new_size.width as f32);
                    x.y_scale = 1.0 / (new_size.height as f32)
                },
                &self.queue,
            )
        }
    }

    ///
    /// Renders the given DrawState using the default pipeline.
    ///
    pub fn render(&mut self, draw_state: DrawState) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut draw_packages = draw_state.render(
            &self.device,
            self.default_uniforms.uniform_struct(),
            &self.models,
            self.size,
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 232.0 / 255.0,
                        g: 220.0 / 255.0,
                        b: 184.0 / 255.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.pipeline.set(&mut render_pass);

        for draw_package in draw_packages.iter_mut() {
            self.default_uniforms
                .update_uniform(|x| *x = draw_package.uniforms, &self.queue);
            self.pipeline
                .bind_uniforms(&mut render_pass, &draw_package.texture);

            draw_package.vertex_array.draw(&mut render_pass);
        }
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
