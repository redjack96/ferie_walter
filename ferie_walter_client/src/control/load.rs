use egui_custom::prelude::{LoadingState, Shared};
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use crate::entity::dipendenti::Dipendente;
use crate::gui::ui::FerieWalter;

pub struct DatiFerie {
    pub dip: Vec<Dipendente>,
    pub fes: Vec<String>
}

impl DatiFerie {
    pub fn load_from(f: FerieWalter) -> Self {
        let (dip, fes) = (f.dipendenti, f.festivita);
        Self {
            dip, fes
        }
    }
}

pub fn start_async_load(mut shared_loading: Shared<LoadingState>, mut shared_res: Shared<Option<DatiFerie>>) {
    // Evita chiamate ripetute
    shared_loading.write(LoadingState::Loading);
    spawn_local(async move {
        match FerieWalter::load().await {
            Ok(data) => {
                info!("✅ Caricati con successo!");
                shared_loading.write(LoadingState::Loaded);
                shared_res.write(Some(DatiFerie::load_from(data)));
            }
            Err(err) => {
                error!("❌ Errore durante il fetch: {:?}", err);
                shared_loading.write(LoadingState::NotLoaded);
                shared_res.write(None);
            }
        }
    });
}
