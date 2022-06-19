mod pycsort;

use eframe::{
    egui::{CentralPanel, ProgressBar, Vec2},
    run_native, App, NativeOptions,
};

use pycsort::PycSort;

impl App for PycSort {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(16.);

            // Header
            ui.heading("PycSort RS");

            ui.add_space(16.);
            ui.separator();
            ui.add_space(16.);

            ui.vertical_centered(|ui| {
                // Current Folder
                ui.label(
                    "Found ".to_owned()
                        + &self.num_files.to_string()
                        + " pictures/videos in "
                        + &self.folder.split('/').last().unwrap(),
                );

                ui.add_space(16.);

                // File Select Button
                if ui.button("Choose Folder").clicked() {
                    self.change_folder();
                }
                ui.add_space(8.);

                // Rescan
                if ui.button("Rescan Folder").clicked() {
                    self.rescan_folder();
                }
            });

            ui.add_space(16.);
            ui.separator();
            ui.add_space(16.);

            ui.vertical_centered(|ui| {
                // Status Label
                ui.label(&self.status);

                ui.add_space(16.);

                // Sort Button
                if ui.button("Sort").clicked() {
                    self.sort_files();
                }
            });
            ui.add_space(32.);

            // Progress Bar
            let progress_bar = ProgressBar::new(self.progress).show_percentage();
            ui.add(progress_bar);
        });
    }
}

fn main() {
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(620.0, 350.0));
    run_native(
        "PycSort RS",
        win_option,
        Box::new(|cc| Box::new(PycSort::new(cc))),
    )
}
