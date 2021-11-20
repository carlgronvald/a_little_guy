use super::renderer::Renderer;
use super::ExternalEvent;
use crate::channels::{LogicToWindowReceiver, WindowToLogicSender};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

pub fn start_window(rx: LogicToWindowReceiver, tx: WindowToLogicSender) {
    let window = unsafe { Window::new(rx) };

    let eh: EventHandler = Box::new(move |event| {
        if let Some(event) = ExternalEvent::create_from_glut_event(event) {
            tx.channel_sender.send(event).unwrap();
        }
    });

    unsafe { window.run(eh) };
}

pub struct Window {
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    window : winit::window::Window,
    rx: LogicToWindowReceiver,
    state: Renderer,
}

pub type EventHandler = Box<dyn FnMut(winit::event::Event<()>) + Send + 'static>;

impl Window {
    ///
    /// unsafe, since calling twice on the same thread is likely to lead to serious trouble.
    /// Also, extremely stateful.
    pub unsafe fn new(rx: LogicToWindowReceiver) -> Window {
        let el = winit::event_loop::EventLoop::new();
        let wb = winit::window::WindowBuilder::new()
            .with_title("Hello world!")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));
        let window = wb.build(&el).unwrap();

        let mut state = pollster::block_on(Renderer::new(&window));

        let screen_dimensions = (
            window.inner_size().width,
            window.inner_size().height,
        );


        //info!("Loading wgpu!");
        //graphics::RenderCaller::initialize_gl(&windowed_context);
        // let render_caller = RenderCaller::new(screen_dimensions);

        let res = Window {
            event_loop: Some(el),
            window,
            rx,
            state,
        };

        res
    }

    // fn get_capabilities(&self) -> GraphicsCapabilities {
    //     GraphicsCapabilities {
    //         vbo_count: self.render_caller.get_vbo_count(),
    //         texture_metadata: self
    //             .render_caller
    //             .get_texture_manager()
    //             .get_texture_metadata(),
    //         shader_metadata: self
    //             .render_caller
    //             .get_shader_manager()
    //             .get_shader_metadata(),
    //         framebuffer_metadata: self
    //             .render_caller
    //             .get_framebuffer_manager()
    //             .get_framebuffer_metadata(),
    //         screen_dimensions: (
    //             self.context.window().inner_size().width,
    //             self.context.window().inner_size().height,
    //         ),
    //     }
    // }

    // fn send_capabilities(&self) {
    //     self.capabilities_sender
    //         .channel_sender
    //         .send(self.get_capabilities())
    //         .unwrap();
    // }

    fn render(&mut self) {
        println!("Rendering!");
        self.state.render().unwrap();
    }

    fn update_screen_dimensions(&mut self, screen_dimensions: PhysicalSize<u32>) {
        self.state.resize(screen_dimensions);

        //gl::Viewport(0, 0, screen_dimensions.0 as i32, screen_dimensions.1 as i32);
        // self.render_caller
        //     .update_screen_dimensions(screen_dimensions);
        // self.send_capabilities();
    }

    ///
    /// Starts this graphics object
    ///
    /// NOTE: THE EXECUTION GOES TO THE GRAPHICS OBJECT WHEN THIS IS CALLED!
    ///
    pub unsafe fn run(mut self, mut event_handler: EventHandler) {
        //Ignore the result from the function.
        let _ = self.window.set_cursor_grab(true);
        self.window.set_cursor_visible(false);
        if let Some(el) = self.event_loop.take() {
            el.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                println!("Event: {:?}", event);

                match event {
                    Event::LoopDestroyed => {} //TODO: HANDLE LOOP DESTROYED
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::Resized(physical_size) => {
                            //self.window.resize(physical_size);
                            self.update_screen_dimensions(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.update_screen_dimensions(new_inner_size.to_owned());
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => event_handler(Event::WindowEvent { window_id, event }),
                    },
                    Event::RedrawRequested(_) => {}
                    Event::NewEvents(cs) => match cs {
                        winit::event::StartCause::Poll => {
                            // Perform a render
                            self.render();
                        }
                        _ => event_handler(Event::NewEvents(cs)),
                    },
                    _ => event_handler(event),
                }
            });
        } else {
            panic!("Graphics object was told to run, but the event loop is already consumed!");
        }
    }

    //pub fn message(&mut self, messages: RenderPack<T>) {
    //    self.render_messages = Some(messages);
    //}
}
