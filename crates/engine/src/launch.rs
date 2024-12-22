use crate::renderer::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Theme, Window, WindowId},
};

#[cfg(target_arch = "wasm32")]
use futures::channel::oneshot::Receiver;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
pub use web_time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Data shared between engine and application layers
pub struct Context {
    renderer: Renderer,
    window: Arc<Window>,
}

impl Context {
    fn new(window: Arc<Window>, renderer: Renderer) -> Self {
        Self { renderer, window }
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }
}

pub trait State {
    fn initialize(&mut self, _context: &mut Context) {}
    fn resize(&mut self, _context: &mut Context, _width: u32, _height: u32) {}
    fn receive_event(&mut self, _context: &mut Context, _event: &WindowEvent) {}
    fn update(&mut self, _context: &mut Context) {}
    fn ui(&mut self, _context: &mut Context, _ui: &egui::Context) {}
}

pub fn launch(initial_state: impl State + 'static) -> Result<(), winit::error::EventLoopError> {
    let event_loop = winit::event_loop::EventLoop::builder().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(initial_state);
    event_loop.run_app(&mut app)
}

#[derive(Default)]
struct App {
    state: Option<Box<dyn State>>,
    gui_state: Option<egui_winit::State>,
    app_context: Option<Context>,
    last_render_time: Option<Instant>,
    #[cfg(target_arch = "wasm32")]
    renderer_receiver: Option<Receiver<Renderer>>,
    last_size: (u32, u32),
}

impl App {
    pub fn new(state: impl State + 'static) -> Self {
        Self {
            state: Some(Box::new(state)),
            ..Default::default()
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();

        #[cfg(not(target_arch = "wasm32"))]
        {
            attributes = attributes.with_title("Standalone Winit/Wgpu Example");
        }

        #[allow(unused_assignments)]
        #[cfg(target_arch = "wasm32")]
        let (mut canvas_width, mut canvas_height) = (0, 0);

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            canvas_width = canvas.width();
            canvas_height = canvas.height();
            self.last_size = (canvas_width, canvas_height);
            attributes = attributes.with_canvas(Some(canvas));
        }

        if let Ok(window) = event_loop.create_window(attributes) {
            let first_window_handle = self.app_context.is_none();
            let window_handle = Arc::new(window);

            if first_window_handle {
                let gui_context = egui::Context::default();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let inner_size = window_handle.inner_size();
                    self.last_size = (inner_size.width, inner_size.height);
                }

                #[cfg(target_arch = "wasm32")]
                {
                    gui_context.set_pixels_per_point(window_handle.scale_factor() as f32);
                }

                let viewport_id = gui_context.viewport_id();
                let gui_state = egui_winit::State::new(
                    gui_context,
                    viewport_id,
                    &window_handle,
                    Some(window_handle.scale_factor() as _),
                    Some(Theme::Dark),
                    None,
                );

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let (width, height) = (
                        window_handle.inner_size().width,
                        window_handle.inner_size().height,
                    );

                    env_logger::init();
                    let renderer_window_handle = window_handle.clone();
                    let renderer = pollster::block_on(async move {
                        Renderer::new(renderer_window_handle, width, height).await
                    });

                    let mut context = Context::new(window_handle.clone(), renderer);
                    if let Some(state) = self.state.as_mut() {
                        state.initialize(&mut context);
                    }
                    self.app_context = Some(context);
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let (sender, receiver) = futures::channel::oneshot::channel();
                    self.renderer_receiver = Some(receiver);
                    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                    console_log::init().expect("Failed to initialize logger!");
                    log::info!("Canvas dimensions: ({canvas_width} x {canvas_height})");
                    let window_handle = window_handle.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let renderer =
                            Renderer::new(window_handle.clone(), canvas_width, canvas_height).await;
                        if sender.send(renderer).is_err() {
                            log::error!("Failed to create and send renderer!");
                        }
                    });
                }

                self.gui_state = Some(gui_state);
                self.last_render_time = Some(Instant::now());
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(receiver) = self.renderer_receiver.take() {
                if let Ok(renderer) = receiver.try_recv() {
                    let window = self.app_context.as_ref().map(|ctx| ctx.window().clone());
                    if let Some(window) = window {
                        let context = Context::new(window, renderer);
                        if let Some(state) = self.state.as_mut() {
                            state.initialize(&mut context);
                        }
                        self.app_context = Some(context);
                    }
                } else {
                    self.renderer_receiver = Some(receiver);
                }
            }
        }

        let (Some(gui_state), Some(last_render_time), Some(state), Some(context)) = (
            self.gui_state.as_mut(),
            self.last_render_time.as_mut(),
            self.state.as_mut(),
            self.app_context.as_mut(),
        ) else {
            return;
        };

        // Receive gui window event
        if gui_state.on_window_event(context.window(), &event).consumed {
            return;
        }

        // If the gui didn't consume the event, handle it
        match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                let (width, height) = ((width).max(1), (height).max(1));
                log::info!("Resizing renderer surface to: ({width}, {height})");
                context.renderer.resize(width, height);
                state.resize(context, width, height);
                self.last_size = (width, height);
            }
            WindowEvent::CloseRequested => {
                log::info!("Close requested. Exiting...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta_time = now - *last_render_time;
                *last_render_time = now;

                state.update(context);

                let gui_input = gui_state.take_egui_input(context.window());
                gui_state.egui_ctx().begin_pass(gui_input);

                state.ui(context, gui_state.egui_ctx());

                let egui_winit::egui::FullOutput {
                    textures_delta,
                    shapes,
                    pixels_per_point,
                    ..
                } = gui_state.egui_ctx().end_pass();

                let paint_jobs = gui_state.egui_ctx().tessellate(shapes, pixels_per_point);

                let screen_descriptor = {
                    let (width, height) = self.last_size;
                    egui_wgpu::ScreenDescriptor {
                        size_in_pixels: [width, height],
                        pixels_per_point: context.window().scale_factor() as f32,
                    }
                };

                context.renderer.render_frame(
                    screen_descriptor,
                    paint_jobs,
                    textures_delta,
                    delta_time,
                );
            }
            event => state.receive_event(context, &event),
        }

        context.window().request_redraw();
    }
}
