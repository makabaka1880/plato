// Created by Sean L. on Jun. 27.
// Last Updated by Sean L. on Jun. 28.
//
// plato-lib
// src/fitch/proof.rs
//
// Makabaka1880, 2026. All rights reserved.

use crate::formula::PropWWF;
use crate::inference::RuleKind;

/// A Fitch-style natural deduction proof.
///
/// Lines are indexed 1-based, matching the convention used by
/// [`Command`](crate::parser::command::Command) line-number references.
pub struct FitchProof {
    pub lines: Vec<FitchLine>,
}

/// One line in a Fitch proof.
pub enum FitchLine {
    /// A derivation step justified by an inference rule.
    Derivation {
        /// The formula derived at this line.
        formula: PropWWF,
        /// Which inference rule was applied.
        kind: RuleKind,
        /// 1-indexed line numbers this derivation cites as premises.
        line_refs: Vec<usize>,
        /// Auxiliary parameters beyond line references
        /// (e.g. discharged formula, variable name, substitution pairs).
        /// Each entry is `(key, value)` where `value` is a display string
        /// (LaTeX for formulae, raw text for variable names).
        params: Vec<(String, String)>,
    },
    /// A subproof opened by an assumption, containing its own proof lines.
    Subproof {
        /// The assumption formula.
        assumption: PropWWF,
        /// The body of the subproof.
        body: FitchProof,
    },
}
