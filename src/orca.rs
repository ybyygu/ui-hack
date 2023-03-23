// [[file:../ui-hack.note::b23949b9][b23949b9]]
#![allow(non_camel_case_types)]

use serde::*;
use enum_iterator::Sequence;
// b23949b9 ends here

// [[file:../ui-hack.note::4ac0cb23][4ac0cb23]]
#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum CalculationType {
    #[serde(rename = "EnGrad")]
    SinglePoint,
    #[serde(rename = "Opt")]
    GeometryOptimization,
    #[serde(rename = "Freq")]
    Frequency,
    #[serde(rename = "MD")]
    MolecularDynamics,
}

impl Default for CalculationType {
    fn default() -> Self {
        Self::SinglePoint
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum DFTGrid {
    DefGrid1,
    DefGrid2,
    DefGrid3,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum Symmetry {
    NoUseSym,
    UseSym,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum Method {
    PBE,
    PB86,
    TPSS,
    B3LYP,
    #[serde(rename = "CAM-B3LYP")]
    CAMB3LYP,
    X3LYP,
    PBE0,
    M06,
    M062X,
    wB97X,
    #[serde(rename = "wB97X-D3")]
    wB97XD3,
    /// Perdew’s SCAN functional
    #[serde(rename = "SCANfunc")]
    SCAN,
    MP2,
    CCSD,
}

impl Default for Method {
    fn default() -> Self {
        Self::B3LYP
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum BasisSet {
    #[serde(rename = "def2-SVP")]
    Def2Svp,
    #[serde(rename = "def2-TZVP")]
    Def2Tzvp,
    #[serde(rename = "def2-TZVPP")]
    Def2Tzvpp,
    #[serde(rename = "def2-QZVPP")]
    Def2Qzvpp,
}

impl Default for BasisSet {
    fn default() -> Self {
        Self::Def2Svp
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum SCFType {
    /// spin unrestricted SCF
    #[serde(rename = "UKS")]
    UnRestricted,

    /// closed-shell SCF
    #[serde(rename = "KS")]
    Restricted,

    /// open-shell spin restricted SCF
    #[serde(rename = "ROKS")]
    RestrictedOpen,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum SCFConvergence {
    // Energy change 1.0e-09 au
    VeryTightSCF,
    // Energy change 1.0e-08 au. Default for geometry optimizations.
    TightSCF,
    // Energy change 1.0e-06 au. Default for single-point calculations.
    NormalSCF,
    // Energy change 1.0e-05 au
    LooseSCF,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
enum Solvent {
    Water,
    Acetone,
    Acetonitrile,
    Ammonia,
    Benzene,
    CCl4,
    CH2Cl2,
    Chloroform,
    Cyclohexane,
    DMF,
    DMSO,
    Ethanol,
    Hexane,
    Methanol,
    Octanol,
    Pyridine,
    THF,
    Toluene,
}

impl Default for Solvent {
    fn default() -> Self {
        Self::Water
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
#[serde(tag = "model", content = "solvent")]
enum Solvation {
    SMD(Solvent),
    CPCM(Solvent),
}

/// DFT Calculations with Atom-pairwise Dispersion Correction
#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum Dispersion {
    D2,
    D3Bj,
    D3Zero,
    D4,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Multiplicity(usize);

impl Default for Multiplicity {
    fn default() -> Self {
        Self(1)
    }
}
// 4ac0cb23 ends here

// [[file:../ui-hack.note::697a91f7][697a91f7]]
mod ui {
    use super::*;
    use egui::Ui;
    macro_rules! enum_value {
        ($v:expr) => {{
            serde_json::to_string($v).unwrap().trim_matches('"').to_string()
        }};
    }

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
}
// 697a91f7 ends here

// [[file:../ui-hack.note::8e8acf6e][8e8acf6e]]
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    calculation: CalculationType,
    method: Method,
    basis_set: BasisSet,
    charge: isize,
    multiplicity: Multiplicity,
    dispersion: Option<Dispersion>,
    scf_type: Option<SCFType>,
    scf_convergence: Option<SCFConvergence>,
    solvation: Option<Solvation>,
    dft_grid: Option<DFTGrid>,
    symmetry: Option<Symmetry>,
}
// 8e8acf6e ends here
