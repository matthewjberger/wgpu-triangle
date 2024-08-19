fn main() {
    if let Err(error) = engine::launch(App::default()) {
        eprintln!("Failed to launch app: {error}");
    }
}

#[derive(Default)]
pub struct App {
    panels_visible: bool,
}

impl engine::State for App {
    fn update(&mut self, _engine_context: &mut engine::Context, context: &engine::egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        let title = "Rust/Wgpu";

        #[cfg(feature = "webgpu")]
        let title = "Rust/Wgpu/Webgpu";

        #[cfg(feature = "webgl")]
        let title = "Rust/Wgpu/Webgl";

        if self.panels_visible {
            engine::egui::TopBottomPanel::top("top").show(context, |ui| {
                ui.horizontal(|ui| {
                    ui.label("File");
                    ui.label("Edit");
                });
            });

            engine::egui::SidePanel::left("left").show(context, |ui| {
                ui.heading("Scene Explorer");
                if ui.button("Click me!").clicked() {
                    engine::log::info!("Button clicked!");
                }
            });

            engine::egui::SidePanel::right("right").show(context, |ui| {
                ui.heading("Inspector");
                if ui.button("Click me!").clicked() {
                    engine::log::info!("Button clicked!");
                }
            });

            engine::egui::TopBottomPanel::bottom("bottom").show(context, |ui| {
                ui.heading("Assets");
                if ui.button("Click me!").clicked() {
                    engine::log::info!("Button clicked!");
                }
            });
        }

        engine::egui::Window::new(title).show(context, |ui| {
            ui.checkbox(&mut self.panels_visible, "Show Panels");
        });
    }
}
