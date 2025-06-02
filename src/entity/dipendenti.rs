use egui_custom::util::common::serde_common;
use egui_custom::prelude::Common;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Dipendente {
    pub nome: String,
    #[serde(with = "serde_common")]
    pub ferie: Common<Vec<String>>,
}

impl Dipendente {
    pub fn ferie_in_questa_data(&self, data_input: &str) -> bool {
        self.ferie.read().contains(&data_input.to_string())
    }
    pub fn add_ferie(&self, ferie: String) {
        self.ferie.get_mut().push(ferie);
    }

    pub fn remove_ferie(&self, ferie: String) {
        self.ferie.get_mut().retain(|data| data != &ferie);
    }
}
