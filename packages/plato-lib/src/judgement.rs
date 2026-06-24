use std::fmt;
use std::rc::Rc;

use crate::context::Context;
use crate::formula::PropWWF;

/// A sequent / judgement `Γ ⊢ p`, where Γ is a context (set of assumptions)
/// and `p` is the derived proposition.
#[derive(Debug, Clone)]
pub struct Judgement {
    pub ctx: Context,
    pub prop: Rc<PropWWF>,
}

impl Judgement {
    /// Creates a new judgement with the given context and proposition.
    pub fn new(ctx: Context, prop: Rc<PropWWF>) -> Self {
        Self { ctx, prop }
    }
}

impl Judgement {
    /// Returns a natural-language description.
    /// "A implies A", "A and B entails A or B", etc.
    pub fn verbal(&self) -> String {
        let ant = if self.ctx.is_empty() {
            "".to_string()
        } else {
            let parts: Vec<String> = self.ctx.iter().map(|f| f.verbal()).collect();
            parts.join(" and ")
        };
        let consq = self.prop.verbal();
        if ant.is_empty() {
            consq
        } else {
            format!("{} entails {}", ant, consq)
        }
    }

    /// Returns a LaTeX representation.
    /// `\vdash A`, `\{A, B\} \vdash A \land B`.
    pub fn latex(&self) -> String {
        let ctx = if self.ctx.is_empty() {
            "\\emptyset".into()
        } else {
            let parts: Vec<String> = self.ctx.iter().map(|f| f.latex()).collect();
            format!("\\{{{}\\}}", parts.join(", "))
        };
        format!("{} \\vdash {}", ctx, self.prop.latex())
    }
}

impl fmt::Display for Judgement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ctx.is_empty() {
            write!(f, "∅ ⊢ {}", self.prop)
        } else {
            write!(f, "{{{}}} ⊢ {}", self.ctx, self.prop)
        }
    }
}
