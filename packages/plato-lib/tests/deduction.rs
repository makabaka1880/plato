use std::rc::Rc;

use plato_lib::{
    context::Context,
    formula::PropWWF,
    inference::DeductionRule,
};

/// Proves `∅ ⊢ A → A` (the identity principle).
#[test]
fn derive_id() {
    let a = Rc::new(PropWWF::Atom("A".to_string()));
    let mut ctx = Context::new();
    ctx.insert(a.as_ref().clone());

    let j1 = DeductionRule::VarIntro(ctx.clone(), a.clone())
        .deduce()
        .unwrap();
    let j2 = DeductionRule::ImpIntro(a.clone(), Rc::new(j1.clone()))
        .deduce()
        .unwrap();

    println!("1. {}", j1);
    println!("2. {}", j2);

    // j2 should be {} |- A -> A
    assert!(j2.ctx.is_empty());
    assert_eq!(
        *j2.prop,
        PropWWF::Imp(a.clone(), a.clone()),
    );
}

/// Proves `∅ ⊢ A ∨ ¬A` — the Law of Excluded Middle (classical).
#[test]
fn derive_lem() {
    let a = Rc::new(PropWWF::Atom("A".to_string()));
    let n_a = Rc::new(PropWWF::Neg(a.clone()));
    let lem = Rc::new(PropWWF::Disj(a.clone(), n_a.clone()));
    let n_lem = Rc::new(PropWWF::Neg(lem.clone()));

    let mut ctx1 = Context::new();
    ctx1.insert(n_lem.as_ref().clone());
    ctx1.insert(a.as_ref().clone());

    let j1 = DeductionRule::VarIntro(ctx1.clone(), n_lem.clone())
        .deduce()
        .unwrap(); // -(A| -A), A |- -(A| -A)
    let j2 = DeductionRule::VarIntro(ctx1.clone(), a.clone())
        .deduce()
        .unwrap(); // -(A| -A), A |- A
    let j3 = DeductionRule::DisjIntroL(Rc::new(j2.clone()), n_a.clone())
        .deduce()
        .unwrap(); // -(A| -A), A |- A| -A
    let j4 = DeductionRule::NegIntro(a.clone(), Rc::new(j3.clone()), Rc::new(j1.clone()))
        .deduce()
        .unwrap(); // -(A| -A), A |- -A

    let mut ctx2 = Context::new();
    ctx2.insert(n_lem.as_ref().clone());
    ctx2.insert(n_a.as_ref().clone());

    let j5 = DeductionRule::VarIntro(ctx2.clone(), n_lem.clone())
        .deduce()
        .unwrap(); // -(A| -A), -A |- -(A| -A)
    let j6 = DeductionRule::VarIntro(ctx2.clone(), n_a.clone())
        .deduce()
        .unwrap(); // -(A| -A), -A |- -A
    let j7 = DeductionRule::DisjIntroR(Rc::new(j6.clone()), a.clone())
        .deduce()
        .unwrap(); // -(A| -A), -A |- A| -A
    let j8 = DeductionRule::NegIntro(n_a.clone(), Rc::new(j7.clone()), Rc::new(j5.clone()))
        .deduce()
        .unwrap(); // -(A| -A) |- --A
    let j9 = DeductionRule::NegIntro(n_lem.clone(), Rc::new(j4.clone()), Rc::new(j8.clone()))
        .deduce()
        .unwrap(); // |- --(A| -A)
    let j10 = DeductionRule::DNegElim(Rc::new(j9.clone()))
        .deduce()
        .unwrap(); // |- A| -A

    println!("1. {}", j1);
    println!("2. {}", j2);
    println!("3. {}", j3);
    println!("4. {}", j4);
    println!("5. {}", j5);
    println!("6. {}", j6);
    println!("7. {}", j7);
    println!("8. {}", j8);
    println!("9. {}", j9);
    println!("*. {}", j10);

    // j10 should be {} |- A \/ -A
    assert!(j10.ctx.is_empty());
    assert_eq!(*j10.prop, *lem);
}
