use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A propositional (now first-order!) well-formed formula.
#[derive(Debug, Clone, Eq)]
pub enum PropWWF {
    /// Truth (verum).
    Top,
    /// Falsity (contradiction).
    Bottom,
    /// An atomic symbol — either a proposition variable (uppercase)
    /// or a term variable (lowercase), distinguished by convention.
    Atom(String),
    /// Negation: ¬p.
    Neg(Rc<PropWWF>),
    /// Conjunction: p ∧ q.
    Conj(Rc<PropWWF>, Rc<PropWWF>),
    /// Disjunction: p ∨ q.
    Disj(Rc<PropWWF>, Rc<PropWWF>),
    /// Implication: p → q.
    Imp(Rc<PropWWF>, Rc<PropWWF>),
    /// Universal quantification: ∀x. φ.
    Forall(String, Rc<PropWWF>),
    /// Existential quantification: ∃x. φ.
    Exists(String, Rc<PropWWF>),
    /// Predicate application: P(t) — an uppercase symbol applied to a lower one.
    /// Nested applications: ((F t1) t2) …
    App(Rc<PropWWF>, Rc<PropWWF>),
    /// Necessity modality: □p.
    Box(Rc<PropWWF>),
    /// Possibility modality: ◇p.
    Diamond(Rc<PropWWF>),
}

impl PartialEq for PropWWF {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Atom(l), Self::Atom(r)) => l == r,
            (Self::Neg(l), Self::Neg(r)) => l == r,
            (Self::Conj(l0, l1), Self::Conj(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            (Self::Disj(l0, l1), Self::Disj(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            (Self::Imp(ant, consq), Self::Imp(ant2, consq2)) => ant == ant2 && consq == consq2,
            (Self::Forall(x, body), Self::Forall(y, body2)) => x == y && body == body2,
            (Self::Exists(x, body), Self::Exists(y, body2)) => x == y && body == body2,
            (Self::App(p1, t1), Self::App(p2, t2)) => p1 == p2 && t1 == t2,
            (Self::Box(p1), Self::Box(p2)) => p1 == p2,
            (Self::Diamond(p1), Self::Diamond(p2)) => p1 == p2,
            (Self::Top, Self::Top) => true,
            (Self::Bottom, Self::Bottom) => true,
            _ => false,
        }
    }
}

impl Hash for PropWWF {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Top | Self::Bottom => {}
            Self::Atom(s) => s.hash(state),
            Self::Neg(p) => p.hash(state),
            Self::Conj(p, q) | Self::Disj(p, q) => {
                let sp = p.to_string();
                let sq = q.to_string();
                if sp <= sq {
                    sp.hash(state); sq.hash(state);
                } else {
                    sq.hash(state); sp.hash(state);
                }
            }
            Self::Imp(ant, consq) => { ant.hash(state); consq.hash(state); }
            Self::Forall(x, body) => { x.hash(state); body.hash(state); }
            Self::Exists(x, body) => { x.hash(state); body.hash(state); }
            Self::App(p, t) => { p.hash(state); t.hash(state); }
            Self::Box(p) => p.hash(state),
            Self::Diamond(p) => p.hash(state),
        }
    }
}

impl PropWWF {
    /// Collect every free term variable (lowercase atoms) in this formula.
    pub fn free_vars(&self) -> HashSet<String> {
        let mut fv = HashSet::new();
        self.collect_free(&mut fv, &HashSet::new());
        fv
    }

    fn collect_free(&self, out: &mut HashSet<String>, bound: &HashSet<String>) {
        match self {
            Self::Atom(s) => {
                if is_term_var(s) && !bound.contains(s) {
                    out.insert(s.clone());
                }
            }
            Self::Neg(p) => p.collect_free(out, bound),
            Self::Conj(p, q) | Self::Disj(p, q) | Self::App(p, q) => {
                p.collect_free(out, bound);
                q.collect_free(out, bound);
            }
            Self::Imp(ant, consq) => {
                ant.collect_free(out, bound);
                consq.collect_free(out, bound);
            }
            Self::Forall(x, body) | Self::Exists(x, body) => {
                let mut b2 = bound.clone();
                b2.insert(x.clone());
                body.collect_free(out, &b2);
            }
            Self::Box(p) | Self::Diamond(p) => p.collect_free(out, bound),
            Self::Top | Self::Bottom => {}
        }
    }

