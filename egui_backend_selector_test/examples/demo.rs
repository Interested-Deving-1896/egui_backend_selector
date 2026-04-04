use egui::Ui;
use egui_backend_selector::{BackendConfiguration, BackendInterop};
use egui_demo_lib::{ColorTest, DemoWindows};
use log::LevelFilter;

struct EguiApp {
    demo: DemoWindows,
    color_test: ColorTest,
}

impl egui_backend_selector::App for EguiApp {
    fn ui(&mut self, ui: &mut Ui, _backend: BackendInterop<'_>) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.demo.ui(ui);
            egui::Window::new("Color Test").show(ui, |ui| {
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    self.color_test.ui(ui);
                });
            });
        });
    }
}
fn main() {
    _ = trivial_log::init_std(LevelFilter::Info);

    egui_backend_selector::run_app(
        "egui-backend-selector-test",
        BackendConfiguration::default(),
        |ctx, _s| {
            egui_extras::install_image_loaders(&ctx);
            EguiApp {
                demo: Default::default(),
                color_test: Default::default(),
            }
        },
    )
    .expect("failed to run app");
}
