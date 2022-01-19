mod pycsort;

use eframe::{
    egui::{CentralPanel, ProgressBar, Vec2},
    epi::App,
    run_native, NativeOptions,
};

use pycsort::{PycSort, SPACING};

impl App for PycSort {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.font_config(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Header
                ui.heading("PycSort RS");
                ui.add_space(SPACING);

                ui.separator();
                ui.add_space(SPACING);

                // Current Folder
                ui.label(
                    "Found ".to_owned()
                        + &self.num_files.to_string()
                        + " pictures/videos in "
                        + &self.folder,
                );
                ui.add_space(SPACING);

                // File Select Button
                if ui.button("Choose Folder").clicked() {
                    self.change_folder();
                }
                ui.add_space(SPACING);

                // Rescan
                if ui.button("Rescan Folder").clicked() {
                    self.rescan_folder();
                }
                ui.add_space(SPACING);

                ui.separator();
                ui.add_space(SPACING);

                // Status Label
                ui.label(&self.status);
                ui.add_space(SPACING);

                // Sort Button
                if ui.button("Sort").clicked() {
                    self.sort_files();
                }
                ui.add_space(SPACING);
            });

            // Progress Bar
            let progress_bar = ProgressBar::new(self.progress).show_percentage();
            ui.add(progress_bar);
            ui.add_space(SPACING);
            ui.separator();
        });
    }

    fn name(&self) -> &str {
        "PycSort RS"
    }
}

fn main() {
    let app = PycSort::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(620.0, 350.0));
    run_native(Box::new(app), win_option)
}
