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
        ui.horizontal(|ui| {
            ui.label("Charge:");
            ui.add(egui::DragValue::new(&mut self.settings.charge).speed(1.0));
            ui.end_row();
            ui.label("Multiplicity:");
            ui.add(egui::DragValue::new(&mut self.settings.multiplicity.0).speed(1.0));
        });
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

// [[file:../../ui-hack.note::9e49412c][9e49412c]]
impl State {
    fn show_scf_type(&mut self, ui: &mut Ui) {
        ui.label("SCF Type");
        let s = enum_value!(&self.settings.scf_type);
        egui::ComboBox::from_id_source("orca-scf-type")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<SCFType>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.scf_type, Some(t), s);
                }
            });
    }

    fn show_dft_grid(&mut self, ui: &mut Ui) {
        ui.label("DFT Grid");
        let s = enum_value!(&self.settings.dft_grid);
        egui::ComboBox::from_id_source("orca-dft-grid")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<DFTGrid>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.dft_grid, Some(t), s);
                }
            });
    }

    pub fn show_scf_convergence(&mut self, ui: &mut Ui) {
        ui.label("SCF Convergence");
        let s = enum_value!(&self.settings.scf_convergence);
        egui::ComboBox::from_id_source("orca-scf-convergence")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<SCFConvergence>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.scf_convergence, Some(t), s);
                }
            });
    }
}
// 9e49412c ends here

// [[file:../../ui-hack.note::292360e9][292360e9]]
impl State {
    pub fn show_dispersion(&mut self, ui: &mut Ui) {
        ui.label("Dispersion");
        let s = enum_value!(&self.settings.dispersion);
        egui::ComboBox::from_id_source("orca-dispersion")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<Dispersion>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.dispersion, t, s);
                }
            });
    }
}
// 292360e9 ends here

// [[file:../../ui-hack.note::61bab0f0][61bab0f0]]
impl State {
    pub fn show_ri(&mut self, ui: &mut Ui) {
        ui.label("RI Approximation");
        let s = enum_value!(&self.settings.ri);
        egui::ComboBox::from_id_source("orca-ri")
            .width(200.0)
            .selected_text(s)
            .show_ui(ui, |ui| {
                for t in enum_iterator::all::<RIApproximation>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut self.settings.ri, t, s);
                }
            });
    }
}
// 61bab0f0 ends here

// [[file:../../ui-hack.note::b4764c7b][b4764c7b]]
impl State {
    pub fn show_symmetry(&mut self, ui: &mut Ui) {
        let radio = &mut self.settings.symmetry;
        ui.horizontal(|ui| {
            ui.selectable_value(radio, Symmetry::NoUseSym, "NoUseSym");
            ui.selectable_value(radio, Symmetry::UseSym, "UseSym");
        });
    }
}
// b4764c7b ends here

// [[file:../../ui-hack.note::866bc573][866bc573]]
fn render_template<S: Serialize>(template: &str, settings: S) -> Option<String> {
    use minijinja::{context, Environment};

    let mut env = Environment::new();
    env.add_template("hello", template).ok()?;
    let tmpl = env.get_template("hello").ok()?;
    let s = tmpl.render(settings).ok()?;
    Some(s)
}

impl State {
    fn show_template_selection(&mut self, ui: &mut Ui) {
        let s = include_str!("../../tests/files/orca.jinja");
        ui.horizontal(|ui| {
            // clipboard button
            let tooltip = "Click to copy generated input";
            if ui.button("📋").on_hover_text(tooltip).clicked() {
                let input = render_template(s, &self.settings).unwrap_or_default();
                ui.output_mut(|o| o.copied_text = input);
            }

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
        egui::Grid::new("orca_grid_core").num_columns(2).show(ui, |ui| {
            self.show_method(ui);
            ui.end_row();
            self.show_calculation(ui);
            ui.end_row();
            self.show_basis(ui);
            ui.end_row();
        });
        self.show_charge_and_multiplicity(ui);
        ui.collapsing("SCF", |ui| {
            egui::Grid::new("orca_grid_misc")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.show_scf_type(ui);
                    ui.end_row();
                    self.show_scf_convergence(ui);
                    ui.end_row();
                    self.show_dft_grid(ui);
                    ui.end_row();
                });
        });
        ui.collapsing("Misc", |ui| {
            egui::Grid::new("orca_grid_misc")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    // ui.label("RI approximation:");
                    self.show_ri(ui);
                    // ui.add(egui::DragValue::new(&mut self.settings.charge).speed(1.0));
                    ui.end_row();
                    ui.label("Symmetry:");
                    self.show_symmetry(ui);
                    ui.end_row();
                    self.show_dispersion(ui);
                    ui.end_row();
                    self.show_solvention(ui);
                    ui.end_row();
                });
        });

        ui.separator();
        self.show_template_selection(ui);
    }
}
// 7e56bc40 ends here
