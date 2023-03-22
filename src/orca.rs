// [[file:../ui-hack.note::b23949b9][b23949b9]]
#![allow(non_camel_case_types)]

use serde::*;
// b23949b9 ends here

// [[file:../ui-hack.note::4ac0cb23][4ac0cb23]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
pub struct Settings {
    calculation: CalculationType,
    method: Method,
    dispersion: Option<Dispersion>,
    basis_set: BasisSet,
    charge: isize,
    multiplicity: usize,
    scf_type: Option<SCFType>,
    scf_convergence: Option<SCFConvergence>,
    solvation: Option<Solvation>,
    dft_grid: Option<DftGrid>,
    symmetry: Option<Symmetry>,
}
// 4ac0cb23 ends here
