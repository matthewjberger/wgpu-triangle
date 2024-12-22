fn main() {
    if let Err(error) = engine::launch(App::default()) {
        eprintln!("Failed to launch app: {error}");
    }
}

#[derive(Default)]
pub struct App {
    panels_visible: bool,
    cycles: u64,
}

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

    fn update(&mut self, _context: &mut engine::Context) {
        self.cycles = self.cycles.saturating_add(1);
        if self.cycles % (60 * 5) == 0 {
            engine::log::info!("Five seconds elapsed");
        }
    }

    fn ui(&mut self, _context: &mut engine::Context, ui: &engine::egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        let title = "Rust/Wgpu";

        #[cfg(feature = "webgpu")]
        let title = "Rust/Wgpu/Webgpu";

        #[cfg(feature = "webgl")]
        let title = "Rust/Wgpu/Webgl";

        if self.panels_visible {
            engine::egui::TopBottomPanel::top("top").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("File");
                    ui.label("Edit");
                });
            });

            engine::egui::SidePanel::left("left").show(ui, |ui| {
                ui.heading("Scene Explorer");
                if ui.button("Click me!").clicked() {
                    engine::log::info!("Button clicked!");
                }
            });

            engine::egui::SidePanel::right("right").show(ui, |ui| {
                ui.heading("Inspector");
                if ui.button("Click me!").clicked() {
                    engine::log::info!("Button clicked!");
                }
            });

            engine::egui::TopBottomPanel::bottom("bottom").show(ui, |ui| {
                ui.heading("Assets");
                if ui.button("Click me!").clicked() {
                    engine::log::info!("Button clicked!");
                }
            });
        }

        engine::egui::Window::new(title).show(ui, |ui| {
            ui.checkbox(&mut self.panels_visible, "Show Panels");
        });
    }
}
