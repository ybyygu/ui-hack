// [[file:../ui-hack.note::b23949b9][b23949b9]]
#![allow(non_camel_case_types)]

use enum_iterator::Sequence;
use serde::*;
// b23949b9 ends here

// [[file:../ui-hack.note::a2f7f450][a2f7f450]]
mod ui;
// a2f7f450 ends here

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

impl Default for Symmetry {
    fn default() -> Self {
        Self::NoUseSym
    }
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Sequence)]
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
    // NOTE: there are bugs before version 5.0.4
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
    symmetry: Symmetry,
}

pub use ui::State;
// 8e8acf6e ends here
