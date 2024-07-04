#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
pub use web_time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Data shared between engine and application layers
#[derive(Default)]
pub struct Context {}

pub trait State {
    fn update(&mut self, _engine_context: &mut Context, _ui_context: &egui::Context);
}

pub fn launch(state: impl State + 'static) {
    let event_loop = winit::event_loop::EventLoopBuilder::with_user_event()
        .build()
        .expect("Failed to create event loop");
    let mut window_builder = winit::window::WindowBuilder::new();

    #[cfg(not(target_arch = "wasm32"))]
    {
        window_builder = window_builder.with_title("Standalone Winit/Wgpu Example");
    }

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        window_builder = window_builder.with_canvas(Some(canvas));
    }

    let window = window_builder
        .build(&event_loop)
        .expect("Failed to create window!");

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run_app(event_loop, window, state));
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run_app(event_loop, window, state));
    }
}

async fn run_app(
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    mut app: impl State + 'static,
) {
    let window = std::sync::Arc::new(window);

    let gui_context = egui::Context::default();

    gui_context.set_pixels_per_point(window.scale_factor() as f32);
    let viewport_id = gui_context.viewport_id();
    let mut gui_state = egui_winit::State::new(
        gui_context,
        viewport_id,
        &window,
        Some(window.scale_factor() as _),
        None,
    );

    #[cfg(not(target_arch = "wasm32"))]
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    #[cfg(not(target_arch = "wasm32"))]
    let (width, height) = (window.inner_size().width, window.inner_size().height.min(1));

    #[cfg(target_arch = "wasm32")]
    let (width, height) = (1280, 720);

    let mut renderer = crate::renderer::Renderer::new(window.clone(), width, height).await;

    let mut app_context = Context::default();

    let mut last_render_time = Instant::now();

    event_loop
        .run(move |event, elwt| {
            match event {
                winit::event::Event::AboutToWait => window.request_redraw(),

                winit::event::Event::WindowEvent { ref event, .. } => {
                    // Receive gui window event
                    if gui_state.on_window_event(&window, event).consumed {
                        return;
                    }

                    // If the gui didn't consume the event, handle it
                    match event {
                        winit::event::WindowEvent::KeyboardInput {
                            event:
                                winit::event::KeyEvent {
                                    physical_key: winit::keyboard::PhysicalKey::Code(key_code),
                                    ..
                                },
                            ..
                        } => {
                            // Exit by pressing the escape key
                            if matches!(key_code, winit::keyboard::KeyCode::Escape) {
                                elwt.exit();
                            }
                        }

                        // Close button handler
                        winit::event::WindowEvent::CloseRequested => {
                            log::info!("The close button was pressed; stopping");
                            elwt.exit();
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize {
                            width,
                            height,
                        }) => {
                            let (width, height) = ((*width).max(1), (*height).max(1));
                            log::info!("Resizing renderer surface to: ({width}, {height})");
                            renderer.resize(width, height);
                        }

                        winit::event::WindowEvent::RedrawRequested => {
                            let now = Instant::now();
                            let delta_time = now - last_render_time;
                            last_render_time = now;

                            let gui_input = gui_state.take_egui_input(&window);
                            gui_state.egui_ctx().begin_frame(gui_input);

                            app.update(&mut app_context, gui_state.egui_ctx());

                            let egui::FullOutput {
                                textures_delta,
                                shapes,
                                pixels_per_point,
                                ..
                            } = gui_state.egui_ctx().end_frame();

                            let paint_jobs =
                                gui_state.egui_ctx().tessellate(shapes, pixels_per_point);

                            #[cfg(not(target_arch = "wasm32"))]
                            let screen_descriptor = {
                                let window_size = window.inner_size();
                                egui_wgpu::ScreenDescriptor {
                                    size_in_pixels: [window_size.width, window_size.height],
                                    pixels_per_point: window.scale_factor() as f32,
                                }
                            };

                            #[cfg(target_arch = "wasm32")]
                            let screen_descriptor = {
                                egui_wgpu::ScreenDescriptor {
                                    size_in_pixels: [1280, 720],
                                    pixels_per_point: window.scale_factor() as f32,
                                }
                            };

                            renderer.render_frame(
                                screen_descriptor,
                                paint_jobs,
                                textures_delta,
                                delta_time,
                            );
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        })
        .unwrap();
}
