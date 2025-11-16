pub mod ui;
pub mod world_manager;

use std::sync::{Arc, Mutex};

use egui::ThemePreference;
use egui_wgpu::WgpuConfiguration;

use crate::ui::View;
use crate::world_manager::WorldManager;

struct App {
    world_manager: Arc<Mutex<WorldManager>>,
    ui: View,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        let world_manager = Arc::new(Mutex::new(WorldManager::new()));

        let controller = ui::Controller::new(Arc::clone(&world_manager));
        let view = ui::View::new(controller);

        Self {
            world_manager,
            ui: view,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui.ui(ctx);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        centered: true,
        vsync: false,
        wgpu_options: WgpuConfiguration {
            present_mode: eframe::wgpu::PresentMode::Mailbox,
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Forma",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;

    Ok(())
}
