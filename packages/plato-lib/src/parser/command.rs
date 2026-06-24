use std::rc::Rc;

use crate::formula::PropWWF;
use super::formula::parse_formula;
use super::sexpr::SExpr;

/// A parsed proof command.
#[derive(Debug, Clone)]
pub enum Command {
    /// Parse a formula and print it (no proof step generated).
    Formula(Rc<PropWWF>),
    /// `(assume F)` — create `{F} ⊢ F`.
    Assume(Rc<PropWWF>),
    /// `(var n F)` — derive `F` from step `n`'s context (if F ∈ ctx).
    Var(usize, Rc<PropWWF>),
    /// `(and-intro n m)` — conjunction introduction.
    AndIntro(usize, usize),
    /// `(and-elim-l n)` — conjunction elimination left.
    AndElimL(usize),
    /// `(and-elim-r n)` — conjunction elimination right.
    AndElimR(usize),
    /// `(or-intro-l n F)` — disjunction introduction left.
    OrIntroL(usize, Rc<PropWWF>),
    /// `(or-intro-r n F)` — disjunction introduction right.
    OrIntroR(usize, Rc<PropWWF>),
    /// `(or-elim n j k)` — disjunction elimination.
    OrElim(usize, usize, usize),
    /// `(->-intro F n)` — implication introduction (discharge F).
    ImpIntro(Rc<PropWWF>, usize),
    /// `(->-elim n m)` — modus ponens.
    ImpElim(usize, usize),
    /// `(->-into n)` — implication unpack.
    ImpInto(usize),
    /// `(not-intro F n m)` — negation introduction.
    NotIntro(Rc<PropWWF>, usize, usize),
    /// `(not-elim n)` — negation elimination.
    NotElim(usize),
    /// `(dne n)` — double negation elimination.
    DNegElim(usize),
    /// `(ex-falso n F)` — ex falso quodlibet.
    ExFalso(usize, Rc<PropWWF>),
    /// `(show n)` — re-print a previous step.
    Show(usize),
}

// ── Helpers ─────────────────────────────────────────────────────

fn expect_usize(sexpr: &SExpr) -> Result<usize, String> {
    match sexpr {
        SExpr::Atom(s) => s
            .parse::<usize>()
            .map_err(|_| format!("expected a step number, got '{}'", s)),
        _ => Err("expected a step number".into()),
    }
}

// ── Command parsing ─────────────────────────────────────────────

fn parse_command(sexpr: &SExpr) -> Result<Command, String> {
    match sexpr {
        // A bare atom is always a formula
        SExpr::Atom(_) => Ok(Command::Formula(parse_formula(sexpr)?)),
        SExpr::List(items) => {
            if items.is_empty() {
                return Err("empty list".into());
            }
            let head = match &items[0] {
                SExpr::Atom(s) => s.as_str(),
                _ => return Err("command must start with a symbol".into()),
            };
            match head {
                "assume" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "assume expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::Assume(parse_formula(&items[1])?))
                }
                "var" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "var expects 2 arguments (step, formula), got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::Var(
                        expect_usize(&items[1])?,
                        parse_formula(&items[2])?,
                    ))
                }
                "and-intro" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "and-intro expects 2 arguments, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::AndIntro(
                        expect_usize(&items[1])?,
                        expect_usize(&items[2])?,
                    ))
                }
                "and-elim-l" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "and-elim-l expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::AndElimL(expect_usize(&items[1])?))
                }
                "and-elim-r" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "and-elim-r expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::AndElimR(expect_usize(&items[1])?))
                }
                "or-intro-l" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "or-intro-l expects 2 arguments, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::OrIntroL(
                        expect_usize(&items[1])?,
                        parse_formula(&items[2])?,
                    ))
                }
                "or-intro-r" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "or-intro-r expects 2 arguments, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::OrIntroR(
                        expect_usize(&items[1])?,
                        parse_formula(&items[2])?,
                    ))
                }
                "or-elim" => {
                    if items.len() != 4 {
                        return Err(format!(
                            "or-elim expects 3 arguments, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::OrElim(
                        expect_usize(&items[1])?,
                        expect_usize(&items[2])?,
                        expect_usize(&items[3])?,
                    ))
                }
                "->-intro" | "imp-intro" | "→-intro" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "->-intro expects 2 arguments (formula, step), got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::ImpIntro(
                        parse_formula(&items[1])?,
                        expect_usize(&items[2])?,
                    ))
                }
                "->-elim" | "imp-elim" | "→-elim" | "modus-ponens" | "mp" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "->-elim expects 2 arguments, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::ImpElim(
                        expect_usize(&items[1])?,
                        expect_usize(&items[2])?,
                    ))
                }
                "->-into" | "imp-into" | "→-into" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "->-into expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::ImpInto(expect_usize(&items[1])?))
                }
                "not-intro" | "¬-intro" | "neg-intro" | "raa" => {
                    if items.len() != 4 {
                        return Err(format!(
                            "not-intro expects 3 arguments (formula, step, step), got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::NotIntro(
                        parse_formula(&items[1])?,
                        expect_usize(&items[2])?,
                        expect_usize(&items[3])?,
                    ))
                }
                "not-elim" | "¬-elim" | "neg-elim" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "not-elim expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::NotElim(expect_usize(&items[1])?))
                }
                "dne" | "dneg-elim" | "¬¬-elim" | "double-neg-elim" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "dne expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::DNegElim(expect_usize(&items[1])?))
                }
                "ex-falso" | "efq" | "explosion" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "ex-falso expects 2 arguments (step, formula), got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::ExFalso(
                        expect_usize(&items[1])?,
                        parse_formula(&items[2])?,
                    ))
                }
                "show" | "print" | "p" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "show expects 1 argument, got {}",
                            items.len() - 1
                        ));
                    }
                    Ok(Command::Show(expect_usize(&items[1])?))
                }
                "help" | "?" => Ok(Command::Show(0)), // special — prints help
                // Fallback: try parsing as a formula
                _ => match parse_formula(sexpr) {
                    Ok(f) => Ok(Command::Formula(f)),
                    Err(_) => Err(format!(
                        "unknown command '{}'. Try typing an s-expression formula, or 'help'.",
                        head
                    )),
                },
            }
        }
    }
}

/// Parse an input string into a command.
pub fn parse_input(s: &str) -> Result<Command, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty input".into());
    }
    let tokens = super::sexpr::tokenize(s);
    if tokens.is_empty() {
        return Err("empty input".into());
    }
    let sexpr = super::sexpr::parse_one(&tokens)?;
    parse_command(&sexpr)
}
