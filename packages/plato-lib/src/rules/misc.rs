use std::rc::Rc;

use crate::context::Context;
use crate::formula::PropWWF;
use crate::judgement::Judgement;

/// Variable introduction (assumption).
///
/// If `x ∈ Γ`, then derive `Γ ⊢ x`.
pub fn var_intro(ctx: &Context, x: Rc<PropWWF>) -> Option<Judgement> {
    if ctx.contains(&x) {
        return Some(Judgement::new(ctx.clone(), x));
    }
    None
}

/// Ex falso quodlibet (principle of explosion).
///
/// From `Γ ⊢ ⊥`, derive `Γ ⊢ p` for any `p`.
pub fn exfalso(j_bot: &Judgement, p: Rc<PropWWF>) -> Option<Judgement> {
    if *j_bot.prop == PropWWF::Bottom {
        return Some(Judgement::new(j_bot.ctx.clone(), p));
    }
    None
}
