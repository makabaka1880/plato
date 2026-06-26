use std::rc::Rc;

use crate::formula::PropWWF;
use super::sexpr::SExpr;

/// Parse a formula from an s-expression.
/// `mode` gates constructors: `"pl"` rejects quantifiers, `"fol"` rejects modal operators.
pub fn parse_formula(sexpr: &SExpr, mode: Option<&str>) -> Result<Rc<PropWWF>, String> {
    match sexpr {
        SExpr::Atom(s) => match s.as_str() {
            "T" | "⊤" => Ok(Rc::new(PropWWF::Top)),
            "_|_" | "⊥" | "bottom" | "false" => Ok(Rc::new(PropWWF::Bottom)),
            _ => Ok(Rc::new(PropWWF::Atom(s.clone()))),
        },
        SExpr::List(items) => {
            if items.is_empty() {
                return Err("empty list is not valid".into());
            }
            let head = match &items[0] {
                SExpr::Atom(s) => s.as_str(),
                _ => return Err("form constructor must be a symbol".into()),
            };
            match head {
                "not" | "neg" | "¬" => {
                    if items.len() != 2 {
                        return Err(format!(
                            "'{}' expects 1 argument, got {}",
                            head,
                            items.len() - 1
                        ));
                    }
                    Ok(Rc::new(PropWWF::Neg(parse_formula(&items[1], mode)?)))
                }
                "and" | "∧" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "'{}' expects 2 arguments, got {}",
                            head,
                            items.len() - 1
                        ));
                    }
                    Ok(Rc::new(PropWWF::Conj(
                        parse_formula(&items[1], mode)?,
                        parse_formula(&items[2], mode)?,
                    )))
                }
                "or" | "∨" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "'{}' expects 2 arguments, got {}",
                            head,
                            items.len() - 1
                        ));
                    }
                    Ok(Rc::new(PropWWF::Disj(
                        parse_formula(&items[1], mode)?,
                        parse_formula(&items[2], mode)?,
                    )))
                }
                "forall" | "∀" => {
                    if mode == Some("pl") {
                        return Err("forall (∀) is not available in PL mode — switch to FOL mode".into());
                    }
                    if items.len() != 3 {
                        return Err(format!("'{}' expects 2 arguments (var body), got {}", head, items.len() - 1));
                    }
                    let var = match &items[1] {
                        SExpr::Atom(s) => s.clone(),
                        _ => return Err("forall first argument must be an atom".into()),
                    };
                    Ok(Rc::new(PropWWF::Forall(var, parse_formula(&items[2], mode)?)))
                }
                "exists" | "∃" => {
                    if mode == Some("pl") {
                        return Err("exists (∃) is not available in PL mode — switch to FOL mode".into());
                    }
                    if items.len() != 3 {
                        return Err(format!("'{}' expects 2 arguments (var body), got {}", head, items.len() - 1));
                    }
                    let var = match &items[1] {
                        SExpr::Atom(s) => s.clone(),
                        _ => return Err("exists first argument must be an atom".into()),
                    };
                    Ok(Rc::new(PropWWF::Exists(var, parse_formula(&items[2], mode)?)))
                }
                "App" => {
                    if mode == Some("pl") {
                        return Err("App (predicate application) is not available in PL mode — switch to FOL mode".into());
                    }
                    if items.len() < 3 {
                        return Err(format!("'App' expects at least 2 arguments, got {}", items.len() - 1));
                    }
                    // Chained application: (App P a b c …)  ≡  (App (…(App (App P a) b)…) …)
                    let mut result = Rc::new(PropWWF::App(
                        parse_formula(&items[1], mode)?,
                        parse_formula(&items[2], mode)?,
                    ));
                    for arg in &items[3..] {
                        result = Rc::new(PropWWF::App(result, parse_formula(arg, mode)?));
                    }
                    Ok(result)
                }
                "box" | "□" | "nec" => {
                    if mode == Some("fol") {
                        return Err("box (□) is not available in FOL mode — switch to PL mode".into());
                    }
                    if items.len() != 2 {
                        return Err(format!(
                            "'{}' expects 1 argument, got {}",
                            head,
                            items.len() - 1
                        ));
                    }
                    Ok(Rc::new(PropWWF::Box(parse_formula(&items[1], mode)?)))
                }
                "diamond" | "◇" | "dia" | "poss" => {
                    if mode == Some("fol") {
                        return Err("diamond (◇) is not available in FOL mode — switch to PL mode".into());
                    }
                    if items.len() != 2 {
                        return Err(format!(
                            "'{}' expects 1 argument, got {}",
                            head,
                            items.len() - 1
                        ));
                    }
                    Ok(Rc::new(PropWWF::Diamond(parse_formula(&items[1], mode)?)))
                }
                "->" | "→" | "imp" | "⊃" => {
                    if items.len() != 3 {
                        return Err(format!(
                            "'{}' expects 2 arguments, got {}",
                            head,
                            items.len() - 1
                        ));
                    }
                    Ok(Rc::new(PropWWF::Imp(
                        parse_formula(&items[1], mode)?,
                        parse_formula(&items[2], mode)?,
                    )))
                }
                _ => Err(format!("unknown formula constructor: '{}'", head)),
            }
        }
    }
}
