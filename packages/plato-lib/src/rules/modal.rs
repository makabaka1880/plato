use std::rc::Rc;

use crate::context::Context;
use crate::formula::PropWWF;
use crate::judgement::Judgement;

/// Necessitation (NEC): from ∅ ⊢ A derive ∅ ⊢ □A.
/// The premise must have an empty context.
pub fn box_intro(j: &Judgement) -> Option<Judgement> {
    if !j.ctx.is_empty() {
        return None;
    }
    Some(Judgement::new(
        Context::new(),
        Rc::new(PropWWF::Box(j.prop.clone())),
    ))
}

/// K axiom: from Γ ⊢ □(A → B) and Δ ⊢ □A derive Γ ∪ Δ ⊢ □B.
pub fn box_elim(j_imp: &Judgement, j_box_a: &Judgement) -> Option<Judgement> {
    let a_to_b = match j_imp.prop.as_ref() {
        PropWWF::Box(inner) => match inner.as_ref() {
            PropWWF::Imp(a, b) => (a.clone(), b.clone()),
            _ => return None,
        },
        _ => return None,
    };
    let a = match j_box_a.prop.as_ref() {
        PropWWF::Box(inner) => inner.clone(),
        _ => return None,
    };
    // The antecedent of the boxed implication must match the boxed formula
    if *a_to_b.0 != *a {
        return None;
    }
    Some(Judgement::new(
        j_imp.ctx.union_with(&j_box_a.ctx),
        Rc::new(PropWWF::Box(a_to_b.1.clone())),
    ))
}

/// ◇-definition forward: from Γ ⊢ ◇A derive Γ ⊢ ¬□¬A.
pub fn diamond_def_fwd(j: &Judgement) -> Option<Judgement> {
    let a = match j.prop.as_ref() {
        PropWWF::Diamond(inner) => inner.clone(),
        _ => return None,
    };
    Some(Judgement::new(
        j.ctx.clone(),
        Rc::new(PropWWF::Neg(Rc::new(PropWWF::Box(Rc::new(
            PropWWF::Neg(a),
        ))))),
    ))
}

/// ◇-definition reverse: from Γ ⊢ ¬□¬A derive Γ ⊢ ◇A.
pub fn diamond_def_rev(j: &Judgement) -> Option<Judgement> {
    // Match ¬(□(¬A))
    let a = match j.prop.as_ref() {
        PropWWF::Neg(outer) => match outer.as_ref() {
            PropWWF::Box(inner) => match inner.as_ref() {
                PropWWF::Neg(a) => a.clone(),
                _ => return None,
            },
            _ => return None,
        },
        _ => return None,
    };
    Some(Judgement::new(
        j.ctx.clone(),
        Rc::new(PropWWF::Diamond(a)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::sexpr;
    use crate::parser::formula::parse_formula;

    fn pf(s: &str) -> Rc<PropWWF> {
        let tokens = sexpr::tokenize(s);
        let sexpr = sexpr::parse_one(&tokens).unwrap();
        parse_formula(&sexpr, None).unwrap()
    }

    fn empty_judgement(f: Rc<PropWWF>) -> Judgement {
        Judgement::new(Context::new(), f)
    }

    #[test]
    fn nec_requires_empty_ctx() {
        let f = pf("(box A)");
        let j = empty_judgement(pf("A"));
        let result = box_intro(&j);
        assert!(result.is_some());
        assert_eq!(result.unwrap().prop, f);
    }

    #[test]
    fn nec_rejects_nonempty_ctx() {
        let mut ctx = Context::new();
        ctx.insert(pf("B").as_ref().clone());
        let j = Judgement::new(ctx, pf("A"));
        assert!(box_intro(&j).is_none());
    }

    #[test]
    fn k_axiom_works() {
        let j1 = empty_judgement(pf("(box (-> A B))"));
        let j2 = empty_judgement(pf("(box A)"));
        let result = box_elim(&j1, &j2);
        assert!(result.is_some());
        assert_eq!(result.unwrap().prop, pf("(box B)"));
    }

    #[test]
    fn k_axiom_rejects_mismatch() {
        let j1 = empty_judgement(pf("(box (-> A B))"));
        let j2 = empty_judgement(pf("(box C)"));
        assert!(box_elim(&j1, &j2).is_none());
    }

    #[test]
    fn diamond_def_fwd_works() {
        let j = empty_judgement(pf("(diamond A)"));
        let result = diamond_def_fwd(&j);
        assert!(result.is_some());
        assert_eq!(result.unwrap().prop, pf("(not (box (not A)))"));
    }

    #[test]
    fn diamond_def_rev_works() {
        let j = empty_judgement(pf("(not (box (not A)))"));
        let result = diamond_def_rev(&j);
        assert!(result.is_some());
        assert_eq!(result.unwrap().prop, pf("(diamond A)"));
    }
}
