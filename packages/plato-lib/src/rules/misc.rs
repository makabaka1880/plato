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

/// Top introduction — axiom.
///
/// ⊤ (truth) is always provable under any context: `Γ ⊢ ⊤`.
pub fn top_intro(ctx: &Context) -> Option<Judgement> {
    Some(Judgement::new(ctx.clone(), Rc::new(PropWWF::Top)))
}

/// Substitution — the uniform replacement of atoms in a proven judgement.
///
/// From `Γ ⊢ p` and a list of `(atom, replacement)` pairs, derive
/// `Γ[subs] ⊢ p[subs]`.  This is a structural rule: a proof that works
/// for atoms also works when those atoms are replaced by arbitrary
/// formulas, as long as the replacement is uniform.
pub fn subst(j: &Judgement, subs: &[(String, Rc<PropWWF>)]) -> Option<Judgement> {
    Some(Judgement::new(
        j.ctx.substitute(subs),
        j.prop.substitute(subs),
    ))
}
