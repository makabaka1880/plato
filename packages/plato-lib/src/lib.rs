use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod context;
pub mod formula;
pub mod inference;
pub mod judgement;
mod parser;
mod rules;

// ── Wasm-bindgen wrappers ──────────────────────────────────────

/// A propositional well-formed formula.
#[wasm_bindgen]
pub struct Formula(Rc<formula::PropWWF>);

#[wasm_bindgen]
impl Formula {
    pub fn top() -> Self {
        Self(Rc::new(formula::PropWWF::Top))
    }
    pub fn bottom() -> Self {
        Self(Rc::new(formula::PropWWF::Bottom))
    }
    pub fn atom(name: &str) -> Self {
        Self(Rc::new(formula::PropWWF::Atom(name.to_string())))
    }
    pub fn neg(p: &Formula) -> Self {
        Self(Rc::new(formula::PropWWF::Neg(p.0.clone())))
    }
    pub fn conj(p: &Formula, q: &Formula) -> Self {
        Self(Rc::new(formula::PropWWF::Conj(p.0.clone(), q.0.clone())))
    }
    pub fn disj(p: &Formula, q: &Formula) -> Self {
        Self(Rc::new(formula::PropWWF::Disj(p.0.clone(), q.0.clone())))
    }
    pub fn imp(antecedent: &Formula, consequent: &Formula) -> Self {
        Self(Rc::new(formula::PropWWF::Imp(
            antecedent.0.clone(),
            consequent.0.clone(),
        )))
    }
    pub fn box_(p: &Formula) -> Self {
        Self(Rc::new(formula::PropWWF::Box(p.0.clone())))
    }
    pub fn diamond(p: &Formula) -> Self {
        Self(Rc::new(formula::PropWWF::Diamond(p.0.clone())))
    }
    pub fn equals(&self, other: &Formula) -> bool {
        self.0 == other.0
    }
}

#[wasm_bindgen]
impl Formula {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Natural-language description: "A and B", "if A then B", etc.
    pub fn verbal(&self) -> String {
        self.0.verbal()
    }

    /// LaTeX representation: `A \\land B`, `\\lnot A`, `A \\to B`, etc.
    pub fn latex(&self) -> String {
        self.0.latex()
    }
}

// ── Context ────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct Ctx(context::Context);

#[wasm_bindgen]
impl Ctx {
    pub fn new() -> Self {
        Self(context::Context::new())
    }
    pub fn insert(&mut self, f: &Formula) {
        self.0.insert(f.0.as_ref().clone());
    }
    pub fn contains(&self, f: &Formula) -> bool {
        self.0.contains(&f.0)
    }
    pub fn decompose(&self, f: &Formula) -> Option<Ctx> {
        self.0.decompose(&f.0).map(Ctx)
    }
    pub fn union_with(&self, other: &Ctx) -> Ctx {
        Ctx(self.0.union_with(&other.0))
    }
}

#[wasm_bindgen]
impl Ctx {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// ── Judgement ──────────────────────────────────────────────────

#[wasm_bindgen]
pub struct Judgement(Rc<judgement::Judgement>);

#[wasm_bindgen]
impl Judgement {
    pub fn ctx(&self) -> Ctx {
        Ctx(self.0.ctx.clone())
    }
    pub fn formula(&self) -> Formula {
        Formula(self.0.prop.clone())
    }
}

#[wasm_bindgen]
impl Judgement {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Natural-language description: "A and B entails A or B", etc.
    pub fn verbal(&self) -> String {
        self.0.verbal()
    }

