use std::rc::Rc;

use crate::formula::PropWWF;
use super::sexpr::SExpr;

/// Parse a formula from an s-expression.
pub fn parse_formula(sexpr: &SExpr) -> Result<Rc<PropWWF>, String> {
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
                    Ok(Rc::new(PropWWF::Neg(parse_formula(&items[1])?)))
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
                        parse_formula(&items[1])?,
                        parse_formula(&items[2])?,
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
                        parse_formula(&items[1])?,
                        parse_formula(&items[2])?,
                    )))
                }
                "forall" | "∀" => {
                    if items.len() != 3 {
                        return Err(format!("'{}' expects 2 arguments (var body), got {}", head, items.len() - 1));
                    }
                    let var = match &items[1] {
                        SExpr::Atom(s) => s.clone(),
                        _ => return Err("forall first argument must be an atom".into()),
                    };
                    Ok(Rc::new(PropWWF::Forall(var, parse_formula(&items[2])?)))
                }
                "exists" | "∃" => {
                    if items.len() != 3 {
                        return Err(format!("'{}' expects 2 arguments (var body), got {}", head, items.len() - 1));
                    }
                    let var = match &items[1] {
                        SExpr::Atom(s) => s.clone(),
                        _ => return Err("exists first argument must be an atom".into()),
                    };
                    Ok(Rc::new(PropWWF::Exists(var, parse_formula(&items[2])?)))
                }
                "App" => {
                    if items.len() != 3 {
                        return Err(format!("'App' expects 2 arguments, got {}", items.len() - 1));
                    }
                    Ok(Rc::new(PropWWF::App(
                        parse_formula(&items[1])?,
                        parse_formula(&items[2])?,
                    )))
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
                        parse_formula(&items[1])?,
                        parse_formula(&items[2])?,
                    )))
                }
                _ => Err(format!("unknown formula constructor: '{}'", head)),
            }
        }
    }
}