    /// Replace every free occurrence of `x` (term var) with `t`.
    /// Avoids capture by not substituting under a binder for `x`.
    pub fn subst_term(&self, x: &str, t: &str) -> Rc<PropWWF> {
        self.subst_term_inner(x, t, &HashSet::new())
    }

    fn subst_term_inner(&self, x: &str, t: &str, bound: &HashSet<String>) -> Rc<PropWWF> {
        if bound.contains(x) {
            return Rc::new(self.clone());
        }
        match self {
            Self::Atom(s) => {
                if s == x { Rc::new(Self::Atom(t.to_string())) }
                else { Rc::new(Self::Atom(s.clone())) }
            }
            Self::Neg(p) => Rc::new(Self::Neg(p.subst_term_inner(x, t, bound))),
            Self::Conj(p, q) => Rc::new(Self::Conj(
                p.subst_term_inner(x, t, bound),
                q.subst_term_inner(x, t, bound),
            )),
            Self::Disj(p, q) => Rc::new(Self::Disj(
                p.subst_term_inner(x, t, bound),
                q.subst_term_inner(x, t, bound),
            )),
            Self::Imp(ant, consq) => Rc::new(Self::Imp(
                ant.subst_term_inner(x, t, bound),
                consq.subst_term_inner(x, t, bound),
            )),
            Self::App(p, a) => Rc::new(Self::App(
                p.subst_term_inner(x, t, bound),
                a.subst_term_inner(x, t, bound),
            )),
            Self::Box(p) => Rc::new(Self::Box(p.subst_term_inner(x, t, bound))),
            Self::Diamond(p) => Rc::new(Self::Diamond(p.subst_term_inner(x, t, bound))),
            Self::Forall(y, body) => {
                let mut b2 = bound.clone();
                b2.insert(y.clone());
                Rc::new(Self::Forall(y.clone(), body.subst_term_inner(x, t, &b2)))
            }
            Self::Exists(y, body) => {
                let mut b2 = bound.clone();
                b2.insert(y.clone());
                Rc::new(Self::Exists(y.clone(), body.subst_term_inner(x, t, &b2)))
            }
            Self::Top => Rc::new(Self::Top),
            Self::Bottom => Rc::new(Self::Bottom),
        }
    }

    pub fn verbal(&self) -> String {
        match self {
            Self::Top => "true".into(),
            Self::Bottom => "false".into(),
            Self::Atom(s) => s.clone(),
            Self::Neg(p) => format!("not {}", p.verbal()),
            Self::Conj(p, q) => format!("{} and {}", p.verbal(), q.verbal()),
            Self::Disj(p, q) => format!("{} or {}", p.verbal(), q.verbal()),
            Self::Imp(ant, consq) => format!("if {} then {}", ant.verbal(), consq.verbal()),
            Self::Forall(x, body) => format!("for all {}: {}", x, body.verbal()),
            Self::Exists(x, body) => format!("there exists {}: {}", x, body.verbal()),
            Self::App(p, t) => {
                let parts = self.flatten_app();
                if parts.len() <= 2 {
                    format!("{}({})", p.verbal(), t.verbal())
                } else {
                    let pred = &parts[0];
                    let args: Vec<String> = parts[1..].iter().map(|a| a.verbal()).collect();
                    format!("{}({})", pred.verbal(), args.join(", "))
                }
            }
            Self::Box(p) => format!("necessarily: {}", p.verbal()),
            Self::Diamond(p) => format!("possibly: {}", p.verbal()),
        }
    }

