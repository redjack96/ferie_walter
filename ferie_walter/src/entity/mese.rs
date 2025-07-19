use strum::{AsRefStr, EnumIter};

#[derive(Default, Debug, PartialEq, Eq, EnumIter, AsRefStr, Clone, Copy)]
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

#[allow(unused)]
impl Mese {
    /// Restituisce l'indice del mese partendo da 0
    pub fn to_index(&self) -> u32 {
        match self {
            Mese::Gennaio => 0,
            Mese::Febbraio => 1,
            Mese::Marzo => 2,
            Mese::Aprile => 3,
            Mese::Maggio => 4,
            Mese::Giugno => 5,
            Mese::Luglio => 6,
            Mese::Agosto => 7,
            Mese::Settembre => 8,
            Mese::Ottobre => 9,
            Mese::Novembre => 10,
            Mese::Dicembre => 11,
        }
    }

    /// Restituisce il numero del mese partendo da 1, come richiesto da NaiveDate::from_ymd_opt() (nonostante non venga scritto nella documentazione)
    pub fn to_ordinal(&self) -> u32 {
        match self {
            Mese::Gennaio => 1,
            Mese::Febbraio => 2,
            Mese::Marzo => 3,
            Mese::Aprile => 4,
            Mese::Maggio => 5,
            Mese::Giugno => 6,
            Mese::Luglio => 7,
            Mese::Agosto => 8,
            Mese::Settembre => 9,
            Mese::Ottobre => 10,
            Mese::Novembre => 11,
            Mese::Dicembre => 12,
        }
    }

    pub fn to_string_pretty(&self) -> String {
        match self {
            Mese::Gennaio => "01",
            Mese::Febbraio => "02",
            Mese::Marzo => "03",
            Mese::Aprile => "04",
            Mese::Maggio => "05",
            Mese::Giugno => "06",
            Mese::Luglio => "07",
            Mese::Agosto => "08",
            Mese::Settembre => "09",
            Mese::Ottobre => "10",
            Mese::Novembre => "11",
            Mese::Dicembre => "12",
        }.to_string()
    }
}
