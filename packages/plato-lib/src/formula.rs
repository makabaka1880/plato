use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A propositional well-formed formula.
#[derive(Debug, Clone, Eq)]
pub enum PropWWF {
    /// Truth (verum).
    Top,
    /// Falsity (contradiction).
    Bottom,
    /// An atomic proposition, e.g. `A`, `p`.
    Atom(String),
    /// Negation: ¬p.
    Neg(Rc<PropWWF>),
    /// Conjunction: p ∧ q.
    Conj(Rc<PropWWF>, Rc<PropWWF>),
    /// Disjunction: p ∨ q.
    Disj(Rc<PropWWF>, Rc<PropWWF>),
    /// Implication: p → q.
    Imp(Rc<PropWWF>, Rc<PropWWF>),
}

impl PartialEq for PropWWF {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Atom(l), Self::Atom(r)) => l == r,
            (Self::Neg(l), Self::Neg(r)) => l == r,
            // Conjunction is commutative: p∧q ≡ q∧p
            (Self::Conj(l0, l1), Self::Conj(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            // Disjunction is commutative: p∨q ≡ q∨p
            (Self::Disj(l0, l1), Self::Disj(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            // Implication is NOT commutative: p→q ≢ q→p
            (Self::Imp(ant, consq), Self::Imp(ant2, consq2)) => ant == ant2 && consq == consq2,
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
            // Commutative: canonicalize by string ordering so a∧b and b∧a hash the same.
            Self::Conj(p, q) | Self::Disj(p, q) => {
                let sp = p.to_string();
                let sq = q.to_string();
                if sp <= sq {
                    sp.hash(state);
                    sq.hash(state);
                } else {
                    sq.hash(state);
                    sp.hash(state);
                }
            }
            // Implication is not commutative — order matters.
            Self::Imp(ant, consq) => {
                ant.hash(state);
                consq.hash(state);
            }
        }
    }
}

impl PropWWF {
    /// Returns a natural-language description.
    /// "A", "A and B", "A or B", "if A then B", "not A", "true", "false".
    pub fn verbal(&self) -> String {
        match self {
            Self::Top => "true".into(),
            Self::Bottom => "false".into(),
            Self::Atom(s) => format!("{}", s),
            Self::Neg(p) => format!("not {}", p.verbal()),
            Self::Conj(p, q) => format!("{} and {}", p.verbal(), q.verbal()),
            Self::Disj(p, q) => format!("{} or {}", p.verbal(), q.verbal()),
            Self::Imp(ant, consq) => format!("if {} then {}", ant.verbal(), consq.verbal()),
        }
    }

    /// Returns a LaTeX representation.
    /// `A`, `\top`, `\bot`, `A \land B`, `A \lor B`, `\lnot A`, `A \to B`.
    pub fn latex(&self) -> String {
        match self {
            Self::Top => "\\top".into(),
            Self::Bottom => "\\bot".into(),
            Self::Atom(s) => s.clone(),
            Self::Neg(p) => format!("\\lnot {}", p.latex()),
            Self::Conj(p, q) => format!("{} \\land {}", p.latex(), q.latex()),
            Self::Disj(p, q) => format!("{} \\lor {}", p.latex(), q.latex()),
            Self::Imp(ant, consq) => format!("{} \\to {}", ant.latex(), consq.latex()),
        }
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
        }
    }
}
