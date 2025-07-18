use egui_custom::util::comandi::ExecutableCommand;
use crate::entity::dipendenti::Dipendente;
use strum::AsRefStr;

#[derive(Clone, Default, AsRefStr)]
pub enum ComandoFerie {
    AggiungiFerie(String, Dipendente),
    RimuoviFerie(String, Dipendente),
    #[default]
    NessunComando,
}

impl ExecutableCommand for ComandoFerie {
    fn execute(&mut self) {
        match self {
            ComandoFerie::AggiungiFerie(giorno, dip) => dip.add_ferie(giorno.clone()),
            ComandoFerie::RimuoviFerie(giorno, dip) => dip.remove_ferie(giorno.clone()),
            ComandoFerie::NessunComando => {}
        }
    }
}