    /// LaTeX representation: `\\{A, B\\} \\vdash A \\land B`.
    pub fn latex(&self) -> String {
        self.0.latex()
    }
}

// ── Prover ─────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct Prover;

#[wasm_bindgen]
impl Prover {
    pub fn var_intro(ctx: &Ctx, x: &Formula) -> Option<Judgement> {
        rules::misc::var_intro(&ctx.0, x.0.clone()).map(|j| Judgement(Rc::new(j)))
    }
    pub fn conj_intro(jl: &Judgement, jr: &Judgement) -> Option<Judgement> {
        rules::conj::conj_intro(&jl.0, &jr.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn conj_elim_lhs(jc: &Judgement) -> Option<Judgement> {
        rules::conj::conj_elim_lhs(&jc.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn conj_elim_rhs(jc: &Judgement) -> Option<Judgement> {
        rules::conj::conj_elim_rhs(&jc.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn disj_intro_lhs(j: &Judgement, pr: &Formula) -> Option<Judgement> {
        rules::disj::disj_intro_lhs(&j.0, pr.0.clone()).map(|j| Judgement(Rc::new(j)))
    }
    pub fn disj_intro_rhs(j: &Judgement, pl: &Formula) -> Option<Judgement> {
        rules::disj::disj_intro_rhs(&j.0, pl.0.clone()).map(|j| Judgement(Rc::new(j)))
    }
    pub fn disj_elim(j: &Judgement, jl: &Judgement, jr: &Judgement) -> Option<Judgement> {
        rules::disj::disj_elim(&j.0, &jl.0, &jr.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn imp_intro(p_ant: &Formula, j: &Judgement) -> Option<Judgement> {
        rules::imp::imp_intro(p_ant.0.clone(), &j.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn imp_into(j: &Judgement) -> Option<Judgement> {
        rules::imp::imp_into(&j.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn imp_elim(j: &Judgement, je: &Judgement) -> Option<Judgement> {
        rules::imp::imp_elim(&j.0, &je.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn neg_intro(np: &Formula, j: &Judgement, nj: &Judgement) -> Option<Judgement> {
        rules::neg::neg_intro(np.0.clone(), &j.0, &nj.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn neg_elim(nj: &Judgement) -> Option<Judgement> {
        rules::neg::neg_elim(&nj.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn double_neg_elim(j: &Judgement) -> Option<Judgement> {
        rules::neg::double_neg_elim(&j.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn ex_falso(j_bot: &Judgement, p: &Formula) -> Option<Judgement> {
        rules::misc::exfalso(&j_bot.0, p.0.clone()).map(|j| Judgement(Rc::new(j)))
    }
    pub fn box_intro(j: &Judgement) -> Option<Judgement> {
        rules::modal::box_intro(&j.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn box_elim(j_imp: &Judgement, j_box_a: &Judgement) -> Option<Judgement> {
        rules::modal::box_elim(&j_imp.0, &j_box_a.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn diamond_def_fwd(j: &Judgement) -> Option<Judgement> {
        rules::modal::diamond_def_fwd(&j.0).map(|j| Judgement(Rc::new(j)))
    }
    pub fn diamond_def_rev(j: &Judgement) -> Option<Judgement> {
        rules::modal::diamond_def_rev(&j.0).map(|j| Judgement(Rc::new(j)))
    }
}

// ── Command metadata ────────────────────────────────────────────

/// Parsed metadata for an s-expression command — its canonical name
/// and a JSON string of template parameters.
#[wasm_bindgen]
pub struct CmdMeta {
    cmd: String,
    params_json: String,
}

#[wasm_bindgen]
impl CmdMeta {
    /// Canonical command name, e.g. `"->-intro"`, `"and-elim-l"`.
    #[wasm_bindgen(getter)]
    pub fn cmd_name(&self) -> String {
        self.cmd.clone()
    }

    /// JSON string of parameter key→value pairs, e.g. `{"n":"1","F":"I"}`.
    /// Parse with `JSON.parse()` on the JS side.
    #[wasm_bindgen(getter)]
    pub fn params_json(&self) -> String {
        self.params_json.clone()
    }
}

// ── Session (s-expression REPL) ─────────────────────────────────

/// A proof session driven by s-expression commands.
///
/// Each call to `execute` parses and runs one command. Successful
/// proof steps are numbered sequentially (1-indexed).
#[wasm_bindgen]
pub struct Session(parser::session::Session);

#[wasm_bindgen]
impl Session {
    /// Start a fresh session.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(parser::session::Session::new())
    }

    /// Parse an s-expression command without executing it, returning
    /// structured metadata (command name + template parameters).
    /// Returns `undefined` (null in JS) on parse error.
    pub fn parse_meta(&self, input: &str) -> Option<CmdMeta> {
        use parser::command::parse_input;
        let cmd = parse_input(input, self.0.mode()).ok()?;
        let (name, params) = cmd.meta();
        // Manual JSON serialization — avoid pulling in serde_json
        let mut json = String::from("{");
        for (i, (k, v)) in params.iter().enumerate() {
            if i > 0 { json.push(','); }
            json.push('"');
            json.push_str(k);
            json.push_str("\":\"");
            json.push_str(&v.replace('\\', "\\\\").replace('"', "\\\""));
            json.push('"');
        }
        json.push('}');
        Some(CmdMeta { cmd: name, params_json: json })
    }

    /// Execute one s-expression command. Returns the formatted result
    /// (or an error message starting with `Error:`).
    pub fn execute(&mut self, input: &str) -> String {
        match self.0.execute(input) {
            Ok(out) => out,
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Returns the number of steps in the proof so far.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Raw text of step `n` (1-indexed), or `undefined` if out of range.
    pub fn step_text(&self, n: usize) -> Option<String> {
        self.0.step_text(n)
    }

    /// LaTeX for step `n` (1-indexed), or `undefined` if out of range.
    pub fn step_latex(&self, n: usize) -> Option<String> {
        self.0.step_latex(n)
    }

    /// Just the conclusion formula's LaTeX for step `n`, or `undefined`.
    #[wasm_bindgen(js_name = stepFormulaLatex)]
    pub fn step_formula_latex(&self, n: usize) -> Option<String> {
        self.0.step_formula_latex(n)
    }

    /// Parse an s-expression formula and return its LaTeX.
    /// e.g. `"(and A B)"` → `"A \\land B"`.
    #[wasm_bindgen(js_name = formulaLatex)]
    pub fn formula_latex(s: &str) -> String {
        parser::session::Session::formula_latex(s)
    }

    /// Returns `true` if two tactic strings are structurally equal
    /// (normalises whitespace, compares ASTs).
    #[wasm_bindgen(js_name = tacticEquals)]
    pub fn tactic_equals(a: &str, b: &str) -> bool {
        parser::sexpr::tactic_equals(a, b)
    }

    /// Returns `true` if the last step proves the goal (empty context,
    /// conclusion equals the parsed goal formula).
    #[wasm_bindgen(js_name = isGoalResolved)]
    pub fn is_goal_resolved(&self, goal: &str) -> bool {
        self.0.last_step_satisfies_goal(goal)
    }

    /// Set the logic mode. `"pl"` = propositional (modal allowed, quantifiers blocked).
    /// `"fol"` = first-order (quantifiers allowed, modal blocked).
    #[wasm_bindgen(js_name = setMode)]
    pub fn set_mode(&mut self, mode: &str) {
        self.0.set_mode(mode);
    }
}
