use crate::entity::dipendenti::Dipendente;

pub struct DatiFerie {
    pub dip: Vec<Dipendente>,
    pub fes: Vec<String>
}

#[cfg(target_arch = "wasm32")]
impl DatiFerie {
    pub fn load_from(f: crate::gui::ui::FerieWalter) -> Self {
        let (dip, fes) = (f.dipendenti, f.festivita);
        Self {
            dip, fes
        }
    }
}

#[cfg(target_arch = "wasm32")]
use egui_custom::prelude::{LoadingState, Shared};
#[cfg(target_arch = "wasm32")]
pub fn start_async_load(mut shared_loading: Shared<LoadingState>, mut shared_res: Shared<Option<DatiFerie>>) {
    // Evita chiamate ripetute
    shared_loading.write(LoadingState::Loading);
    use log::{error, info};
    wasm_bindgen_futures::spawn_local(async move {
        match crate::FerieWalter::load().await {
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
