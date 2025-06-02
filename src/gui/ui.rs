use crate::entity::anno::Anno;
use crate::entity::dipendenti::Dipendenti;
use crate::entity::mese::Mese;
use eframe::Frame;
use eframe::epaint::Color32;
use egui::{Button, ComboBox, Context, RichText};
use egui_custom::griglia::GrigliaInterattiva;
use egui_custom::griglia::cella::Cella;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct FerieWalter {
    anno_selezionato: Anno,
    mese_selezionato: Mese,
    dipendenti: Vec<Dipendenti>,
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
                let mut griglia = GrigliaInterattiva::new(33, vec![]);
                griglia = griglia.add_cella(Cella::from_testo("Nome"));

                ui.add(griglia);
            });
        });
    }
}
