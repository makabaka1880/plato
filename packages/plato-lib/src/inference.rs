use std::rc::Rc;

use crate::context::Context;
use crate::formula::PropWWF;
use crate::judgement::Judgement;
use crate::rules;

/// An inference rule of natural deduction.
///
/// Each variant carries the premises needed to apply the rule.
/// Call [`deduce`](Self::deduce) to attempt to derive the conclusion.
pub enum DeductionRule {
    /// Variable introduction: `x ∈ Γ ⇒ Γ ⊢ x`.
    VarIntro(Context, Rc<PropWWF>),
    /// Conjunction introduction: `Γ ⊢ p, Δ ⊢ q ⇒ Γ,Δ ⊢ p∧q`.
    ConjIntro(Rc<Judgement>, Rc<Judgement>),
    /// Conjunction elimination left: `Γ ⊢ p∧q ⇒ Γ ⊢ p`.
    ConjElimL(Rc<Judgement>),
    /// Conjunction elimination right: `Γ ⊢ p∧q ⇒ Γ ⊢ q`.
    ConjElimR(Rc<Judgement>),
    /// Disjunction introduction left: `Γ ⊢ p ⇒ Γ ⊢ p∨q`.
    DisjIntroL(Rc<Judgement>, Rc<PropWWF>),
    /// Disjunction introduction right: `Γ ⊢ q ⇒ Γ ⊢ p∨q`.
    DisjIntroR(Rc<Judgement>, Rc<PropWWF>),
    /// Disjunction elimination (proof by cases):
    /// `Γ ⊢ p∨q, Δ₁,p ⊢ s, Δ₂,q ⊢ s ⇒ Γ,Δ₁,Δ₂ ⊢ s`.
    DisjElim(Rc<Judgement>, Rc<Judgement>, Rc<Judgement>),
    /// Implication introduction (deduction theorem):
    /// `Γ,p ⊢ q ⇒ Γ ⊢ p→q`.
    ImpIntro(Rc<PropWWF>, Rc<Judgement>),
    /// Implication unpack: `Γ ⊢ p→q ⇒ Γ,p ⊢ q`.
    ImpInto(Rc<Judgement>),
    /// Implication elimination (modus ponens):
    /// `Γ ⊢ p→q, Δ ⊢ p ⇒ Γ,Δ ⊢ q`.
    ImpElim(Rc<Judgement>, Rc<Judgement>),
    /// Negation introduction (reductio ad absurdum):
    /// `Γ,p ⊢ q, Δ,p ⊢ ¬q ⇒ Γ,Δ ⊢ ¬p`.
    NegIntro(Rc<PropWWF>, Rc<Judgement>, Rc<Judgement>),
    /// Negation elimination: `Γ ⊢ ¬p ⇒ Γ ⊢ p→⊥`.
    NegElim(Rc<Judgement>),
    /// Double negation elimination: `Γ ⊢ ¬¬p ⇒ Γ ⊢ p`.
    DNegElim(Rc<Judgement>),
    /// Ex falso quodlibet (principle of explosion):
    /// `Γ ⊢ ⊥ ⇒ Γ ⊢ p` for any `p`.
    ExFalso(Rc<Judgement>, Rc<PropWWF>),
}

impl DeductionRule {
    /// Attempts to apply this rule. Returns `Some(judgement)` on success,
    /// or `None` if the rule's preconditions are not met.
    pub fn deduce(self) -> Option<Judgement> {
        match self {
            Self::VarIntro(ctx, x) => rules::misc::var_intro(&ctx, x),
            Self::ConjIntro(jl, jr) => rules::conj::conj_intro(&jl, &jr),
            Self::ConjElimL(j) => rules::conj::conj_elim_lhs(&j),
            Self::ConjElimR(j) => rules::conj::conj_elim_rhs(&j),
            Self::DisjIntroL(j, pr) => rules::disj::disj_intro_lhs(&j, pr),
            Self::DisjIntroR(j, pl) => rules::disj::disj_intro_rhs(&j, pl),
            Self::DisjElim(j, jl, jr) => rules::disj::disj_elim(&j, &jl, &jr),
            Self::ImpIntro(p_ant, j) => rules::imp::imp_intro(p_ant, &j),
            Self::ImpInto(j) => rules::imp::imp_into(&j),
            Self::ImpElim(j, je) => rules::imp::imp_elim(&j, &je),
            Self::NegIntro(np, j, nj) => rules::neg::neg_intro(np, &j, &nj),
            Self::NegElim(nj) => rules::neg::neg_elim(&nj),
            Self::DNegElim(j) => rules::neg::double_neg_elim(&j),
            Self::ExFalso(j_bot, p) => rules::misc::exfalso(&j_bot, p),
        }
    }
}
