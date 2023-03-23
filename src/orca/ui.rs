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
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    templates: Vec<String>,
}
// 0dd8f72b ends here

// [[file:../../ui-hack.note::2e7b6c8e][2e7b6c8e]]
impl Settings {
    fn show_basis(&mut self, ui: &mut Ui) {
        ui.label("Basis set:");
        let s = enum_value!(&self.basis_set);
        egui::ComboBox::from_id_source("orca-basis")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<BasisSet>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.basis_set, t, s);
                }
            });
    }

    fn show_method(&mut self, ui: &mut Ui) {
        ui.label("Method:");
        let s = enum_value!(&self.method);
        egui::ComboBox::from_id_source("orca-method")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<Method>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.method, t, s);
                }
            });
    }

    fn show_solvention(&mut self, ui: &mut Ui) {
        ui.label("Solvent:");
        let s = enum_value!(&self.solvation);
        egui::ComboBox::from_id_source("orca-solvation")
            .width(400.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<Solvation>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.solvation, Some(t), s);
                }
            });
    }

    fn show_calculation(&mut self, ui: &mut Ui) {
        ui.label("Calculation:");
        let s = enum_value!(&self.calculation);
        egui::ComboBox::from_id_source("orca-calc")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<CalculationType>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.calculation, t, s);
                }
            });
    }
}
// 2e7b6c8e ends here

// [[file:../../ui-hack.note::*template][template:1]]
impl State {
    fn show_template_selection(&mut self, ui: &mut Ui) {

    }
}
// template:1 ends here

// [[file:../../ui-hack.note::7e56bc40][7e56bc40]]
impl Settings {
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
        });
    }
}

impl State {
    pub fn show(&mut self, ui: &mut Ui) {
        egui::Grid::new("orca").num_columns(2).show(ui, |ui| {
            ui.label("test");
            ui.end_row();
        });
    }
}
// 7e56bc40 ends here
