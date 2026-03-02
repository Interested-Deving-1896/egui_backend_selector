use crate::app::EguiApp;
use egui_backend_selector::{Backend, BackendConfiguration};
use log::LevelFilter;

#[path = "app/app.rs"]
mod app;

fn main() {
    _ = trivial_log::init_std(LevelFilter::Trace);

    egui_backend_selector::overwrite_backend(Backend::SoftwareBackend);

    egui_backend_selector::run_app(
        "egui-backend-selector-test",
        BackendConfiguration::default(),
        |e, s| EguiApp::new(e, s),
    )
    .expect("failed to run app");
}
