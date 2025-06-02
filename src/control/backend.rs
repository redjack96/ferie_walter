use crate::control::comandi::ComandoFerie;
use crate::gui::ui::FerieWalter;
use egui_custom::prelude::Shared;
use egui_custom::util::comandi::Backend;

impl Backend<ComandoFerie> for FerieWalter {
    type Container = Shared<Vec<ComandoFerie>>;

    fn esegui(&self, comando: &ComandoFerie) {
        match comando {
            ComandoFerie::AggiungiFerie(giorno, dip) => {
                dip.add_ferie(giorno.clone())
            }
            ComandoFerie::RimuoviFerie(giorno, dip) => {
                dip.remove_ferie(giorno.clone())
            }
            ComandoFerie::NessunComando => {}
        }
    }

    fn esegui_tutti(&self, comandi: Self::Container) {
        for c in comandi.read().iter() {
            self.esegui(c);
        }
        comandi.get_mut().clear();
    }
}