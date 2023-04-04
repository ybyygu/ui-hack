// [[file:../../ui-hack.note::697a91f7][697a91f7]]
use super::*;
use egui::Ui;
// 697a91f7 ends here

// [[file:../../ui-hack.note::0dd8f72b][0dd8f72b]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    templates: Vec<String>,
    current_template: String,
    rendered_input: String,
    input_template: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            templates: vec!["normal.jinja".to_owned(), "spectrum.jinja".to_owned(), "custom".to_owned()],
            current_template: "normal.jinja".to_owned(),
            input_template: String::new(),
            rendered_input: String::new(),
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

// [[file:../../ui-hack.note::f4738d12][f4738d12]]
macro_rules! enum_value {
    ($v:expr) => {{
        serde_json::to_string($v).unwrap().trim_matches('"').to_string()
    }};
}

macro_rules! show_combo_box_enum {
    ($id:literal, $ui:ident, $var:expr, $type:ty, $width:literal) => {
        let s = enum_value!(&$var);
        egui::ComboBox::from_id_source($id)
            .width($width)
            .selected_text(s)
            .show_ui($ui, |ui| {
                for t in enum_iterator::all::<$type>() {
                    let s = enum_value!(&t);
                    ui.selectable_value(&mut $var, t.into(), s);
                }
            });
    };
}
// f4738d12 ends here

// [[file:../../ui-hack.note::cd0bd135][cd0bd135]]
impl State {
    fn show_basis(&mut self, ui: &mut Ui) {
        ui.label("Basis set:");
        show_combo_box_enum!("orca-basis", ui, self.settings.basis_set, BasisSet, 200.0);
    }

    fn show_method(&mut self, ui: &mut Ui) {
        ui.label("Method:");
        show_combo_box_enum!("orca-method", ui, self.settings.method, Method, 200.0);
    }

    fn show_solvention(&mut self, ui: &mut Ui) {
        ui.label("Solvent:");
        show_combo_box_enum!("orca-solvation", ui, self.settings.solvation, Solvation, 300.0);
    }

    fn show_calculation(&mut self, ui: &mut Ui) {
        ui.label("Calculation:");
        show_combo_box_enum!("orca-calc", ui, self.settings.calculation, CalculationType, 200.0);
    }
}
// cd0bd135 ends here

// [[file:../../ui-hack.note::9e49412c][9e49412c]]
impl State {
    fn show_scf_type(&mut self, ui: &mut Ui) {
        ui.label("SCF Type:");
        show_combo_box_enum!("orca-scf-type", ui, self.settings.scf_type, SCFType, 200.0);
    }

    fn show_dft_grid(&mut self, ui: &mut Ui) {
        ui.label("DFT Grid:");
        show_combo_box_enum!("orca-dft-grid", ui, self.settings.dft_grid, DFTGrid, 200.0);
    }

    pub fn show_scf_convergence(&mut self, ui: &mut Ui) {
        ui.label("SCF Convergence:");
        show_combo_box_enum!("orca-scf-convergence", ui, self.settings.scf_convergence, SCFConvergence, 200.0);
    }
}
// 9e49412c ends here

// [[file:../../ui-hack.note::292360e9][292360e9]]
impl State {
    pub fn show_dispersion(&mut self, ui: &mut Ui) {
        ui.label("Dispersion");
        show_combo_box_enum!("orca-dispersion", ui, self.settings.dispersion, Dispersion, 200.0);
    }
}
// 292360e9 ends here

// [[file:../../ui-hack.note::61bab0f0][61bab0f0]]
impl State {
    pub fn show_ri(&mut self, ui: &mut Ui) {
        ui.label("RI Approximation");
        show_combo_box_enum!("orca-ri", ui, self.settings.ri, RIApproximation, 200.0);
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
        ui.horizontal(|ui| {
            // clipboard button
            let tooltip = "Click to copy generated input";
            if ui.button("📋").on_hover_text(tooltip).clicked() {
                self.rendered_input = render_template(&self.input_template, &self.settings).unwrap_or_default();
                ui.output_mut(|o| o.copied_text = self.rendered_input.clone());
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
                let mut s = include_str!("../../tests/files/orca.jinja");
                selectable_text(ui, &mut s, "template");
                self.input_template = s.to_string();
            }
            "spectrum.jinja" => {
                selectable_text(ui, &mut "todo!", "template");
            }
            "custom" => {
                editable_text(ui, &mut self.input_template, "template");
            }
            _ => todo!(),
        }

        selectable_text(ui, &mut self.rendered_input.as_str(), "rendered");
    }
}

fn editable_text(ui: &mut Ui, text: &mut String, label: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    });
}

// NOTE: read-only
fn selectable_text(ui: &mut Ui, mut text: &str, label: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    });
}
// 866bc573 ends here

// [[file:../../ui-hack.note::7e56bc40][7e56bc40]]
impl State {
    /// Show UI for all orca settings
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
                    self.show_ri(ui);
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
