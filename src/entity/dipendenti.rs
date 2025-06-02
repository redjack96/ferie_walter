use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Dipendenti {
    pub nome: String,
    pub ferie: Vec<String>,
}