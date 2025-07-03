use egui_custom::util::shared::serde_shared;
use egui_custom::util::common::serde_common;
use egui_custom::prelude::{Common, Shared};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Dipendente {
    pub nome: String,
    #[serde(with = "serde_shared")]
    pub ferie: Shared<Vec<String>>,
}

impl Dipendente {
    pub fn ferie_in_questa_data(&self, data_input: &str) -> bool {
        self.ferie.read().contains(&data_input.to_string())
    }
    pub fn add_ferie(&mut self, ferie: String) {
        self.ferie.get_mut().push(ferie);
    }

    pub fn remove_ferie(&mut self, ferie: String) {
        self.ferie.get_mut().retain(|data| data != &ferie);
    }
}
