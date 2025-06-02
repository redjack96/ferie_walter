use strum::{AsRefStr, EnumIter};

#[derive(Default, PartialEq, Eq, EnumIter, AsRefStr, Clone, Copy)]
pub enum Mese {
    Gennaio,
    Febbraio,
    Marzo,
    Aprile,
    Maggio,
    #[default]
    Giugno,
    Luglio,
    Agosto,
    Settembre,
    Ottobre,
    Novembre,
    Dicembre,
}
