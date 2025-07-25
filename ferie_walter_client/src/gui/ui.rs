/*
   file ui.rs
   del PROGETTO FERIE WALTER

   FASI OPERATIVE DEL PROGRAMMA:

   1. INIZIALIZZAZIONE UI:
      - Importazioni e struttura dati `FerieWalter`.

   2. COSTRUZIONE DELL’INTERFACCIA GRAFICA:
      - Pannello superiore (titolo app).
      - Selettore anno, pulsanti Carica/Salva.
      - Barra mesi.
      - Griglia ferie con intestazioni, giorni, dipendenti e celle cliccabili.

   3. LOGICA DI GESTIONE COMANDI:
      - Sistema di comando undo/redo per ferie.
      - Aggiunta/rimozione ferie con click.

   4. LETTURA/SCRITTURA DA/IN FILE:
      - Caricamento da JSON.
      - Salvataggio in JSON.
*/

// 1. INIZIALIZZAZIONE UI
#[cfg(not(target_arch = "wasm32"))]
use crate::FILE_JSON;
// Percorso file JSON per salvare/caricare dati

use crate::control::comandi::ComandoFerie;
// Comandi personalizzati per gestione ferie
use crate::control::date::get_giorni_nel_mese;
// Funzione per ottenere numero giorni in un mese
use crate::entity::anno::Anno;
// Tipo Anno personalizzato
use crate::entity::dipendenti::Dipendente;
// Tipo Dipendente
use crate::entity::mese::Mese;
// File system per lettura/scrittura file JSON
// Manipolazione date e giorni settimana
use chrono::{Datelike, NaiveDate, Weekday};
// Frame della GUI
use eframe::epaint::Color32;
// Tipo Mese personalizzato
use eframe::Frame;
// Colori per la UI
use egui::{Button, ComboBox, Context, RichText};
// Griglia interattiva personalizzata
use egui_custom::griglia::cella::Cella;
// Cella della griglia
use egui_custom::griglia::posizione::Posizione;
// Componenti UI base da egui
use egui_custom::griglia::GrigliaInterattiva;
// Posizione testo nelle celle
use egui_custom::prelude::{Commands, LoadingState, Shared};
use log::info;
// Utility comuni
use crate::control::load::{DatiFerie};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
// Iteratore per enum (Anno, Mese)

#[derive(Serialize, Deserialize)]
pub struct FerieWalter {
    #[serde(skip)]
    anno_selezionato: Anno, // Anno corrente selezionato, non serializzato nel JSON
    #[serde(skip)]
    mese_selezionato: Mese, // Mese corrente selezionato, non serializzato nel JSON
    pub dipendenti: Vec<Dipendente>, // Lista di dipendenti gestiti
    pub festivita: Vec<String>,
    #[serde(skip)]
    pub comandi: Shared<Commands>, // Gestione comandi per undo/redo, non serializzata
    #[serde(skip)]
    pub loading: Shared<LoadingState>,
    #[serde(skip)]
    loading_res: Shared<Option<DatiFerie>>,
}

impl Default for FerieWalter {
    fn default() -> Self {
        Self {
            anno_selezionato: Default::default(),
            mese_selezionato: Default::default(),
            dipendenti: vec![],
            festivita: vec!["2025-01-1".to_string()],
            comandi: Default::default(),
            loading: Shared::new(LoadingState::Loaded),
            loading_res: Shared::new(None),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FerieWalter {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let send_req = match gloo_net::http::Request::get("http://127.0.0.1:3000/getAll").send().await {
            Ok(res) => res,
            Err(err) => return Err(Box::new(err)),
        };

        let res = match send_req.text().await {
            Ok(res) => res,
            Err(err) => return Err(Box::new(err)),
        };

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DipendenteJson {
            pub nome: String,
            pub ferie: Vec<String>,
        }
        info!("Recupero i dipendenti dal server!");
        let dipendenti = match serde_json::from_str::<Vec<DipendenteJson>>(&res) {
            Ok(dip) => dip,
            Err(err) => return Err(Box::new(err)),
        };

        let dipendenti = dipendenti
            .into_iter()
            .map(|d| Dipendente {
                nome: d.nome,
                ferie: Shared::new(d.ferie),
            })
            .collect();

        // let festivita = // todo richiamare api getFestivita

        Ok(Self {
            anno_selezionato: Default::default(),
            mese_selezionato: Default::default(),
            dipendenti,
            festivita: vec![], // TODO: aggiungere api getFestivita
            comandi: Default::default(),
            loading: Shared::new(LoadingState::Loaded),
            loading_res: Shared::new(None),
        })
    }
}

impl eframe::App for FerieWalter {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // 2. COSTRUZIONE DELL’INTERFACCIA GRAFICA

        // Pannello superiore con titolo dell'app
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label(
                RichText::new(
                    "Gestione Ferie Lavori Pubblici / Manutenzione / Mobilità --> by W.R.",
                )
                .size(25.0),
            );
        });

