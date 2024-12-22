fn main() {
    if let Err(error) = engine::launch(App::default()) {
        eprintln!("Failed to launch app: {error}");
    }
}

#[derive(Default)]
pub struct App {}

impl engine::State for App {
    fn initialize(&mut self, _context: &mut engine::Context) {
        engine::log::info!("App initialized!");
    }

    fn receive_event(
        &mut self,
        _context: &mut engine::Context,
        _event: &engine::winit::event::WindowEvent,
    ) {
        engine::log::info!("Event received: {:?}", _event);
    }

    fn update(&mut self, _context: &mut engine::Context) {}

    fn ui(&mut self, _context: &mut engine::Context, _ui: &engine::egui::Context) {}
}
