use std::rc::Rc;

use crate::formula::PropWWF;
use crate::judgement::Judgement;

/// Negation introduction (¬-intro, reductio ad absurdum).
///
/// From `Γ, p ⊢ q` and `Δ, p ⊢ ¬q`, derive `Γ ∪ Δ ⊢ ¬p`.
pub fn neg_intro(np: Rc<PropWWF>, j: &Judgement, nj: &Judgement) -> Option<Judgement> {
    match j.ctx.decompose(np.as_ref()) {
        Some(ctx_np_p) => match nj.ctx.decompose(np.as_ref()) {
            Some(ctx_p_p) => match nj.prop.as_ref() {
                PropWWF::Neg(p) => {
                    if j.prop == *p {
                        return Some(Judgement::new(
                            ctx_p_p.union_with(&ctx_np_p),
                            Rc::new(PropWWF::Neg(np)),
                        ));
                    }
                    return None;
                }
                _ => return None,
            },
            None => return None,
        },
        None => None,
    }
}

/// Negation elimination (¬-elim).
///
/// From `Γ ⊢ ¬p`, derive `Γ ⊢ p → ⊥`.
pub fn neg_elim(nj: &Judgement) -> Option<Judgement> {
    match nj.prop.as_ref() {
        PropWWF::Neg(p) => Some(Judgement::new(
            nj.ctx.clone(),
            Rc::new(PropWWF::Imp(p.clone(), Rc::new(PropWWF::Bottom))),
        )),
        _ => None,
    }
}

/// Double negation elimination (¬¬-elim).
///
/// From `Γ ⊢ ¬¬p`, derive `Γ ⊢ p`.
/// This is a classical rule; it is not valid in intuitionistic logic.
pub fn double_neg_elim(j: &Judgement) -> Option<Judgement> {
    match j.prop.as_ref() {
        PropWWF::Neg(np) => match np.as_ref() {
            PropWWF::Neg(p) => Some(Judgement::new(j.ctx.clone(), p.clone())),
            _ => None,
        },
        _ => None,
    }
}
