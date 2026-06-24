use std::rc::Rc;

use crate::formula::PropWWF;
use crate::judgement::Judgement;

/// Disjunction introduction — left (∨-intro-L).
///
/// From `Γ ⊢ p`, derive `Γ ⊢ p ∨ q`.
pub fn disj_intro_lhs(j: &Judgement, pr: Rc<PropWWF>) -> Option<Judgement> {
    Some(Judgement::new(
        j.ctx.clone(),
        Rc::new(PropWWF::Disj(j.prop.clone(), pr)),
    ))
}

/// Disjunction introduction — right (∨-intro-R).
///
/// From `Γ ⊢ q`, derive `Γ ⊢ p ∨ q`.
pub fn disj_intro_rhs(j: &Judgement, pl: Rc<PropWWF>) -> Option<Judgement> {
    Some(Judgement::new(
        j.ctx.clone(),
        Rc::new(PropWWF::Disj(pl, j.prop.clone())),
    ))
}

/// Disjunction elimination (∨-elim, proof by cases).
///
/// From `Γ ⊢ p ∨ q`, `Δ₁, p ⊢ s`, and `Δ₂, q ⊢ s`,
/// derive `Γ ∪ Δ₁ ∪ Δ₂ ⊢ s`.
pub fn disj_elim(j: &Judgement, jl: &Judgement, jr: &Judgement) -> Option<Judgement> {
    match j.prop.as_ref() {
        PropWWF::Disj(lhs, rhs) => {
            if jl.ctx.contains(lhs.as_ref())
                && jr.ctx.contains(rhs.as_ref())
                && jl.prop == jr.prop
            {
                return Some(Judgement::new(
                    j.ctx.union_with(&jl.ctx.union_with(&jr.ctx)),
                    jl.prop.clone(),
                ));
            }
        }
        _ => {}
    }
    None
}
