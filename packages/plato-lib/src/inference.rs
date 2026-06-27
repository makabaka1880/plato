use std::rc::Rc;

use crate::context::Context;
use crate::formula::PropWWF;
use crate::judgement::Judgement;
use crate::rules;

// ── Rule identity ────────────────────────────────────────────────────

/// The identity of an inference rule, independent of how its parameters
/// are represented — line numbers in a [`Command`], or resolved premises
/// in a [`DeductionRule`].
///
/// This is the polymorphic key shared by both representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleKind {
    Assume,
    Var,
    ConjIntro,
    ConjElimL,
    ConjElimR,
    DisjIntroL,
    DisjIntroR,
    DisjElim,
    ImpIntro,
    ImpInto,
    ImpElim,
    NegIntro,
    NegElim,
    DNegElim,
    ExFalso,
    TopIntro,
    TopIntroCtx,
    Extend,
    Fix,
    ForallIntro,
    ForallElim,
    ExistsIntro,
    ExistsElim,
    BoxIntro,
    BoxElim,
    DiamondDef,
    DiamondDefRev,
    Subst,
}

impl RuleKind {
    /// Canonical s-expression command name, e.g. `"->-intro"`, `"and-elim-l"`.
    pub fn canonical_name(self) -> &'static str {
        match self {
            RuleKind::Assume => "assume",
            RuleKind::Var => "var",
            RuleKind::ConjIntro => "and-intro",
            RuleKind::ConjElimL => "and-elim-l",
            RuleKind::ConjElimR => "and-elim-r",
            RuleKind::DisjIntroL => "or-intro-l",
            RuleKind::DisjIntroR => "or-intro-r",
            RuleKind::DisjElim => "or-elim",
            RuleKind::ImpIntro => "->-intro",
            RuleKind::ImpInto => "->-into",
            RuleKind::ImpElim => "->-elim",
            RuleKind::NegIntro => "not-intro",
            RuleKind::NegElim => "not-elim",
            RuleKind::DNegElim => "dne",
            RuleKind::ExFalso => "ex-falso",
            RuleKind::TopIntro | RuleKind::TopIntroCtx => "top-intro",
            RuleKind::Extend => "extend",
            RuleKind::Fix => "fix",
            RuleKind::ForallIntro => "forall-intro",
            RuleKind::ForallElim => "forall-elim",
            RuleKind::ExistsIntro => "exists-intro",
            RuleKind::ExistsElim => "exists-elim",
            RuleKind::BoxIntro => "box-intro",
            RuleKind::BoxElim => "box-elim",
            RuleKind::DiamondDef => "diamond-def",
            RuleKind::DiamondDefRev => "diamond-def-rev",
            RuleKind::Subst => "subst",
        }
    }

    /// Number of line-number references this rule takes
    /// (useful for Fitch-proof display and validation).
    pub fn arity(self) -> usize {
        match self {
            RuleKind::Assume => 0,
            RuleKind::Var => 1,
            RuleKind::ConjIntro => 2,
            RuleKind::ConjElimL => 1,
            RuleKind::ConjElimR => 1,
            RuleKind::DisjIntroL => 1,
            RuleKind::DisjIntroR => 1,
            RuleKind::DisjElim => 3,
            RuleKind::ImpIntro => 1,
            RuleKind::ImpInto => 1,
            RuleKind::ImpElim => 2,
            RuleKind::NegIntro => 2,
            RuleKind::NegElim => 1,
            RuleKind::DNegElim => 1,
            RuleKind::ExFalso => 1,
            RuleKind::TopIntro => 0,
            RuleKind::TopIntroCtx => 1,
            RuleKind::Extend => 1,
            RuleKind::Fix => 0,
            RuleKind::ForallIntro => 1,
            RuleKind::ForallElim => 1,
            RuleKind::ExistsIntro => 1,
            RuleKind::ExistsElim => 2,
            RuleKind::BoxIntro => 1,
            RuleKind::BoxElim => 2,
            RuleKind::DiamondDef => 1,
            RuleKind::DiamondDefRev => 1,
            RuleKind::Subst => 1,
        }
    }
}

// ── Deduction rule ──────────────────────────────────────────────────