        // Pannello centrale con tutto il contenuto principale
        egui::CentralPanel::default().show(ctx, |ui| {
            // Riga orizzontale con selettore anno e bottoni Carica/Salva
            ui.horizontal(|ui| {
                ComboBox::from_id_salt("anno")
                    .selected_text(
                        RichText::new(self.anno_selezionato.to_string_pretty()).size(32.0),
                    )
                    .show_ui(ui, |ui| {
                        // Elenco anni disponibili per la selezione
                        for anno in Anno::iter() {
                            let anno_string = anno.to_string_pretty();
                            ui.selectable_value(&mut self.anno_selezionato, anno, anno_string);
                        }
                    });

                // 4. LETTURA/SCRITTURA DA/IN FILE

                // Bottone per caricare dati da file JSON
                ui.add_enabled_ui(self.loading.copy() != LoadingState::Loading, |ui| {
                    if ui.button(RichText::new("Carica").size(30.0)).clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(contenuto) = std::fs::read_to_string(FILE_JSON).ok() {
                            if let Ok(ferie) = serde_json::from_str(&contenuto) {
                                *self = ferie;
                            }
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            info!("Cliccato Carica");
                            let loading_shared = self.loading.clone();
                            let result = self.loading_res.clone();
                            crate::control::load::start_async_load(loading_shared, result);
                        }
                    }

                    // Se il caricamento ha prodotto dei risultati, aggiorno self
                    if let Some(data) = self.loading_res.take() {
                        info!("Aggiorno i dati");
                        self.dipendenti = data.dip;
                        self.festivita = data.fes;
                        self.loading.write(LoadingState::Loaded);
                    }

                    // Bottone per salvare dati su file JSON
                    if ui.button(RichText::new("Salva").size(30.0)).clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Ok(ferie_json) = serde_json::to_string_pretty(&self) {
                            std::fs::write(FILE_JSON, ferie_json).ok();
                        }
                        // #[cfg(target_arch = "wasm32")]
                        // TODO
                    }
                });

                if self.loading.copy() == LoadingState::Loading {
                    ui.spinner();
                }
            });

            ui.add_space(10.0);

            // Barra orizzontale con pulsanti per ogni mese
            ui.horizontal(|ui| {
                for mese in Mese::iter() {
                    let button = if self.mese_selezionato == mese {
                        // @CREA@MESE@SELEZIONATO: Evidenzia il mese selezionato con sfondo blu scuro + ANNO CORRENTE
                        let testo = RichText::new(format!(
                            "{} {}",
                            mese.as_ref(),
                            self.anno_selezionato.to_string_pretty()
                        ))
                        .strong()
                        .size(20.0);
                        Button::new(testo).fill(Color32::DARK_BLUE)
                    } else {
                        //@CREA@MESI = tutti gli altri senza anno
                        let testo = RichText::new(mese.as_ref()).size(20.0);
                        Button::new(testo)
                    };

                    if ui.add(button).clicked() {
                        self.mese_selezionato = mese;
                    }
                }
            });

            ui.add_space(10.0);

            // Area di disegno della griglia principale per la gestione ferie

            // Calcola numero giorni nel mese selezionato
            let giorni_del_mese =
                get_giorni_nel_mese(self.anno_selezionato.to_i32(), self.mese_selezionato);

            // Inizializza la griglia con righe pari a (giorni mese + intestazioni)

            // Array di abbreviazioni dei giorni della settimana (lun-dom)
            let abbreviazioni = ["Lun", "Mar", "Mer", "Gio", "Ven", "Sab", "Dom"];

            let mut vett_header_giorni = vec![];
            // Intestazione con il giorno della settimana corrispondente a ciascun giorno
            for giorno in 1..=giorni_del_mese {
                let data_string = format!(
                    "{:04}-{:02}-{:02}",
                    self.anno_selezionato.to_i32(),
                    self.mese_selezionato.to_ordinal(),
                    giorno
                );

                // Parsing data e determinazione giorno della settimana
                let giorno_settimana = NaiveDate::parse_from_str(&data_string, "%Y-%m-%d")
                    .map(|d| d.weekday())
                    .unwrap_or(Weekday::Mon);

                let idx = giorno_settimana.num_days_from_monday() as usize;
                let testo_giorno = abbreviazioni[idx];
                vett_header_giorni.push(format!("{testo_giorno}\n{giorno}"));
            }
            //creo l'header del conteggio ferie MESE + ANNO
            vett_header_giorni.push("Tot\nMese".to_string());
            vett_header_giorni.push("Tot\nAnno".to_string());

