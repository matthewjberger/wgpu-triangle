fn main() {
    engine::launch(App);
}

#[derive(Default)]
pub struct App;

impl engine::State for App {
    fn update(
        &mut self,
        _engine_context: &mut engine::Context,
        ui_context: &engine::egui::Context,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        let title = "Rust/Wgpu";

        #[cfg(feature = "webgpu")]
        let title = "Rust/Wgpu/Webgpu";

        #[cfg(feature = "webgl")]
        let title = "Rust/Wgpu/Webgl";

        engine::egui::Window::new(title).show(ui_context, |ui| {
            ui.heading("Hello, world!");
            if ui.button("Click me!").clicked() {
                engine::log::info!("Button clicked!");
            }
        });
    }
}
