use eframe::Storage;
use egui_backend_selector::BackendInterop;
use log::info;

pub struct EguiApp {
    frame_counter: u128,
    data: String,
}

impl EguiApp {
    pub fn new(context: egui::Context, storage: Option<&dyn Storage>) -> Self {
        egui_extras::install_image_loaders(&context);
        let data = storage
            .map(|storage| storage.get_string("payload").unwrap_or_default())
            .unwrap_or_default();

        EguiApp {
            data,
            frame_counter: 0,
        }
    }
}

impl egui_backend_selector::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, backend: BackendInterop<'_>) {
        self.frame_counter += 1;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Hello World! Running on {}",
                backend.backend_name()
            ));
            ui.label("Persistent String:");
            ui.text_edit_singleline(&mut self.data);
            ui.label(format!("Frame Counter: {}", self.frame_counter));
        });
    }

    fn on_exit(&mut self) {
        info!("EXIT CALLED");
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        info!("SAVE CALLED");
        storage.set_string("payload", self.data.clone());
    }
}
