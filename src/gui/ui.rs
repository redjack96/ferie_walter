use crate::entity::anno::Anno;
use crate::entity::dipendenti::Dipendenti;
use crate::entity::mese::Mese;
use eframe::Frame;
use eframe::epaint::Color32;
use egui::{Button, ComboBox, Context, RichText};
use egui_custom::griglia::GrigliaInterattiva;
use egui_custom::griglia::cella::Cella;
use egui_custom::griglia::posizione::Posizione;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use crate::control::date::get_giorni_nel_mese;

#[derive(Default, Serialize, Deserialize)]
pub struct FerieWalter {
    #[serde(skip)]
    anno_selezionato: Anno,
    #[serde(skip)]
    mese_selezionato: Mese,
    dipendenti: Vec<Dipendenti>,
}

impl FerieWalter {
    pub fn new(dipendenti: Vec<Dipendenti>) -> Self {
        Self {
            anno_selezionato: Default::default(),
            mese_selezionato: Default::default(),
            dipendenti,
        }
    }
}

impl eframe::App for FerieWalter {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label(
                RichText::new("Gestione Ferie Lavori Pubblici / Manutenzione / Mobilità")
                    .size(22.0),
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ComboBox::from_id_salt("anno")
                .selected_text(RichText::new(self.anno_selezionato.to_string_pretty()).size(32.0))
                .show_ui(ui, |ui| {
                    for anno in Anno::iter() {
                        let anno_string = anno.to_string_pretty();
                        ui.selectable_value(&mut self.anno_selezionato, anno, anno_string);
                    }
                });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                for mese in Mese::iter() {
                    let button = if self.mese_selezionato == mese {
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

            ui.allocate_ui(ui.available_size(), |ui| {
                // recupero i giorni del mese selezionato, per l'anno selezionato, compresi casi eccezionali come gli anni bisestili
                let giorni_del_mese = get_giorni_nel_mese(self.anno_selezionato.to_i32(), self.mese_selezionato);
                dbg!(&self.mese_selezionato);
                dbg!(&self.anno_selezionato);
                let mut griglia = GrigliaInterattiva::new((2 + giorni_del_mese) as usize, vec![]);
                griglia = griglia.add_cella(Cella::from_testo("Nome"));
                for i in 1..=giorni_del_mese {
                    griglia = griglia.add_cella(Cella::from_testo(&i.to_string()));
                }
                griglia = griglia.add_cella(Cella::from_testo("Tot"));
                for dip in self.dipendenti.iter() {
                    griglia = griglia.add_cella_semplice(dip.nome.clone());
                    for i in 1..=giorni_del_mese {
                        griglia = griglia
                            .add_cella(Cella::from_testo("").on_click(|cella| {
                                if cella.get_testo(Posizione::Centro).is_empty() {
                                    cella.set_testo("X".into(), Posizione::Centro);
                                } else {
                                    cella.set_testo("".into(), Posizione::Centro);
                                }

                            }));
                    }

                    griglia = griglia.add_cella_semplice("0".into());
                }
                ui.add(griglia);
            });
        });
    }
}
