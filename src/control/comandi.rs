use egui_custom::util::comandi::ComandoApp;
use strum::AsRefStr;
use crate::entity::dipendenti::Dipendente;

#[derive(Clone, Default, AsRefStr)]
pub enum ComandoFerie {
    AggiungiFerie(String, Dipendente),
    RimuoviFerie(String, Dipendente),
    #[default]
    NessunComando
}

impl ComandoApp for ComandoFerie{}