use super::session::Session;

#[test]
fn parse_atom_formula() {
    let mut s = Session::new();
    assert_eq!(s.execute("A"), Ok("formula: A".into()));
}

#[test]
fn parse_compound_formula() {
    let mut s = Session::new();
    let out = s.execute("(-> A (and B C))").unwrap();
    assert!(out.starts_with("formula: (A) → ((B) ⋀ (C))"));
}

#[test]
fn assume_and_var() {
    let mut s = Session::new();
    assert_eq!(s.execute("(assume A)").unwrap(), "1. {A} ⊢ A");
    assert_eq!(s.execute("(var 1 A)").unwrap(), "2. {A} ⊢ A");
    assert!(s.execute("(var 1 B)").is_err());
}

#[test]
fn derive_identity() {
    let mut s = Session::new();
    s.execute("(assume A)").unwrap();
    let out = s.execute("(->-intro A 1)").unwrap();
    assert_eq!(out, "2. ∅ ⊢ (A) → (A)");
}

#[test]
fn conj_intro_and_elim() {
    let mut s = Session::new();
    s.execute("(assume A)").unwrap();
    s.execute("(assume B)").unwrap();
    let out = s.execute("(and-intro 1 2)").unwrap();
    assert!(out.contains("⋀"));
    let out = s.execute("(and-elim-l 3)").unwrap();
    assert!(out.contains("⊢ A") && out.contains("A") && out.contains("B"));
    let out = s.execute("(and-elim-r 3)").unwrap();
    assert!(out.contains("⊢ B") && out.contains("A") && out.contains("B"));
}

#[test]
fn derive_lem_s_expr() {
    let mut s = Session::new();
    s.execute("(assume A)").unwrap();
    let out = s.execute("(->-intro A 1)").unwrap();
    assert_eq!(out, "2. ∅ ⊢ (A) → (A)");
}

#[test]
fn modus_ponens() {
    let mut s = Session::new();
    s.execute("(assume A)").unwrap(); // 1: {A} ⊢ A
    s.execute("(->-intro A 1)").unwrap(); // 2: ∅ ⊢ A -> A
    s.execute("(assume A)").unwrap(); // 3: {A} ⊢ A
    let out = s.execute("(->-elim 2 3)").unwrap();
    assert_eq!(out, "4. {A} ⊢ A");
}

#[test]
fn ex_falso_test() {
    let mut s = Session::new();
    s.execute("(assume _|_)").unwrap();
    let out = s.execute("(ex-falso 1 A)").unwrap();
    assert_eq!(out, "2. {⊥} ⊢ A");
}

#[test]
fn double_neg_elim() {
    let mut s = Session::new();
    s.execute("(assume (not (not A)))").unwrap();
    let out = s.execute("(dne 1)").unwrap();
    assert_eq!(out, "2. {¬(¬(A))} ⊢ A");
}

#[test]
fn parse_comment_ignored() {
    let mut s = Session::new();
    assert_eq!(
        s.execute("(assume A) ; this is a comment").unwrap(),
        "1. {A} ⊢ A"
    );
}

#[test]
fn error_on_bad_step() {
    let mut s = Session::new();
    assert!(s.execute("(and-elim-l 99)").is_err());
}

#[test]
fn tactic_equals_normalises_whitespace() {
    use crate::parser::sexpr::tactic_equals;
    assert!(tactic_equals("(assume I)", "(assume  I)"));
    assert!(tactic_equals("(->-intro I 1)", "(->-intro I 1)"));
    assert!(!tactic_equals("(assume I)", "(assume A)"));
    assert!(!tactic_equals("(->-intro I 1)", "(->-elim I 1)"));
}

#[test]
fn goal_resolution() {
    let empty: Vec<String> = vec![];
    let mut s = Session::new();
    assert!(!s.last_step_satisfies_goal("(-> I I)", &empty));

    s.execute("(assume I)").unwrap();
    // context is {I}, not empty — still not resolved
    assert!(!s.last_step_satisfies_goal("(-> I I)", &empty));

    s.execute("(->-intro I 1)").unwrap();
    // context is now empty, conclusion is I→I
    assert!(s.last_step_satisfies_goal("(-> I I)", &empty));
    assert!(!s.last_step_satisfies_goal("(-> I A)", &empty));
}

#[test]
fn verbal_formulas() {
    use crate::formula::PropWWF;
    use crate::judgement::Judgement;
    use crate::context::Context;
    use std::rc::Rc;

    let a = Rc::new(PropWWF::Atom("A".into()));
    let b = Rc::new(PropWWF::Atom("B".into()));
    let and_ab = Rc::new(PropWWF::Conj(a.clone(), b.clone()));
    let imp = Rc::new(PropWWF::Imp(a.clone(), a.clone()));

    assert_eq!(a.verbal(), "A");
    assert_eq!(and_ab.verbal(), "A and B");
    assert_eq!(imp.verbal(), "if A then A");

    let mut ctx = Context::new();
    ctx.insert(a.as_ref().clone());
    let j = Judgement::new(ctx, a.clone());
    assert_eq!(j.verbal(), "A entails A");
    assert!(j.verbal().contains("entails"));
}
