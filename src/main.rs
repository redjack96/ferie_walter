#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod entity;
mod gui;
mod control;

use crate::entity::errori::ErroreApplicazione;
use crate::gui::ui::FerieWalter;
use egui::ThemePreference;
use std::fs;

const FILE_JSON: &str = "dipendenti.json";

fn main() -> Result<(), ErroreApplicazione> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size([1200.0, 740.0]),
        centered: true,
        ..Default::default()
    };

    let dati_app = if let Some(contenuto) = fs::read_to_string(FILE_JSON).ok() {
        if let Ok(dati_app) = serde_json::from_str::<FerieWalter>(&contenuto) {
            // Aggiungo il giocatore letto dal file corrispondente
            Ok(dati_app)
        } else {
            return Err(ErroreApplicazione::ImpossibileLeggereFileJson);
        }
    } else {
        return Err(ErroreApplicazione::ImpossibileTrovareFile);
    }?;

    eframe::run_native(
        "Ferie Walter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); //fa partire le immagini
            cc.egui_ctx.set_theme(ThemePreference::Dark); // quando si avvia l'app, inizia dark
            Ok(Box::new(dati_app))
        }),
    )
    .map_err(|_| ErroreApplicazione::ErroreEgui)?;

    Ok(())
}