    /// Flatten nested (App (App (App P t1) t2) …) into (P, [t1, t2, …]).
    fn flatten_app(&self) -> Vec<Rc<PropWWF>> {
        let mut parts: Vec<Rc<PropWWF>> = vec![];
        // Walk the chain: push each leaf argument, then recurse into p.
        match self {
            Self::App(p, t) => {
                parts.extend(p.flatten_app());
                parts.push(Rc::clone(t));
            }
            _ => {
                parts.push(Rc::new(self.clone()));
            }
        }
        parts
    }

    pub fn latex(&self) -> String {
        match self {
            Self::Top => "\\top".into(),
            Self::Bottom => "\\bot".into(),
            Self::Atom(s) => s.clone(),
            Self::Neg(p) => {
                let inner = p.latex();
                if p.is_compound() {
                    format!("\\lnot \\left({}\\right)", inner)
                } else {
                    format!("\\lnot {}", inner)
                }
            }
            Self::Conj(p, q) => format!("{} \\land {}", p.latex_child(), q.latex_child()),
            Self::Disj(p, q) => format!("{} \\lor {}", p.latex_child(), q.latex_child()),
            Self::Imp(ant, consq) => format!("{} \\to {}", ant.latex_child(), consq.latex_child()),
            Self::Forall(x, body) => format!("\\forall {} \\; {}", x, body.latex_child()),
            Self::Exists(x, body) => format!("\\exists {} \\; {}", x, body.latex_child()),
            Self::App(p, t) => {
                let parts = self.flatten_app();
                if parts.len() <= 2 {
                    format!("{}({})", p.latex(), t.latex())
                } else {
                    let pred = &parts[0];
                    let args: Vec<String> = parts[1..].iter().map(|a| a.latex()).collect();
                    format!("{}({})", pred.latex(), args.join(", "))
                }
            }
            Self::Box(p) => format!("\\Box {}", p.latex_child()),
            Self::Diamond(p) => format!("\\Diamond {}", p.latex_child()),
        }
    }

    fn is_compound(&self) -> bool {
        matches!(
            self,
            Self::Conj(..) | Self::Disj(..) | Self::Imp(..)
            | Self::Neg(..) | Self::Forall(..) | Self::Exists(..)
            | Self::Box(..) | Self::Diamond(..)
        )
    }

    pub fn substitute(&self, subs: &[(String, Rc<PropWWF>)]) -> Rc<PropWWF> {
        match self {
            Self::Top => Rc::new(Self::Top),
            Self::Bottom => Rc::new(Self::Bottom),
            Self::Atom(s) => {
                for (name, repl) in subs {
                    if s == name { return repl.clone(); }
                }
                Rc::new(Self::Atom(s.clone()))
            }
            Self::Neg(p) => Rc::new(Self::Neg(p.substitute(subs))),
            Self::Conj(p, q) => Rc::new(Self::Conj(p.substitute(subs), q.substitute(subs))),
            Self::Disj(p, q) => Rc::new(Self::Disj(p.substitute(subs), q.substitute(subs))),
            Self::Imp(ant, consq) => Rc::new(Self::Imp(ant.substitute(subs), consq.substitute(subs))),
            Self::Forall(x, body) => Rc::new(Self::Forall(x.clone(), body.substitute(subs))),
            Self::Exists(x, body) => Rc::new(Self::Exists(x.clone(), body.substitute(subs))),
            Self::Box(p) => Rc::new(Self::Box(p.substitute(subs))),
            Self::Diamond(p) => Rc::new(Self::Diamond(p.substitute(subs))),
            Self::App(p, t) => Rc::new(Self::App(p.substitute(subs), t.substitute(subs))),
        }
    }

    fn latex_child(&self) -> String {
        let s = self.latex();
        if self.is_compound() {
            format!("\\left({}\\right)", s)
        } else {
            s
        }
    }
}