            //crea un griglia che contiene l'header dei giorni
            let mut griglia = GrigliaInterattiva::new_empty_with_header(
                (2 + giorni_del_mese) as usize,
                60.0,
                vett_header_giorni,
            )
            .header_verticale(self.dipendenti.iter().map(|d| d.nome.clone()).collect());

            // Riga di separazione vuota dopo giorni settimana
            // griglia = griglia.add_cella(Cella::from_testo(""));

            // Per ogni dipendente aggiunge una riga con il nome e i giorni
            for dip in self.dipendenti.iter() {
                for giorno in 1..=giorni_del_mese {
                    // Creo una stringa data nel formato "YYYY-MM-DD" usando anno, mese e giorno corrente
                    let data_string = format!(
                        "{:04}-{:02}-{:02}",
                        self.anno_selezionato.to_i32(),
                        self.mese_selezionato.to_ordinal(),
                        giorno
                    );

                    // Provo a parsare la data dalla stringa nel formato specificato
                    let giorno_settimana_result =
                        NaiveDate::parse_from_str(&data_string, "%Y-%m-%d");

                    // Controllo se il parsing della data è andato a buon fine
                    if giorno_settimana_result.is_err() {
                        // In caso di errore, stampo un messaggio di errore e salto all'iterazione successiva
                        eprintln!(
                            "ERRORE PARSING DATA '{}': {}",
                            data_string,
                            giorno_settimana_result.unwrap_err()
                        );
                        continue;
                    }

                    // Se il parsing è corretto, ricavo il giorno della settimana dalla data
                    let giorno_settimana = giorno_settimana_result.unwrap().weekday();

                    // Preparo il testo da mostrare nella cella: se il dipendente è in ferie in questa data, metto "X", altrimenti stringa vuota
                    let testo_cella = if dip.ferie_in_questa_data(&data_string) {
                        "X"
                    } else {
                        ""
                    }
                    .to_string();

                    // 3. LOGICA DI GESTIONE COMANDI

                    // Clono le strutture necessarie per usarle all'interno delle closure (move)
                    let mut comandi = self.comandi.clone();
                    let dip_clone = dip.clone();
                    let data_string_clone = data_string.clone();

                    // Creo una nuova cella con il testo precedentemente definito
                    let mut cella = Cella::from_testo(testo_cella);

                    // MODIFICA PER GESTIONE SABATO E DOMENICA CLICCABILI:
                    // Se il giorno è Sabato o Domenica, la cella diventa cliccabile con un comportamento specifico
                    if matches!(giorno_settimana, Weekday::Sat | Weekday::Sun)
                        || self.festivita.contains(&data_string_clone)
                    {
                        // Imposto l'evento on_click sulla cella per rimuovere ferie solo se la cella non è vuota
                        cella = cella.on_click(move |cella| {
                            if !cella.get_testo(Posizione::Centro).is_empty() {
                                comandi.get_mut().add(ComandoFerie::RimuoviFerie(
                                    data_string_clone.clone(),
                                    dip_clone.clone(),
                                ));
                            }
                        });
                        // Imposto il colore di sfondo della cella a un rosso chiaro semitrasparente
                        cella = cella.colore_sfondo(Color32::LIGHT_RED.gamma_multiply(0.45));
                    } else {
                        // Per giorni feriali: on_click alterna tra aggiungere o rimuovere ferie a seconda che la cella sia vuota o no
                        cella = cella.on_click(move |cella| {
                            if cella.get_testo(Posizione::Centro).is_empty() {
                                comandi.get_mut().add(ComandoFerie::AggiungiFerie(
                                    data_string_clone.clone(),
                                    dip_clone.clone(),
                                ));
                            } else {
                                comandi.get_mut().add(ComandoFerie::RimuoviFerie(
                                    data_string_clone.clone(),
                                    dip_clone.clone(),
                                ));
                            }
                        });
                    }

                    // Aggiungo la cella creata alla griglia della UI
                    griglia = griglia.add_cella(cella);
                }

                self.comandi.get_mut().execute_all();

                let conta_ferie = dip.ferie.read().iter().count();
                //
                let conta_ferie_mese = dip
                    .ferie
                    .read()
                    .iter()
                    .filter(|data_ferie| {
                        //filtriamo le date del mese corrente
                        let data_ferie_result = NaiveDate::parse_from_str(&data_ferie, "%Y-%m-%d");
                        if let Ok(ferie) = data_ferie_result {
                            let mese_da_1 = ferie.month();
                            if mese_da_1 == self.mese_selezionato.to_ordinal() {
                                return true;
                            }
                        }
                        false
                    })
                    .count();
                griglia = griglia.add_cella_semplice(&conta_ferie_mese.to_string());
                griglia = griglia.add_cella_semplice(&conta_ferie.to_string());
            }

            // Mostra la griglia aggiornata nell'interfaccia utente
            griglia.show(ui);
        });
    }
}
