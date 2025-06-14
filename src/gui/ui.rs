/*
   file ui.rs
   del PROGETTO FERIE WALTER
*/

use crate::FILE_JSON; // Percorso file JSON per salvare/caricare dati
use crate::control::comandi::ComandoFerie; // Comandi personalizzati per gestione ferie
use crate::control::date::get_giorni_nel_mese; // Funzione per ottenere numero giorni in un mese
use crate::entity::anno::Anno; // Tipo Anno personalizzato
use crate::entity::dipendenti::Dipendente; // Tipo Dipendente
use crate::entity::mese::Mese; // Tipo Mese personalizzato
use eframe::Frame; // Frame della GUI
use eframe::epaint::Color32; // Colori per la UI
use egui::{Button, ComboBox, Context, RichText}; // Componenti UI base da egui
use egui_custom::griglia::GrigliaInterattiva; // Griglia interattiva personalizzata
use egui_custom::griglia::cella::Cella; // Cella della griglia
use egui_custom::griglia::posizione::Posizione; // Posizione testo nelle celle
use egui_custom::prelude::Common; // Utility comuni
use egui_custom::util::comandi::{Backend, Comandi}; // Gestione comandi e undo/redo
use serde::{Deserialize, Serialize}; // Serializzazione/deserializzazione JSON
use std::fs; // File system per lettura/scrittura file JSON
// AGGIUNTO PER GESTIONE DATE
use chrono::{NaiveDate, Datelike, Weekday}; // Manipolazione date e giorni settimana
use strum::IntoEnumIterator; // Iteratore per enum (Anno, Mese)

#[derive(Default, Serialize, Deserialize)]
pub struct FerieWalter {
   #[serde(skip)]
   anno_selezionato: Anno, // Anno corrente selezionato, non serializzato nel JSON
   #[serde(skip)]
   mese_selezionato: Mese, // Mese corrente selezionato, non serializzato nel JSON
   pub dipendenti: Vec<Dipendente>, // Lista di dipendenti gestiti
   #[serde(skip)]
   pub comandi: Common<Comandi<ComandoFerie>>, // Gestione comandi per undo/redo, non serializzata
}

