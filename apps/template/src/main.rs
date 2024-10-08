fn main() {
    if let Err(error) = engine::launch(App) {
        eprintln!("Failed to launch app: {error}");
    }
}

#[derive(Default)]
pub struct App;

impl engine::State for App {
    fn update(
        &mut self,
        _engine_context: &mut engine::Context,
        ui_context: &engine::egui::Context,
    ) {
        engine::egui::Window::new("Template").show(ui_context, |ui| {
            ui.heading("Template App");
        });
    }
}