/// An inference rule of natural deduction.
///
/// Each variant carries the resolved premises needed to apply the rule.
/// Call [`deduce`](Self::deduce) to attempt to derive the conclusion.
#[derive(Debug, Clone)]
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
    /// Top introduction — axiom:
    /// `Γ ⊢ ⊤` for any context Γ.
    TopIntro(Context),
    /// Top introduction in step N's context:
    /// From step N's context, derive ⊤.
    TopIntroCtx(Rc<Judgement>),
    /// Weakening: `Γ ⊢ p ⇒ Γ, q ⊢ p`.
    Extend(Rc<Judgement>, Rc<PropWWF>),
    /// Introduce a term variable: `{x} ⊢ x`.
    Fix(Rc<PropWWF>),
    /// Universal generalisation: `Γ ⊢ φ` (with x fresh in Γ)
    /// ⇒ `Γ ⊢ ∀x.φ`.
    ForallIntro(String, Rc<Judgement>),
    /// Universal instantiation: `Γ ⊢ ∀x.φ ⇒ Γ ⊢ φ[x\t]`.
    ForallElim(Rc<Judgement>, String),
    /// Existential generalisation: `Γ ⊢ φ[x\t] ⇒ Γ ⊢ ∃x.φ`.
    ExistsIntro(Rc<Judgement>, String, String),
    /// Existential witness elimination:
    /// `Γ ⊢ ∃x.φ, Δ,φ[α\x] ⊢ ψ ⇒ Γ,Δ ⊢ ψ` (α fresh in Δ, ψ).
    ExistsElim(Rc<Judgement>, Rc<Judgement>, String),
    /// Necessitation (NEC): `∅ ⊢ A ⇒ ∅ ⊢ □A`.
    BoxIntro(Rc<Judgement>),
    /// K axiom: `∅ ⊢ □(A→B), ∅ ⊢ □A ⇒ ∅ ⊢ □B`.
    BoxElim(Rc<Judgement>, Rc<Judgement>),
    /// ◇-definition forward: `Γ ⊢ ◇A ⇒ Γ ⊢ ¬□¬A`.
    DiamondDef(Rc<Judgement>),
    /// ◇-definition reverse: `Γ ⊢ ¬□¬A ⇒ Γ ⊢ ◇A`.
    DiamondDefRev(Rc<Judgement>),
    /// Atomic substitution: replace atoms in a judgement's
    /// conclusion and context according to the given pairs.
    Subst(Rc<Judgement>, Vec<(String, Rc<PropWWF>)>),
}

impl DeductionRule {
    /// Returns the [`RuleKind`] identifying which inference rule this is.
    pub fn kind(&self) -> RuleKind {
        match self {
            Self::VarIntro(..) => RuleKind::Var,
            Self::ConjIntro(..) => RuleKind::ConjIntro,
            Self::ConjElimL(..) => RuleKind::ConjElimL,
            Self::ConjElimR(..) => RuleKind::ConjElimR,
            Self::DisjIntroL(..) => RuleKind::DisjIntroL,
            Self::DisjIntroR(..) => RuleKind::DisjIntroR,
            Self::DisjElim(..) => RuleKind::DisjElim,
            Self::ImpIntro(..) => RuleKind::ImpIntro,
            Self::ImpInto(..) => RuleKind::ImpInto,
            Self::ImpElim(..) => RuleKind::ImpElim,
            Self::NegIntro(..) => RuleKind::NegIntro,
            Self::NegElim(..) => RuleKind::NegElim,
            Self::DNegElim(..) => RuleKind::DNegElim,
            Self::ExFalso(..) => RuleKind::ExFalso,
            Self::TopIntro(..) => RuleKind::TopIntro,
            Self::TopIntroCtx(..) => RuleKind::TopIntroCtx,
            Self::Extend(..) => RuleKind::Extend,
            Self::Fix(..) => RuleKind::Fix,
            Self::ForallIntro(..) => RuleKind::ForallIntro,
            Self::ForallElim(..) => RuleKind::ForallElim,
            Self::ExistsIntro(..) => RuleKind::ExistsIntro,
            Self::ExistsElim(..) => RuleKind::ExistsElim,
            Self::BoxIntro(..) => RuleKind::BoxIntro,
            Self::BoxElim(..) => RuleKind::BoxElim,
            Self::DiamondDef(..) => RuleKind::DiamondDef,
            Self::DiamondDefRev(..) => RuleKind::DiamondDefRev,
            Self::Subst(..) => RuleKind::Subst,
        }
    }

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
            Self::TopIntro(ctx) => rules::misc::top_intro(&ctx),
            Self::TopIntroCtx(j) => rules::misc::top_intro(&j.ctx),
            Self::Extend(j, q) => rules::misc::extend(&j, q),
            Self::Fix(f) => {
                let mut ctx = Context::new();
                ctx.insert(f.as_ref().clone());
                rules::misc::var_intro(&ctx, f)
            }
            Self::ForallIntro(x, j) => rules::quant::forall_intro(&x, &j),
            Self::ForallElim(j, t) => rules::quant::forall_elim(&j, &t),
            Self::ExistsIntro(j, t, x) => rules::quant::exists_intro(&j, &t, &x),
            Self::ExistsElim(je, jw, x) => rules::quant::exists_elim(&je, &jw, &x),
            Self::BoxIntro(j) => rules::modal::box_intro(&j),
            Self::BoxElim(ji, jb) => rules::modal::box_elim(&ji, &jb),
            Self::DiamondDef(j) => rules::modal::diamond_def_fwd(&j),
            Self::DiamondDefRev(j) => rules::modal::diamond_def_rev(&j),
            Self::Subst(j, pairs) => rules::misc::subst(&j, &pairs),
        }
    }
}
