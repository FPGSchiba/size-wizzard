use eframe::egui;

use crate::Tree;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SizeWizzardApp {
    tree: Tree,
    is_scanning: bool,
}

impl Default for SizeWizzardApp {
    fn default() -> Self {
        Self {
            tree: Tree::empty(),
            is_scanning: false,
        }
    }
}

impl SizeWizzardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SizeWizzardApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Directory Tree");

            if ui.button("Scan").clicked() {
                // TODO: Disable and start processs
                self.is_scanning = true;
                self.tree = Tree::construct();
            }

            ui.separator();

            self.tree.ui(ui);

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
