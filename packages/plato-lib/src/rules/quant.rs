use std::rc::Rc;

use crate::formula::{self, PropWWF};
use crate::judgement::Judgement;

/// ∀-intro (universal generalisation).
///
/// From `Γ, x ⊢ φ` where `x` is a term variable not free in `Γ`,
/// derive `Γ ⊢ ∀x. φ`.
pub fn forall_intro(x: &str, j: &Judgement) -> Option<Judgement> {
    if !formula::is_term_var(x) {
        return None;
    }
    // The context must contain Atom(x).
    let atom_x = PropWWF::Atom(x.to_string());
    let ctx_without_x = j.ctx.decompose(&atom_x)?;
    // No OTHER formula in the context may have x free.
    for a in ctx_without_x.iter() {
        if a.free_vars().contains(x) {
            return None;
        }
    }
    let forall = Rc::new(PropWWF::Forall(x.to_string(), j.prop.clone()));
    Some(Judgement::new(ctx_without_x, forall))
}

/// ∀-elim (universal instantiation).
///
/// From `Γ ⊢ ∀x. φ`, derive `Γ ⊢ φ[t/x]`.
pub fn forall_elim(j: &Judgement, t: &str) -> Option<Judgement> {
    match j.prop.as_ref() {
        PropWWF::Forall(x, body) => {
            let inst = body.subst_term(x, t);
            Some(Judgement::new(j.ctx.clone(), inst))
        }
        _ => None,
    }
}

/// ∃-intro (existential generalisation).
///
/// From `Γ ⊢ φ` where term `t` occurs in φ, derive `Γ ⊢ ∃x. φ[x/t]`.
/// `t` is the concrete term to generalise; `x` is the fresh binding variable.
pub fn exists_intro(j: &Judgement, t: &str, x: &str) -> Option<Judgement> {
    if !formula::is_term_var(x) {
        return None;
    }
    let subst = j.prop.subst_term(t, x);
    let exists = Rc::new(PropWWF::Exists(x.to_string(), subst));
    Some(Judgement::new(j.ctx.clone(), exists))
}

/// ∃-elim (existential instantiation / witness elimination).
///
/// From `Γ ⊢ ∃x. φ` and `Δ, x, φ[y/x] ⊢ ψ` where `y` is not free in `Δ`, `ψ`,
/// nor `∃x.φ`, derive `Γ ∪ Δ ⊢ ψ`.  Both the witness atom `y` and the instantiated
/// body `φ[y/x]` are discharged.
pub fn exists_elim(j_ex: &Judgement, j_wit: &Judgement, y: &str) -> Option<Judgement> {
    if !formula::is_term_var(y) {
        return None;
    }
    // Step N must be ∃x. φ — extract the body.
    let (x_b, body) = match j_ex.prop.as_ref() {
        PropWWF::Exists(x, body) => (x.clone(), body.clone()),
        _ => return None,
    };
    if x_b == y {
        return None; // the witness variable must differ from the bound variable
    }
    // The witness context must contain Atom(y) (the bare witness term).
    let atom_y = PropWWF::Atom(y.to_string());
    let ctx1 = j_wit.ctx.decompose(&atom_y)?;

    // Compute φ[y/x] — the instantiated body — and remove it from ctx1.
    let witness_body = body.subst_term(&x_b, y);
    let ctx2 = ctx1.decompose(&witness_body)?;

    // y must not be free in the remaining context Δ.
    for a in ctx2.iter() {
        if a.free_vars().contains(y) { return None; }
    }
    // y must not be free in the conclusion ψ.
    if j_wit.prop.free_vars().contains(y) { return None; }

    // Result: Γ ∪ Δ, conclusion ψ.
    let ctx = j_ex.ctx.union_with(&ctx2);
    Some(Judgement::new(ctx, j_wit.prop.clone()))
}
