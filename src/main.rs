mod entity;
mod gui;

use crate::gui::ui::FerieWalter;
use egui::ThemePreference;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Ferie Walter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); //fa partire le immagini
            cc.egui_ctx.set_theme(ThemePreference::Dark); // quando si avvia l'app, inizia dark
            Ok(Box::new(FerieWalter::default()))
        }),
    )
    .expect("Impossibile eseguire l'applicazione!");
}
