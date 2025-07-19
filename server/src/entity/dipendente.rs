use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Dipendente {
    pub nome: String,
    pub ferie: Vec<String>,
}

impl Default for Dipendente {
    fn default() -> Self {
        Self {
            nome: "Walter".to_string(),
            ferie: vec![],
        }
    }
}
