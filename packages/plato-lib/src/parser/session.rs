use std::rc::Rc;

use crate::context::Context;
use crate::inference::DeductionRule;
use crate::judgement::Judgement;
use crate::rules;
use super::command::{Command, parse_input};

/// A proof session. Maintains a numbered history of judgements.
pub struct Session {
    steps: Vec<Rc<Judgement>>,
    /// Optional logic-mode gate. `None` means no restriction (legacy).
    /// `Some("pl")` — propositional logic: allows box/diamond, rejects quantifiers.
    /// `Some("fol")` — first-order logic: allows quantifiers, rejects box/diamond.
    mode: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Self { steps: Vec::new(), mode: None }
    }

    /// Set the logic mode. `"pl"` for propositional (modal), `"fol"` for first-order.
    pub fn set_mode(&mut self, mode: &str) {
        self.mode = Some(mode.to_string());
    }

    /// Get the current logic mode, if set.
    pub fn mode(&self) -> Option<&str> {
        self.mode.as_deref()
    }

    /// Number of steps so far.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Returns the Display text of step `n` (1-indexed), or None if out of range.
    pub fn step_text(&self, n: usize) -> Option<String> {
        if n < 1 || n > self.steps.len() {
            return None;
        }
        Some(format!("{}. {}", n, self.steps[n - 1]))
    }

    /// Returns LaTeX for step `n` (1-indexed), or None if out of range.
    pub fn step_latex(&self, n: usize) -> Option<String> {
        if n < 1 || n > self.steps.len() {
            return None;
        }
        Some(format!("{}.\\; {}", n, self.steps[n - 1].latex()))
    }

    /// Returns just the conclusion formula as LaTeX for step `n` (1-indexed), or None.
    pub fn step_formula_latex(&self, n: usize) -> Option<String> {
        if n < 1 || n > self.steps.len() {
            return None;
        }
        Some(self.steps[n - 1].prop.latex())
    }

    /// Parse an s-expression formula string and return its LaTeX.
    /// e.g. `"(and A B)"` → `"A \\land B"`.
    pub fn formula_latex(s: &str) -> String {
        let tokens = super::sexpr::tokenize(s);
        match super::sexpr::parse_one(&tokens) {
            Ok(sexpr) => match super::formula::parse_formula(&sexpr, None) {
                Ok(f) => f.latex(),
                Err(e) => e,
            },
            Err(e) => e,
        }
    }

    /// Checks whether the last step satisfies the goal.
    ///
    /// `goal` is an s-expression formula string (e.g. `"(-> I I)"`).
    /// `premises` are the premise formula strings given by the problem.
    ///
    /// Returns `true` if the last judgement's conclusion equals the
    /// parsed goal formula **and** the context contains exactly the
    /// given premises (no more, no less).
    pub fn last_step_satisfies_goal(&self, goal: &str, premises: &[String]) -> bool {
        if self.steps.is_empty() {
            return false;
        }

        // Parse goal formula
        let tokens = super::sexpr::tokenize(goal);
        let sexpr = match super::sexpr::parse_one(&tokens) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let goal_f = match super::formula::parse_formula(&sexpr, None) {
            Ok(f) => f,
            Err(_) => return false,
        };

        // Parse premises into a Context
        let mut expected_ctx = Context::new();
        for p_str in premises {
            let p_tokens = super::sexpr::tokenize(p_str);
            let p_sexpr = match super::sexpr::parse_one(&p_tokens) {
                Ok(s) => s,
                Err(_) => return false,
            };
            let p_f = match super::formula::parse_formula(&p_sexpr, None) {
                Ok(f) => f,
                Err(_) => return false,
            };
            expected_ctx.insert(p_f.as_ref().clone());
        }

        let last = &self.steps[self.steps.len() - 1];
        // Check conclusion matches goal AND context matches premises exactly
        *last.prop == *goal_f && last.ctx == expected_ctx
    }

    /// Returns the judgement at step `n` (1-indexed).
    fn step(&self, n: usize) -> Result<&Rc<Judgement>, String> {
        if n < 1 || n > self.steps.len() {
            Err(format!(
                "step {} does not exist (have {} steps)",
                n,
                self.steps.len()
            ))
        } else {
            Ok(&self.steps[n - 1])
        }
    }

    /// Execute one input line. Returns the formatted output.
    pub fn execute(&mut self, input: &str) -> Result<String, String> {
        let cmd = parse_input(input, self.mode.as_deref())?;
        Ok(match cmd {
            Command::Formula(f) => {
                format!("formula: {}", f)
            }
            Command::Show(0) => self.help(),
            Command::Show(n) => {
                let j = self.step(n)?;
                format!("{}  [{} ⊢ {}]", n, j.ctx, j.prop)
            }
            Command::Assume(f) => {
                let mut ctx = Context::new();
                ctx.insert(f.as_ref().clone());
                let j = rules::misc::var_intro(&ctx, f)
                    .ok_or_else(|| "internal error: assume".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::Var(n, f) => {
                let jn = self.step(n)?;
                let f2 = f.clone();
                let j = rules::misc::var_intro(&jn.ctx, f2)
                    .ok_or_else(|| format!("{} is not in the context of step {}", f, n))?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::AndIntro(n, m) => {
                let jn = self.step(n)?;
                let jm = self.step(m)?;
                let j = rules::conj::conj_intro(jn, jm)
                    .ok_or_else(|| "and-intro failed".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::AndElimL(n) => {
                let jn = self.step(n)?;
                let j = rules::conj::conj_elim_lhs(jn)
                    .ok_or_else(|| "and-elim-l failed: step is not a conjunction".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::AndElimR(n) => {
                let jn = self.step(n)?;
                let j = rules::conj::conj_elim_rhs(jn)
                    .ok_or_else(|| "and-elim-r failed: step is not a conjunction".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::OrIntroL(n, f) => {
                let jn = self.step(n)?;
                let j = rules::disj::disj_intro_lhs(jn, f)
                    .ok_or_else(|| "or-intro-l failed".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::OrIntroR(n, f) => {
                let jn = self.step(n)?;
                let j = rules::disj::disj_intro_rhs(jn, f)
                    .ok_or_else(|| "or-intro-r failed".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::OrElim(n, m, k) => {
                let jn = self.step(n)?;
                let jm = self.step(m)?;
                let jk = self.step(k)?;
                let j = rules::disj::disj_elim(jn, jm, jk)
                    .ok_or_else(|| "or-elim failed: check that step {} is a disjunction and steps {}/{} have the same conclusion with the appropriate cases in their contexts".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ImpIntro(f, n) => {
                let jn = self.step(n)?;
                let f2 = f.clone();
                let j = rules::imp::imp_intro(f2, jn).ok_or_else(|| {
                    format!("->-intro failed: {} is not in the context of step {}", f, n)
                })?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ImpElim(n, m) => {
                let jn = self.step(n)?;
                let jm = self.step(m)?;
                let j = rules::imp::imp_elim(jn, jm).ok_or_else(|| {
                    format!(
                        "->-elim failed: step {} does not imply the formula of step {}",
                        n, m
                    )
                })?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ImpInto(n) => {
                let jn = self.step(n)?;
                let j = rules::imp::imp_into(jn)
                    .ok_or_else(|| "->-into failed: step is not an implication".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::NotIntro(f, n, m) => {
                let jn = self.step(n)?;
                let jm = self.step(m)?;
                let j = rules::neg::neg_intro(f, jn, jm)
                    .ok_or_else(|| "not-intro failed: check that the discharged formula is in both contexts, and the two conclusions are contradictory".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::NotElim(n) => {
                let jn = self.step(n)?;
                let j = rules::neg::neg_elim(jn)
                    .ok_or_else(|| "not-elim failed: step is not a negation".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::DNegElim(n) => {
                let jn = self.step(n)?;
                let j = rules::neg::double_neg_elim(jn)
                    .ok_or_else(|| "dne failed: step is not a double negation".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ExFalso(n, f) => {
                let jn = self.step(n)?;
                let j = rules::misc::exfalso(jn, f)
                    .ok_or_else(|| "ex-falso failed: step is not ⊥".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::Subst(n, pairs) => {
                let jn = self.step(n)?.clone();
                let j = rules::misc::subst(&jn, &pairs)
                    .ok_or_else(|| "subst failed".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::Fix(f) => {
                let mut ctx = Context::new();
                ctx.insert(f.as_ref().clone());
                let j = rules::misc::var_intro(&ctx, f)
                    .ok_or_else(|| "internal error: fix".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ForallIntro(x, n) => {
                let jn = self.step(n)?;
                let j = rules::quant::forall_intro(&x, jn)
                    .ok_or_else(|| format!("forall-intro failed: {} is not in the context of step {}, or it appears free in other assumptions", x, n))?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ForallElim(n, t) => {
                let jn = self.step(n)?;
                let j = rules::quant::forall_elim(jn, &t)
                    .ok_or_else(|| "forall-elim failed: step is not a universal formula".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ExistsIntro(n, t, x) => {
                let jn = self.step(n)?;
                let j = rules::quant::exists_intro(jn, &t, &x)
                    .ok_or_else(|| format!("exists-intro failed: check that '{}' is a term variable", x))?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::ExistsElim(n, m, x) => {
                let jn = self.step(n)?;
                let jm = self.step(m)?;
                let j = rules::quant::exists_elim(jn, jm, &x)
                    .ok_or_else(|| format!("exists-elim failed: check that step {} is an existential, step {} has {} in its context, and {} is not free in the witness context or conclusion", n, m, x, x))?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::BoxIntro(n) => {
                let jn = self.step(n)?;
                let j = rules::modal::box_intro(jn)
                    .ok_or_else(|| "box-intro (NEC) failed: the context must be empty — no assumptions allowed".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::BoxElim(n, m) => {
                let jn = self.step(n)?;
                let jm = self.step(m)?;
                let j = rules::modal::box_elim(jn, jm)
                    .ok_or_else(|| format!("box-elim (K) failed: step {} must be □(X → Y) and step {} must be □X", n, m))?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::DiamondDef(n) => {
                let jn = self.step(n)?;
                let j = rules::modal::diamond_def_fwd(jn)
                    .ok_or_else(|| "diamond-def failed: step is not a ◇ formula".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::DiamondDefRev(n) => {
                let jn = self.step(n)?;
                let j = rules::modal::diamond_def_rev(jn)
                    .ok_or_else(|| "diamond-def-rev failed: step is not a ¬□¬A formula".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::TopIntro => {
                let j = DeductionRule::TopIntro(Context::new()).deduce().unwrap();
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::TopIntroCtx(n) => {
                let jn = self.step(n)?;
                let j = DeductionRule::TopIntroCtx(Rc::clone(jn)).deduce().unwrap();
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
            Command::Extend(f, n) => {
                let jn = self.step(n)?;
                let j = rules::misc::extend(jn, f)
                    .ok_or_else(|| "extend failed: could not add formula to context".to_string())?;
                self.steps.push(Rc::new(j));
                self.last_fmt()
            }
        })
    }

    fn last_fmt(&self) -> String {
        let n = self.steps.len();
        let j = &self.steps[n - 1];
        format!("{}. {}", n, j)
    }

    fn help(&self) -> String {
        "\
Commands:
  (assume F)            new assumption {F} ⊢ F
  (var N F)             derive F from step N's context
  (and-intro N M)       conjunction introduction
  (and-elim-l N)        conjunction elimination left
  (and-elim-r N)        conjunction elimination right
  (or-intro-l N F)      disjunction introduction left
  (or-intro-r N F)      disjunction introduction right
  (or-elim N M K)       disjunction elimination (proof by cases)
  (->-intro F N)        implication introduction (discharges F)
  (->-elim N M)         modus ponens
  (->-into N)           implication unpack
  (not-intro F N M)     negation introduction (reductio)
  (not-elim N)          negation elimination
  (dne N)               double negation elimination
  (ex-falso N F)        ex falso quodlibet
  (fix x)               introduce a term variable {x}⊢x
  (forall-intro x N)    universal generalisation (discharges x)
  (forall-elim N t)     universal instantiation
  (exists-intro N t x)  existential generalisation
  (exists-elim N M x)   existential witness elimination
  (top-intro)            truth introduction — Γ ⊢ ⊤ in empty context
  (top-intro N)          truth introduction in step N's context
  (extend F N)           weakening: add formula F to step N's context
  (box-intro N)          necessitation (NEC) — from ∅⊢A derive ∅⊢□A
  (box-elim N M)         K axiom — from □(A→B) and □A derive □B
  (diamond-def N)        ◇-definition — from ◇A derive ¬□¬A
  (diamond-def-rev N)    reverse ◇-definition — from ¬□¬A derive ◇A
  (subst N (A F)...)    substitute atom A with formula F in step N
  (show N)              re-print step N
  (and A B) ..etc       parse a formula (just prints it)

Formula syntax:
  A, B, p, q            atoms
  T                      truth
  _|_                    falsity (also: bottom, ⊥)
  (not F)                negation
  (and F G)              conjunction
  (or F G)               disjunction
  (-> F G)               implication
  (box F)                necessity (□F)
  (diamond F)            possibility (◇F)
  (forall x F)           universal quantification
  (exists x F)           existential quantification
  (App P t)              predicate application

Atoms are terms (lowercase) or propositions (uppercase) by convention.
Aliases: mp=->-elim, raa=not-intro, efq=ex-falso, imp-intro=->-intro"
            .to_string()
    }
}
