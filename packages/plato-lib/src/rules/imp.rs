use std::rc::Rc;

use crate::formula::PropWWF;
use crate::judgement::Judgement;

/// Implication introduction (→-intro, deduction theorem).
///
/// From `Γ, p ⊢ q`, derive `Γ ⊢ p → q`.
/// Returns `None` if `p` is not in the context.
pub fn imp_intro(p_ant: Rc<PropWWF>, j: &Judgement) -> Option<Judgement> {
    match j.ctx.decompose(&p_ant) {
        Some(ctx_p) => Some(Judgement::new(
            ctx_p,
            Rc::new(PropWWF::Imp(p_ant, j.prop.clone())),
        )),
        None => None,
    }
}

/// Implication unpack (reverse of →-intro).
///
/// From `Γ ⊢ p → q`, derive `Γ, p ⊢ q`.
pub fn imp_into(j: &Judgement) -> Option<Judgement> {
    match j.prop.as_ref() {
        PropWWF::Imp(ant, consq) => {
            let mut new_ctx = j.ctx.clone();
            new_ctx.insert(ant.as_ref().clone());
            Some(Judgement::new(new_ctx, consq.clone()))
        }
        _ => None,
    }
}

/// Implication elimination (→-elim, modus ponens).
///
/// From `Γ ⊢ p → q` and `Δ ⊢ p`, derive `Γ ∪ Δ ⊢ q`.
pub fn imp_elim(j: &Judgement, je: &Judgement) -> Option<Judgement> {
    match j.prop.as_ref() {
        PropWWF::Imp(ant, consq) => {
            if *ant == je.prop {
                return Some(Judgement::new(
                    j.ctx.union_with(&je.ctx),
                    consq.clone(),
                ));
            }
        }
        _ => {}
    }
    None
}
