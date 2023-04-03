// [[file:../../ui-hack.note::697a91f7][697a91f7]]
use super::*;
use egui::Ui;

macro_rules! enum_value {
    ($v:expr) => {{
        serde_json::to_string($v).unwrap().trim_matches('"').to_string()
    }};
}
// 697a91f7 ends here

// [[file:../../ui-hack.note::0dd8f72b][0dd8f72b]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    templates: Vec<String>,
    current_template: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            templates: vec!["normal.jinja".to_owned(), "spectrum.jinja".to_owned()],
            current_template: "normal.jinja".to_owned(),
        }
    }
}
// 0dd8f72b ends here

// [[file:../../ui-hack.note::05df7b55][05df7b55]]
impl State {
    fn show_charge_and_multiplicity(&mut self, ui: &mut Ui) {
        ui.label("Charge:");
        ui.add(egui::DragValue::new(&mut self.settings.charge).speed(1.0));
        ui.end_row();
        ui.label("Multiplicity:");
        ui.add(egui::DragValue::new(&mut self.settings.multiplicity.0).speed(1.0));
    }
}
// 05df7b55 ends here

// [[file:../../ui-hack.note::cd0bd135][cd0bd135]]
impl State {
    fn show_basis(&mut self, ui: &mut Ui) {
        ui.label("Basis set:");
        let s = enum_value!(&self.settings.basis_set);
        egui::ComboBox::from_id_source("orca-basis")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<BasisSet>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.basis_set, t, s);
                }
            });
    }

    fn show_method(&mut self, ui: &mut Ui) {
        ui.label("Method:");
        let s = enum_value!(&self.settings.method);
        egui::ComboBox::from_id_source("orca-method")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<Method>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.method, t, s);
                }
            });
    }

    fn show_solvention(&mut self, ui: &mut Ui) {
        ui.label("Solvent:");
        let s = enum_value!(&self.settings.solvation);
        egui::ComboBox::from_id_source("orca-solvation")
            .width(400.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<Solvation>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.solvation, Some(t), s);
                }
            });
    }

    fn show_calculation(&mut self, ui: &mut Ui) {
        ui.label("Calculation:");
        let s = enum_value!(&self.settings.calculation);
        egui::ComboBox::from_id_source("orca-calc")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<CalculationType>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.calculation, t, s);
                }
            });
    }
}
// cd0bd135 ends here

// [[file:../../ui-hack.note::866bc573][866bc573]]
impl State {
    fn show_template_selection(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Render template:");
            egui::ComboBox::from_id_source("orca-template")
                .width(200.0)
                .selected_text(&self.current_template)
                .show_ui(ui, |ui| {
                    for t in self.templates.iter() {
                        ui.selectable_value(&mut self.current_template, t.to_string(), t);
                    }
                });
        });

        ui.separator();
        match self.current_template.as_str() {
            "normal.jinja" => {
                let s = include_str!("../../tests/files/orca.jinja");
                selectable_text(ui, &s);
            }
            "spectrum.jinja" => {
                selectable_text(ui, "spectrum");
            }
            _ => todo!(),
        }
        ui.separator();
    }
}

fn selectable_text(ui: &mut Ui, mut text: &str) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add(
            egui::TextEdit::multiline(&mut text)
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace.resolve(ui.style())),
        );
    });
}
// 866bc573 ends here

// [[file:../../ui-hack.note::7e56bc40][7e56bc40]]
impl State {
    pub fn show(&mut self, ui: &mut Ui) {
        egui::Grid::new("orca").num_columns(2).show(ui, |ui| {
            self.show_calculation(ui);
            ui.end_row();
            self.show_method(ui);
            ui.end_row();
            self.show_basis(ui);
            ui.end_row();
            self.show_solvention(ui);
            ui.end_row();
            self.show_charge_and_multiplicity(ui);
            ui.end_row();
        });
        ui.separator();
        self.show_template_selection(ui);
    }
}
// 7e56bc40 ends here
