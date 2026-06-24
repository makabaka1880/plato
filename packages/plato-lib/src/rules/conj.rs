use std::rc::Rc;

use crate::formula::PropWWF;
use crate::judgement::Judgement;

/// Conjunction introduction (∧-intro).
///
/// From `Γ ⊢ p` and `Δ ⊢ q`, derive `Γ ∪ Δ ⊢ p ∧ q`.
pub fn conj_intro(jl: &Judgement, jr: &Judgement) -> Option<Judgement> {
    Some(Judgement::new(
        jl.ctx.union_with(&jr.ctx),
        Rc::new(PropWWF::Conj(jl.prop.clone(), jr.prop.clone())),
    ))
}

/// Conjunction elimination — left (∧-elim-L).
///
/// From `Γ ⊢ p ∧ q`, derive `Γ ⊢ p`.
pub fn conj_elim_lhs(jc: &Judgement) -> Option<Judgement> {
    let ctx = jc.ctx.clone();
    match jc.prop.as_ref() {
        PropWWF::Conj(jl, _) => {
            return Some(Judgement::new(ctx, jl.clone()));
        }
        _ => {}
    }
    None
}

/// Conjunction elimination — right (∧-elim-R).
///
/// From `Γ ⊢ p ∧ q`, derive `Γ ⊢ q`.
pub fn conj_elim_rhs(jc: &Judgement) -> Option<Judgement> {
    let ctx = jc.ctx.clone();
    match jc.prop.as_ref() {
        PropWWF::Conj(_, jr) => {
            return Some(Judgement::new(ctx, jr.clone()));
        }
        _ => {}
    }
    None
}
