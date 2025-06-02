use chrono::Datelike;
use chrono::NaiveDate;
use crate::entity::mese::Mese;

/// Restituisce il numero di giorni in un mese specifico.
///
/// # Argomenti
/// * `year` - L'anno (es. 2025)
/// * `month` - Il mese (1 = Gennaio, 12 = Dicembre)
///
/// # Ritorna
/// * `Some(u32)` con il numero di giorni, oppure `None` se i parametri non sono validi.
pub fn get_giorni_nel_mese(year: i32, mese: Mese) -> u32 {
    let mese_corrente = mese.to_index();

    // Crea la data del primo giorno del mese successivo
    let data_prossimo_mese = if mese_corrente == 11 {
        NaiveDate::from_ymd_opt(year + 1, 0, 1)
    } else {
        NaiveDate::from_ymd_opt(year, mese_corrente + 1, 1)
    };

    // Prendi il giorno precedente, cioè l'ultimo giorno del mese corrente
    data_prossimo_mese
        .and_then(|data| data.pred_opt())
        .map(|data_ultimo_giorno| data_ultimo_giorno.day())
        .unwrap_or(30) // fallback se qualcosa va storto
}