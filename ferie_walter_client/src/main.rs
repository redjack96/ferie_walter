#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod control;
mod entity;
mod gui;

use log::{error};
use crate::gui::ui::FerieWalter;

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    // eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");
        let ferie = match FerieWalter::load().await {
            Ok(ferie) => ferie,
            Err(e) => {
                error!(
                    "Impossibile comunicare con il server. Esegui 'cargo run -p server': {}",
                    e.to_string()
                );
                return;
            }
        };

        let start_result = eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(|_| Ok(Box::new(ferie))))
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
use crate::entity::errori::ErroreApplicazione;
#[cfg(not(target_arch = "wasm32"))]
const FILE_JSON: &str = "dipendenti.json";
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), ErroreApplicazione> {
    use egui::ThemePreference;
    use std::fs;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size([1200.0, 740.0]),
        centered: true,
        ..Default::default()
    };

    //LEGGE IL FILE JSON SE ESISTE = lo deserializza nell'oggetto di rust FerieWalter
    let dati_app = if let Some(contenuto) = fs::read_to_string(FILE_JSON).ok() {
        if let Ok(dati_app) = serde_json::from_str::<FerieWalter>(&contenuto) {
            // Aggiungo il giocatore letto dal file corrispondente
            Ok(dati_app)
        } else {
            return Err(ErroreApplicazione::ImpossibileLeggereFileJson);
        }
    } else {
        //SE NON ESISTE IL JSON = restituisce un oggetto FerieWalter di default
        Ok(FerieWalter::default())
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
