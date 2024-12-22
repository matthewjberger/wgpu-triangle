use engine::winit::event::WindowEvent;

fn main() {
    if let Err(error) = engine::launch(App) {
        eprintln!("Failed to launch app: {error}");
    }
}

#[derive(Default)]
pub struct App;

impl engine::State for App {
    fn initialize(&mut self, _context: &mut engine::Context) {}

    fn resize(&mut self, _context: &mut engine::Context, _width: u32, _height: u32) {}

    fn receive_event(&mut self, _context: &mut engine::Context, _event: &WindowEvent) {}

    fn update(&mut self, _context: &mut engine::Context) {}

    fn ui(&mut self, _context: &mut engine::Context, ui: &engine::egui::Context) {
        engine::egui::Window::new("Template").show(ui, |ui| {
            ui.heading("Template App");
        });
    }
}