/// `true` for atoms that conventionally name term variables (start with lowercase).
pub fn is_term_var(s: &str) -> bool {
    s.starts_with(|c: char| c.is_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(s: &str) -> String {
        use crate::parser::sexpr;
        let tokens = sexpr::tokenize(s);
        let sexpr = sexpr::parse_one(&tokens).unwrap();
        crate::parser::formula::parse_formula(&sexpr, None).unwrap().latex()
    }

    #[test]
    fn latex_parens() {
        assert_eq!(f("A"), "A");
        assert_eq!(f("(and A B)"), "A \\land B");
        assert_eq!(f("(-> A (or A B))"), "A \\to \\left(A \\lor B\\right)");
        assert_eq!(f("(-> (and A B) A)"), "\\left(A \\land B\\right) \\to A");
        assert_eq!(
            f("(-> (or A B) (or B A))"),
            "\\left(A \\lor B\\right) \\to \\left(B \\lor A\\right)"
        );
        assert_eq!(
            f("(-> (and (or A B) (-> A C)) (or C B))"),
            "\\left(\\left(A \\lor B\\right) \\land \\left(A \\to C\\right)\\right) \\to \\left(C \\lor B\\right)"
        );
    }

    #[test]
    fn quantifier_latex() {
        assert_eq!(f("(forall x (App P x))"), "\\forall x \\; P(x)");
        assert_eq!(
            f("(-> (forall x (App mortal x)) (App mortal socrates))"),
            "\\left(\\forall x \\; mortal(x)\\right) \\to mortal(socrates)"
        );
    }

    #[test]
    fn free_vars_simple() {
        let tokens = crate::parser::sexpr::tokenize("(forall x (App P x))");
        let sexpr = crate::parser::sexpr::parse_one(&tokens).unwrap();
        let fm = crate::parser::formula::parse_formula(&sexpr, None).unwrap();
        let fv = fm.free_vars();
        // Only uppercase P is free; x is bound.
        assert!(fv.is_empty(), "expected no free vars, got {:?}", fv);
    }

    #[test]
    fn free_vars_open() {
        let tokens = crate::parser::sexpr::tokenize("(App P x)");
        let sexpr = crate::parser::sexpr::parse_one(&tokens).unwrap();
        let fm = crate::parser::formula::parse_formula(&sexpr, None).unwrap();
        let fv = fm.free_vars();
        assert!(fv.contains("x"), "x should be free, got {:?}", fv);
    }

    #[test]
    fn subst_term_avoid_capture() {
        let tokens = crate::parser::sexpr::tokenize("(forall x (App P x))");
        let sexpr = crate::parser::sexpr::parse_one(&tokens).unwrap();
        let fm = crate::parser::formula::parse_formula(&sexpr, None).unwrap();
        // substituting x->y inside ∀x.P(x) should be a no-op
        let r = fm.subst_term("x", "y");
        assert_eq!(*fm, *r);
        // substituting P->Q should work normally
        let r2 = fm.substitute(&[("P".into(), Rc::new(PropWWF::Atom("Q".into())))]);
        assert_eq!(r2.to_string(), "∀x.(Q(x))");
    }
}

impl fmt::Display for PropWWF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "T"),
            Self::Bottom => write!(f, "⊥"),
            Self::Atom(s) => write!(f, "{}", s),
            Self::Neg(p) => write!(f, "¬({})", p),
            Self::Conj(lhs, rhs) => write!(f, "({}) ⋀ ({})", lhs, rhs),
            Self::Disj(lhs, rhs) => write!(f, "({}) ⋁ ({})", lhs, rhs),
            Self::Imp(ant, consq) => write!(f, "({}) → ({})", ant, consq),
            Self::Forall(x, body) => write!(f, "∀{}.({})", x, body),
            Self::Exists(x, body) => write!(f, "∃{}.({})", x, body),
            Self::Box(p) => write!(f, "□({})", p),
            Self::Diamond(p) => write!(f, "◇({})", p),
            Self::App(p, t) => {
                let parts = self.flatten_app();
                if parts.len() <= 2 {
                    write!(f, "{}({})", p, t)
                } else {
                    write!(f, "{}(", parts[0])?;
                    for (i, a) in parts[1..].iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", a)?;
                    }
                    write!(f, ")")
                }
            }
        }
    }
}
