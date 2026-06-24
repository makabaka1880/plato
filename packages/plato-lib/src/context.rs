use std::collections::HashSet;
use std::fmt;

use crate::formula::PropWWF;

/// A context — a set of assumptions in a sequent `Γ ⊢ p`.
#[derive(Debug, Clone, PartialEq)]
pub struct Context(HashSet<PropWWF>);

impl Context {
    /// Creates an empty context.
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    /// Inserts a formula into this context.
    pub fn insert(&mut self, prop: PropWWF) {
        self.0.insert(prop);
    }

    /// Returns `true` if the formula is in this context.
    pub fn contains(&self, prop: &PropWWF) -> bool {
        self.0.contains(prop)
    }

    /// If `prop` is in the context, returns a new context with it removed;
    /// otherwise returns `None`.
    pub fn decompose(&self, prop: &PropWWF) -> Option<Self> {
        if self.contains(prop) {
            let mut new_set = self.0.clone();
            new_set.remove(prop);
            return Some(Self(new_set));
        }
        None
    }

    /// Returns the union of this context with another.
    pub fn union_with(&self, other: &Self) -> Self {
        Self(self.0.union(&other.0).cloned().collect())
    }

    /// Returns an iterator over the assumptions.
    pub fn iter(&self) -> impl Iterator<Item = &PropWWF> {
        self.0.iter()
    }

    /// Returns the number of assumptions.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the context is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for a in &self.0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", a)?;
            first = false;
        }
        Ok(())
    }
}
