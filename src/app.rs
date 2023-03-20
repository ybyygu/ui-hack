// [[file:../ui-hack.note::f7df0467][f7df0467]]
use serde::*;

use egui::Ui;
// f7df0467 ends here

// [[file:../ui-hack.note::7586f426][7586f426]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum InputTemplate {
    Gaussian,
    Orca,
    Vasp,
}
// 7586f426 ends here

// [[file:../ui-hack.note::9a0316a5][9a0316a5]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum Enum {
    First,
    Second,
    Third,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // current input generator
    template: InputTemplate,

    title: String,

    radio: Enum,

    charge: isize,

    multiplicity: usize,

    // functional, basis set etc.
    theory: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            value: 2.7,
            radio: Enum::First,
            title: String::new(),
            charge: 0,
            multiplicity: 1,
            theory: String::new(),
            template: InputTemplate::Orca,
        }
    }
}
// 9a0316a5 ends here

// [[file:../ui-hack.note::e67677fe][e67677fe]]
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
// e67677fe ends here

// [[file:../ui-hack.note::fbd820f2][fbd820f2]]
fn ui_side_panel(ui: &mut Ui, state: &mut TemplateApp) {
    ui.heading("Input templates");

    ui.vertical(|ui| {
        ui.selectable_value(&mut state.template, InputTemplate::Vasp, "VASP");
        ui.selectable_value(&mut state.template, InputTemplate::Orca, "ORCA");
        ui.selectable_value(&mut state.template, InputTemplate::Gaussian, "Gaussian");
    });

    ui.separator();

    // show egui logo
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        });
    });
}
// fbd820f2 ends here

// [[file:../ui-hack.note::d0f130e0][d0f130e0]]
fn ui_central_panel(ui: &mut Ui, state: &mut TemplateApp) {
    ui.heading(format!("{:?} input generator", state.template));
    match state.template {
        InputTemplate::Orca => {
            ui.heading("ORCA input");

            // 格线对齐
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Title:");
                    ui.add(egui::TextEdit::singleline(&mut state.title).hint_text("Geom optim"));
                    ui.end_row();
                    ui.label("Calculation Type:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", &mut state.radio))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.radio, Enum::First, "Single Point");
                            ui.selectable_value(&mut state.radio, Enum::Second, "Geometry Optimization");
                            ui.selectable_value(&mut state.radio, Enum::Third, "Frequencies");
                        });
                    ui.end_row();
                    ui.label("Charge:");
                    ui.add(egui::DragValue::new(&mut state.charge).speed(1.0));
                    ui.end_row();
                    ui.label("Multiplicity:");
                    ui.add(egui::DragValue::new(&mut state.multiplicity).speed(1.0));
                    ui.end_row();
                    ui.label("Theory:");
                    ui.add(egui::TextEdit::singleline(&mut state.theory).hint_text("B3LYP Def2-SVP"));
                    ui.end_row();
                });

            ui.separator();
            let tooltip = "Click to copy generated input in json";
            if ui.button("📋").on_hover_text(tooltip).clicked() {
                ui.output_mut(|o| o.copied_text = "some_text".to_string());
            }
            ui.separator();
            ui.collapsing("Misc", |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Charge:");
                        ui.add(egui::DragValue::new(&mut state.charge).speed(1.0));
                        ui.end_row();
                        ui.label("RI approximation:");
                        ui.add(egui::DragValue::new(&mut state.charge).speed(1.0));
                        ui.end_row();
                    });
            });
        }
        _ => {
            ui.separator();
            ui.label("Under Construction!");
        }
    }
}
// d0f130e0 ends here

// [[file:../ui-hack.note::5a6d6884][5a6d6884]]
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui_side_panel(ui, self);

        });

        // The central panel the region left after adding TopPanel's and SidePanel's
        egui::CentralPanel::default().show(ctx, |ui| {
            ui_central_panel(ui, self);
        });
    }
}
// 5a6d6884 ends here
