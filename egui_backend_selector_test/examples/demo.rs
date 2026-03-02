use egui::Context;
use egui_demo_lib::{ColorTest, DemoWindows};
use log::LevelFilter;
use egui_backend_selector::{BackendConfiguration, BackendInterop};


struct EguiApp {
    demo: DemoWindows,
    color_test: ColorTest,
}

impl egui_backend_selector::App for EguiApp {
    fn update(&mut self, context: &Context, _backend: BackendInterop<'_>) {
        egui::CentralPanel::default().show(context, |ui| {
            self.demo.ui(context);
            egui::Window::new("Color Test").show(context, |ui| {
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
        |e, s| EguiApp {
            demo: Default::default(),
            color_test: Default::default(),
        },
    )
        .expect("failed to run app");
}