impl eframe::App for FerieWalter {
   fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
      // Pannello superiore con titolo dell'app
      egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
         ui.label(
            RichText::new("Gestione Ferie Lavori Pubblici / Manutenzione / Mobilità")
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

            // Bottone per caricare dati da file JSON
            if ui.button(RichText::new("Carica").size(30.0)).clicked() {
               if let Some(contenuto) = fs::read_to_string(FILE_JSON).ok() {
                  if let Ok(ferie) = serde_json::from_str(&contenuto) {
                     *self = ferie;
                  }
               }
            }

            // Bottone per salvare dati su file JSON
            if ui.button(RichText::new("Salva").size(30.0)).clicked() {
               if let Ok(ferie_json) = serde_json::to_string_pretty(&self) {
                  fs::write(FILE_JSON, ferie_json).ok();
               }
            }
         });

         ui.add_space(10.0);

         // Barra orizzontale con pulsanti per ogni mese
         ui.horizontal(|ui| {
            for mese in Mese::iter() {
               let button = if self.mese_selezionato == mese {
                  // Evidenzia il mese selezionato con sfondo blu scuro
                  let testo = RichText::new(mese.as_ref()).strong().size(20.0);
                  Button::new(testo).fill(Color32::DARK_BLUE)
               } else {
                  let testo = RichText::new(mese.as_ref()).size(20.0);
                  Button::new(testo)
               };

               if ui.add(button).clicked() {
                  self.mese_selezionato = mese;
               }
            }
         });

         // Area di disegno della griglia principale per la gestione ferie
         ui.allocate_ui(ui.available_size(), |ui| {
            // Calcola numero giorni nel mese selezionato
            let giorni_del_mese =
               get_giorni_nel_mese(self.anno_selezionato.to_i32(), self.mese_selezionato);

            // Inizializza la griglia con righe pari a (giorni mese + intestazioni)
            let mut griglia = GrigliaInterattiva::new((2 + giorni_del_mese) as usize, vec![]);

            // Intestazione colonna "Nome" per i dipendenti
            griglia = griglia.add_cella(Cella::from_testo("Nome"));

            // Intestazioni con i numeri dei giorni del mese
            for i in 1..=giorni_del_mese {
               griglia = griglia.add_cella(Cella::from_testo(&i.to_string()));
            }

            // Intestazione colonna "Tot" per il totale ferie
            griglia = griglia.add_cella(Cella::from_testo("Tot"));

            // Riga di separazione vuota sotto intestazioni
            griglia = griglia.add_cella(Cella::from_testo(""));

            // Array di abbreviazioni dei giorni della settimana (lun-dom)
            let abbreviazioni = ["Lun", "Mar", "Mer", "Gio", "Ven", "Sab", "Dom"];

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

               griglia = griglia.add_cella(Cella::from_testo(testo_giorno));
            }

            // Riga di separazione vuota dopo giorni settimana
            griglia = griglia.add_cella(Cella::from_testo(""));

            // Per ogni dipendente aggiunge una riga con il nome e i giorni
            for dip in self.dipendenti.iter() {
               griglia = griglia.add_cella_semplice(&dip.nome);

               for giorno in 1..=giorni_del_mese {
                  let data_string = format!(
                     "{:04}-{:02}-{:02}",
                     self.anno_selezionato.to_i32(),
                     self.mese_selezionato.to_ordinal(),
                     giorno
                  );

                  // Parsing data per sicurezza
                  let giorno_settimana_result = NaiveDate::parse_from_str(&data_string, "%Y-%m-%d");

                  if giorno_settimana_result.is_err() {
                     eprintln!(
                        "ERRORE PARSING DATA '{}': {}",
                        data_string,
                        giorno_settimana_result.unwrap_err()
                     );
                     continue;
                  }

                  let giorno_settimana = giorno_settimana_result.unwrap().weekday();

                  if cfg!(debug_assertions) {
                     println!("DEBUG: Data: {}, Giorno settimana: {:?}", data_string, giorno_settimana);
                  }

                  // Testo cella "X" se il dipendente ha ferie in quella data, altrimenti vuoto
                  let testo_cella = if dip.ferie_in_questa_data(&data_string) {
                     "X"
                  } else {
                     ""
                  }.to_string();

                  // Clona variabili per uso nel closure on_click
                  let comandi = self.comandi.clone();
                  let dip_clone = dip.clone();
                  let data_string_clone = data_string.clone();

                  let mut cella = Cella::from_testo(testo_cella);

                  // MODIFICA PER GESTIONE SABATO E DOMENICA CLICCABILI:
                  if matches!(giorno_settimana, Weekday::Sat | Weekday::Sun) {
                     // Nei weekend la cella è cliccabile sempre
                     // Se c'è "X" la rimuovo al click, se è vuota non faccio nulla (rimane vuota)
                     cella = cella.on_click(move |cella| {
                        if !cella.get_testo(Posizione::Centro).is_empty() {
                           // Rimuovo la "X" se presente cliccando
                           comandi.read().add(ComandoFerie::RimuoviFerie(data_string_clone.clone(), dip_clone.clone()));
                        }
                        // Se vuota non aggiungo "X", cioè rimane vuota
                     });

                     // Facoltativo: evidenzio il weekend con uno sfondo rosso chiaro
                     cella = cella.colore_sfondo(Color32::LIGHT_RED.gamma_multiply(0.45));
                  } else {
                     // Giorni feriali: comportamento normale
                     // Cliccando su vuoto aggiungo "X", cliccando su "X" la rimuovo
                     cella = cella.on_click(move |cella| {
                        if cella.get_testo(Posizione::Centro).is_empty() {
                           comandi.read().add(ComandoFerie::AggiungiFerie(data_string_clone.clone(), dip_clone.clone()));
                        } else {
                           comandi.read().add(ComandoFerie::RimuoviFerie(data_string_clone.clone(), dip_clone.clone()));
                        }
                     });
                  }

                  // Aggiunge la cella così configurata alla griglia
                  griglia = griglia.add_cella(cella);
               }

               // Applica tutte le modifiche dei comandi accumulati (undo/redo)
               self.esegui_tutti(self.comandi.read().work.clone());

               // Conta e mostra il totale ferie del dipendente
               let conta_ferie = dip.ferie.read().iter().count();
               griglia = griglia.add_cella_semplice(&conta_ferie.to_string());
            }

            // Mostra la griglia aggiornata nell'interfaccia utente
            griglia.show(ui);
         });
      });
   }
}
