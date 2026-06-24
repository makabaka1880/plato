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

    /// Returns a LaTeX representation with explicit parentheses
    /// for compound sub-formulas so the structure is unambiguous.
    /// `A`, `\top`, `\bot`, `A \land B`, `A \lor B`, `\lnot A`, `A \to B`.
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
            Self::Conj(p, q) => format!(
                "{} \\land {}",
                p.latex_child(),
                q.latex_child()
            ),
            Self::Disj(p, q) => format!(
                "{} \\lor {}",
                p.latex_child(),
                q.latex_child()
            ),
            Self::Imp(ant, consq) => format!(
                "{} \\to {}",
                ant.latex_child(),
                consq.latex_child()
            ),
        }
    }

    /// Whether this formula is compound (needs parentheses when used as a child).
    fn is_compound(&self) -> bool {
        matches!(
            self,
            Self::Conj(..) | Self::Disj(..) | Self::Imp(..) | Self::Neg(..)
        )
    }

    /// Render a child formula, parenthesizing it if compound.
    fn latex_child(&self) -> String {
        let s = self.latex();
        if self.is_compound() {
            format!("\\left({}\\right)", s)
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    fn f(s: &str) -> String {
        use crate::parser::sexpr;
        let tokens = sexpr::tokenize(s);
        let sexpr = sexpr::parse_one(&tokens).unwrap();
        crate::parser::formula::parse_formula(&sexpr).unwrap().latex()
    }

    #[test]
    fn latex_parens() {
        // Atoms don't get parenthesized
        assert_eq!(f("A"), "A");
        // Simple binary — no internal parens needed
        assert_eq!(f("(and A B)"), "A \\land B");
        // Nesting: compound child gets wrapped
        assert_eq!(f("(-> A (or A B))"), "A \\to \\left(A \\lor B\\right)");
        assert_eq!(
            f("(-> (and A B) A)"),
            "\\left(A \\land B\\right) \\to A"
        );
        assert_eq!(
            f("(-> (or A B) (or B A))"),
            "\\left(A \\lor B\\right) \\to \\left(B \\lor A\\right)"
        );
        // Deep nesting: the previously-ambiguous formula
        assert_eq!(
            f("(-> (and (or A B) (-> A C)) (or C B))"),
            "\\left(\\left(A \\lor B\\right) \\land \\left(A \\to C\\right)\\right) \\to \\left(C \\lor B\\right)"
        );
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
