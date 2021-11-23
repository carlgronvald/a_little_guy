use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use super::{
    pipeline::Pipeline,
    texture::Texture,
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
    diffuse_texture: Texture,
    default_uniforms: Uniform<DefaultUniforms>,
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

        let diffuse_bytes = include_bytes!("Trees.png");
        let diffuse_texture = Texture::new(&device, &queue, diffuse_bytes);
        let (texture_bind_group_layout, texture_bind_group) =
            diffuse_texture.create_bind_group(&device);

        let default_uniforms = Uniform::new(
            &device,
            DefaultUniforms::new(
                1.0 / (size.width as f32),
                1.0 / (size.height as f32),
                [0.0, 0.0],
            ),
        );
        let (default_bind_group_layout, default_bind_group) =
            default_uniforms.create_bind_group(&device);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        // Create our main pipeline
        let pipeline = Pipeline::new(
            &device,
            &shader,
            &config,
            &[&texture_bind_group_layout, &default_bind_group_layout],
            vec![texture_bind_group, default_bind_group],
        );

        println!("Returning renderer");
        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            diffuse_texture,
            default_uniforms,
        }
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

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        let vertex_array = draw_state.render(&self.device, &self.queue, &mut self.default_uniforms);

        //self.default_uniforms.update_uniform(|x| { x.x_scale *= 0.99}, &self.queue);
        self.pipeline.set(&mut render_pass);
        vertex_array.draw(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
