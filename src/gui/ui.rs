/*
    MAIN ORIGINALE
    PROGETTO FERIE WALTER

*/
use crate::FILE_JSON;
use crate::control::comandi::ComandoFerie;
use crate::control::date::get_giorni_nel_mese;
use crate::entity::anno::Anno;
use crate::entity::dipendenti::Dipendente;
use crate::entity::mese::Mese;
use eframe::Frame;
use eframe::epaint::Color32;
use egui::{Button, ComboBox, Context, RichText};
use egui_custom::griglia::GrigliaInterattiva;
use egui_custom::griglia::cella::Cella;
use egui_custom::griglia::posizione::Posizione;
use egui_custom::prelude::Common;
use egui_custom::util::comandi::{Backend, Comandi};
use serde::{Deserialize, Serialize};
use std::fs;
use egui::Key::C;
use strum::IntoEnumIterator;

#[derive(Default, Serialize, Deserialize)]
pub struct FerieWalter {
    #[serde(skip)]
    anno_selezionato: Anno,
    #[serde(skip)]
    mese_selezionato: Mese,
    pub dipendenti: Vec<Dipendente>,
    #[serde(skip)]
    pub comandi: Common<Comandi<ComandoFerie>>,
}

impl eframe::App for FerieWalter {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // @PANNELLO@TOP@BUTTON= inserisco la label
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ///@LABEL@TITOLO = IL TITOLO DELLA FORM
            ui.label(
                RichText::new("Gestione Ferie Lavori Pubblici / Manutenzione / Mobilità")
                    .size(25.0),
            );
        });

    // @PANNELLO@CENTRALE=  nel pannello centro inserisco la @COMBO@BOX @ANNO + BUTTON CARICA + BUTTON SALVA
      // ----------------------------------------------------------------------------------------------------------------------------------//
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        //CASELLA COMBINATA
                        ComboBox::from_id_salt("anno")
                            .selected_text(
                                RichText::new(self.anno_selezionato.to_string_pretty()).size(32.0),
                            )
                            .show_ui(ui, |ui| {
                                for anno in Anno::iter() {
                                    let anno_string = anno.to_string_pretty();
                                    //@Evento@anno@selezionato = recupero l'anno numerico e anno string
                                    ui.selectable_value(&mut self.anno_selezionato, anno, anno_string);
                                }
                            });

                        ///@BUTTON@CARICA = carica il file json
                        if ui.button(RichText::new("Carica").size(30.0)).clicked() {
                            if let Some(contenuto) = fs::read_to_string(FILE_JSON).ok() {
                                if let Ok(ferie) = serde_json::from_str(&contenuto) {
                                    // Aggiungo i dati letti
                                    *self = ferie;
                                }
                            }
                        };
                        // @BUTTON@SALVA = button salva dati nel  file json
                        if ui.button(RichText::new("Salva").size(30.0)).clicked() {
                            if let Ok(ferie_json) = serde_json::to_string_pretty(&self) {
                                // Quando clicco Salva, salvo i dati della scheda del giocatore (nella cartella di configurazione)
                                fs::write(FILE_JSON, ferie_json).ok();
                            }
                        };
                    });
                    // ----------------------------------------------------------------------------------------------------------------------------------//

            //AGGIUNGI SPAZIO
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                //Itera nell'enum MESE (gennaio , febbraio ecc..)
                //----------------------------------------------------------------------//
                    for mese in Mese::iter() {
                        let button = if self.mese_selezionato == mese {
                            let testo = RichText::new(mese.as_ref()).strong().size(20.0);
                            Button::new(testo).fill(Color32::DARK_BLUE)
                        } else {
                            let testo = RichText::new(mese.as_ref()).size(20.0);
                            Button::new(testo)
                        };

                        //SELEZIONO MESE
                        if ui.add(button).clicked() {
                            self.mese_selezionato = mese;
                        }
                    }

                //----------------------------------------------------------------------//
            });

            ui.allocate_ui(ui.available_size(), |ui| {
                // recupero i giorni del mese selezionato, per l'anno selezionato, compresi casi eccezionali come gli anni bisestili
                let giorni_del_mese =
                    get_giorni_nel_mese(self.anno_selezionato.to_i32(), self.mese_selezionato);

                let mut griglia = GrigliaInterattiva::new((2 + giorni_del_mese) as usize, vec![]);
               //
                //griglia = griglia.add_cella(Cella::from_testo("Nome"));
                griglia = griglia.add_cella(Cella::from_testo("Nome"));


                for i in 1..=giorni_del_mese {
                    griglia = griglia.add_cella(Cella::from_testo(&i.to_string()));
                }
                griglia = griglia.add_cella(Cella::from_testo("Tot"));

                for dip in self.dipendenti.iter() {
                    //todo: INSERIRE LARGHEZZA NOME DIPENDENTE ALMENO 30
                    griglia = griglia.add_cella_semplice(&dip.nome);


                    //@GIORNI@DEL@MESE = ciclo per la stampa dei giorni del mese
                    for giorno in 1..=giorni_del_mese {
                        //todo: a partire dalla stringa determina se è domenica, sabato o festa traducendola in NaiveDate
                        let data_string = format!(
                            "{}-{}-{giorno}",
                            self.anno_selezionato.to_i32(),
                            self.mese_selezionato.to_string_pretty()
                        );

                        //@IMPOSTA@X
                        let testo_cella = if dip.ferie_in_questa_data(&data_string) {
                            "X"
                        } else {
                            ""
                        }
                        .to_string();
                        let comandi = self.comandi.clone();
                        let dip_clone = dip.clone();
                        let data_string_clone = data_string.clone();

                        let mut cella =Cella::from_testo(testo_cella).on_click(
                            move |cella| {
                                if cella.get_testo(Posizione::Centro).is_empty() {
                                    comandi.read().add(ComandoFerie::AggiungiFerie(
                                        data_string_clone.clone(),
                                        dip_clone.clone(),
                                    ));
                                } else {
                                    comandi.read().add(ComandoFerie::RimuoviFerie(
                                        data_string_clone.clone(),
                                        dip_clone.clone(),
                                    ));
                                }
                            },
                        );

                        if &data_string == "2025-06-20" {
                            cella=cella.colore_sfondo(Color32::RED.gamma_multiply(0.2));
                        }
                        griglia = griglia.add_cella(cella);
                    }

                    self.esegui_tutti(self.comandi.read().work.clone());

                    let conta_ferie = dip.ferie.read().iter().count();

                    griglia = griglia.add_cella_semplice(&conta_ferie.to_string());
                }
                griglia.show(ui);
            });
        });
    }
}
