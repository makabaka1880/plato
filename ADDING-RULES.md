# Adding Inference Rules to Plato

This guide documents every step needed to add a new inference rule (command/tactic) to Plato, from the Rust proof engine to the Vue frontend. Use this as a checklist — there are **17-20 files** to touch across the stack.

## Table of Contents

1. [Overview](#overview)
2. [Rust: The rule function](#rust-the-rule-function)
3. [Rust: Parser registration](#rust-parser-registration)
4. [Rust: Session execution](#rust-session-execution)
5. [Rust: Inference engine](#rust-inference-engine)
6. [Rust: WASM bindings](#rust-wasm-bindings)
7. [Frontend: Tactics data](#frontend-tactics-data)
8. [Frontend: NLG templates](#frontend-nlg-templates)
9. [Frontend: i18n strings](#frontend-i18n-strings)
10. [Frontend: HelpModal](#frontend-helpmodal)
11. [Frontend: Autocomplete](#frontend-autocomplete)
12. [Frontend: Section configuration](#frontend-section-configuration)
13. [Frontend: Unlocking the tactic](#frontend-unlocking-the-tactic)
14. [Problem updates](#problem-updates)
15. [Verification checklist](#verification-checklist)

---

## Overview

An inference rule in Plato is a **pure function** that takes one or more `Judgement` premises and computes a new `Judgement` (or `None` if preconditions aren't met). The rule is exposed to users as an **s-expression command** in the REPL (e.g., `(extend A 1)`, `(->-intro P 3)`).

The full pipeline:

```
User types s-expression in REPL
  → parser/command.rs parses it into a Command enum variant
  → parser/session.rs calls the rule function with the parsed arguments
  → rules/*.rs rule function validates and returns a Judgement
  → The new Judgement is pushed onto the proof stack
  → Frontend reads the session state and displays the result
```

Convention: The **rule function** lives in `rules/` (grouped by connective). The **command parsing** lives in `parser/command.rs`. The **execution dispatch** lives in `parser/session.rs`.

---

## Rust: The rule function

**File:** `packages/plato-lib/src/rules/misc.rs` (or a new file in `rules/`)

Every rule is a standalone `pub fn` returning `Option<Judgement>`. It never panics — validation is via `Option`.

### Signature patterns

| Type | Signature | Example |
|---|---|---|
| 0-premise (axiom) | `fn(ctx: &Context) -> Option<Judgement>` | `top_intro` |
| 1-premise + formula | `fn(j: &Judgement, f: Rc<PropWWF>) -> Option<Judgement>` | `exfalso`, `extend` |
| 1-premise | `fn(j: &Judgement) -> Option<Judgement>` | `imp_into` |
| 2-premise | `fn(j1: &Judgement, j2: &Judgement) -> Option<Judgement>` | `conj_intro`, `imp_elim` |
| 3-premise | `fn(j1: &Judgement, j2: &Judgement, j3: &Judgement) -> Option<Judgement>` | `disj_elim` |

### Template

```rust
/// Short doc comment describing the rule.
///
/// Γ ⊢ p, some condition ⇒ Γ' ⊢ q.
pub fn my_rule(j: &Judgement, extra: Rc<PropWWF>) -> Option<Judgement> {
    // 1. Validate preconditions
    // 2. Clone and modify the context as needed
    let mut ctx = j.ctx.clone();
    ctx.insert(extra.as_ref().clone());
    // 3. Construct the new judgement
    Some(Judgement::new(ctx, j.prop.clone()))
}
```

### Key types

- **`Judgement`** (`judgement.rs`): `{ ctx: Context, prop: Rc<PropWWF> }` — a sequent `Γ ⊢ p`.
- **`Context`** (`context.rs`): `HashSet<PropWWF>` behind a newtype. Methods: `new()`, `insert()`, `contains()`, `decompose()`, `union_with()`, `clone()`, `substitute()`.
- **`PropWWF`** (`formula.rs`): Enum with variants `Top`, `Bottom`, `Atom(String)`, `Neg(Rc<PropWWF>)`, `Conj(…)`, `Disj(…)`, `Imp(…)`, `Forall(String, …)`, `Exists(String, …)`, `App(…)`, `Box(…)`, `Diamond(…)`.

### Registration in `rules/mod.rs`

If you created a new file, add `pub mod your_file;`. If adding to an existing file (like `misc.rs`), no change needed.

---

## Rust: Parser registration

**File:** `packages/plato-lib/src/parser/command.rs`

Three changes in one file:

### 1. Add a variant to the `Command` enum

```rust
/// `(my-cmd F N)` — does something.
MyCmd(Rc<PropWWF>, usize),
```

Name the variant after the command (PascalCase). Store parsed arguments directly (formulas as `Rc<PropWWF>`, step numbers as `usize`, variable names as `String`).

### 2. Add a parser arm in `parse_command()`

```rust
"my-cmd" | "my-cmd-alias" => {
    if items.len() != 3 {
        return Err(format!(
            "my-cmd expects 2 arguments (formula, step), got {}",
            items.len() - 1
        ));
    }
    Ok(Command::MyCmd(
        parse_formula(&items[1], mode)?,
        expect_usize(&items[2])?,
    ))
}
```

**Conventions:**
- Use `parse_formula(&items[N], mode)?` for formula arguments.
- Use `expect_usize(&items[N])?` for step numbers.
- For variable names, match `SExpr::Atom(s) => s.clone()`.
- Provide aliases separated by `|` (e.g., `"->-elim" | "mp"`).
- Gate mode-specific commands with `if mode == Some("pl") { return Err(...) }`.

**Parser helpers available:**
- `parse_formula(sexpr, mode) -> Result<Rc<PropWWF>>` — parses any formula.
- `expect_usize(sexpr) -> Result<usize>` — extracts a step number.
- `SExpr::Atom(s)` / `SExpr::List(items)` — pattern match the AST.

### 3. Add a `meta()` arm

```rust
Command::MyCmd(f, n) => ("my-cmd".into(), vec![
    ("F".into(), f.latex()),
    ("n".into(), n.to_string()),
]),
```

The `meta()` method returns `(canonical_name, [(param_key, param_value)])`. All values are strings. Formula parameters use `f.latex()`; step numbers use `n.to_string()`; variable names are raw strings. The canonical name must match the key in `tactics.json` and `nlg.ts`.

**Parameter key conventions:**
- `F` — a formula
- `n`, `m`, `k` — step numbers
- `x`, `t` — variable/term names

---

## Rust: Session execution

**File:** `packages/plato-lib/src/parser/session.rs`

### Add an `execute()` arm

```rust
Command::MyCmd(f, n) => {
    let jn = self.step(n)?;                          // fetch premise
    let j = rules::misc::my_rule(jn, f)              // apply rule
        .ok_or_else(|| "my-cmd failed: reason".to_string())?;
    self.steps.push(Rc::new(j));                     // record new step
    self.last_fmt()                                   // format output
}
```

Pattern: `self.step(n)?` for each premise (auto-validates step number), then call your rule function, handle the `Option` with `.ok_or_else(|| "...")`, push the result, and return `self.last_fmt()`.

The error message is shown to the user — make it helpful. Mention what the rule expects and what went wrong.

### Add a help line

In the `help()` method, add a line in alphabetical-ish order:

```
  (my-cmd F N)          does something useful
```

---

## Rust: Inference engine

**File:** `packages/plato-lib/src/inference.rs`

The `DeductionRule` enum provides a programmatic (non-s-expression) API for the proof engine. Each variant carries the premises needed.

### Add a variant

```rust
/// My command: Γ ⊢ p ⇒ Γ, q ⊢ p.
MyCmd(Rc<Judgement>, Rc<PropWWF>),
```

### Add a `deduce()` arm

```rust
Self::MyCmd(j, q) => rules::misc::my_rule(&j, q),
```

---

## Rust: WASM bindings

**File:** `packages/plato-lib/src/lib.rs`

Add a method to the `Prover` struct in the `impl Prover` block:

```rust
pub fn my_cmd(j: &Judgement, q: &Formula) -> Option<Judgement> {
    rules::misc::my_rule(&j.0, q.0.clone()).map(|j| Judgement(Rc::new(j)))
}
```

Wrap the Rust types in their WASM-friendly counterparts (`Judgement` wraps `Rc<judgement::Judgement>`, `Formula` wraps `Rc<PropWWF>`). Use `.0` to unwrap, `.clone()` for `Rc`, and `.map(|j| Judgement(Rc::new(j)))` to wrap the result.

---

## Frontend: Tactics data

**Files:** `packages/plato-site/src/data/en/tactics.json` and `packages/plato-site/src/data/zh/tactics.json`

Add an entry to both locale files:

```json
{
    "name": "my-cmd",
    "description": "Human-readable description. Use $\\LaTeX$ for formulas.",
    "syntax": "(my-cmd F N)",
    "example": "1. {P} ⊢ Q\n2. (my-cmd R 1)  →  {P, R} ⊢ Q",
    "rule": "\\frac{\\Gamma \\vdash p}{\\Gamma, q \\vdash p}"
}
```

| Field | Purpose |
|---|---|
| `name` | Canonical command name — matches the `meta()` canonical name and `nlg.ts` key |
| `description` | What the rule does. Supports inline LaTeX (`$...$`) |
| `syntax` | User-facing command syntax with placeholder names |
| `example` | A concrete worked example showing input → output |
| `rule` | KaTeX fraction for the inference rule notation |

**Placement:** Group with similar rules (structural rules with structural, connectives with their group).

---

## Frontend: NLG templates

**Files:** `packages/plato-site/src/data/en/nlg.ts` and `packages/plato-site/src/data/zh/nlg.ts`

Add a natural language template keyed by the canonical command name:

```typescript
// en/nlg.ts
'my-cmd': 'We add {F} to the context of step {n}, and {conclusion} still follows.',

// zh/nlg.ts
'my-cmd': '在第 {n} 行的上下文里多放一个 {F}，{conclusion} 照样成立。',
```

**Template variables:**
- `{F}`, `{n}`, `{m}`, `{k}`, `{x}`, `{t}` — match the `meta()` param keys
- `{conclusion}` — auto-injected by the frontend (the conclusion of the resulting judgement)

Formula keys (`F`, `conclusion`) are automatically wrapped in `$...$` for KaTeX rendering. Others are rendered as plain text.

---

## Frontend: i18n strings

**Files:** `packages/plato-site/src/i18n/locales/en.ts` and `packages/plato-site/src/i18n/locales/zh.ts`

Add an entry in `help.commands`:

```typescript
// en.ts — inside help.commands
'my-cmd': 'Brief one-line description of the command for the Help modal',

// zh.ts — inside help.commands
'my-cmd': '命令的简短中文描述',
```

---

## Frontend: HelpModal

**File:** `packages/plato-site/src/components/HelpModal.vue`

Add an entry to the appropriate `groups` array. Each group has an `i18nGroup` key and an `entries` array:

```typescript
{
    i18nGroup: 'help.groups.basics',  // pick the right group
    entries: [
        // ... existing entries ...
        {
            syntax: '(my-cmd F N)',
            i18nKey: 'help.commands.my-cmd',
            unlocked: store.collected.includes('my-cmd'),  // gated by unlock
            rule: '\\frac{\\Gamma \\vdash p}{\\Gamma, q \\vdash p}',  // optional
        },
    ],
},
```

**Choosing a group:**
- `basics` — structural rules (assume, extend, fix, subst, show)
- `implication` — → rules
- `conjunction` — ∧ rules
- `disjunction` — ∨ rules
- `negation` — ¬ rules
- `quantifiers` — ∀, ∃ rules
- `modal` — □, ◇ rules

If `unlocked` is `false` and no `rule` is provided, the entry shows with reduced opacity and a lock emoji. Once the tactic is collected, it becomes fully visible with the rule notation.

---

## Frontend: Autocomplete

**File:** `packages/plato-site/src/composables/useAutocomplete.ts`

Add an entry to the `STATIC` array:

```typescript
{ label: 'my-cmd', insert: 'my-cmd ', description: 'Brief description for autocomplete popup', kind: 'command' },
```

**Fields:**
- `label` — what the user types to match (lowercase, no spaces)
- `insert` — what gets inserted on accept (include trailing space)
- `description` — shown in the autocomplete dropdown
- `kind` — `'command'` for tactics, `'formula'` for connectives

The autocomplete filter (line ~96-118 in `useAutocomplete.ts`) gates commands by `allowedTactics`. If your new command is in the section's `allowedTactics` array, it will appear when the user types its prefix. No additional filter logic needed for standard commands.

---

## Frontend: Section configuration

**Files (one per section):**
- `packages/plato-site/src/data/en/sections/propositional/section.json`
- `packages/plato-site/src/data/en/sections/first-order/section.json`
- `packages/plato-site/src/data/en/sections/modal/section.json`

Add the command name to the `allowedTactics` array in each section where it should be available:

```json
{
    "allowedTactics": [
        "assume",
        "my-cmd",
        "ex-falso",
        ...
    ]
}
```

This controls:
1. Whether the command appears in the autocomplete for that section
2. Whether the HelpModal shows it as relevant (groups with no unlocked tactics are hidden)
3. Whether the command parser in the session accepts it (mode gating happens in the parser)

---

## Frontend: Unlocking the tactic

Choose an early problem where the tactic is introduced, and add its name to that problem's `unlocks` array. The full metadata is looked up from `tactics.json` at runtime — you only need the name.

**Files:** `packages/plato-site/src/data/{en,zh}/sections/{section}/problems/NN-problem.json`

```json
"unlocks": ["my-cmd"]
```

**Best practices for choosing an unlock location:**
- **Structural rules** (like assume, extend, subst): unlock in the very first problem (01-identity) — players should have them from the start.
- **Connective rules**: unlock in the problem that first teaches that connective.
- Each problem can unlock multiple tactics: `"unlocks": ["or-intro-l", "or-intro-r"]`

When the player solves the problem, `useVictory.ts` iterates over the `unlocks` strings and calls `tacticsStore.add(name)` for each, which persists to `localStorage` under key `plato-tactics`. The HelpModal, autocomplete, and tactic sidebar all react to this store.

---

## Problem updates

When you add a new rule that simplifies existing proof patterns, update the problem set:

1. **Identify the pattern** — e.g., `and-intro N M` followed immediately by `and-elim-r` or `and-elim-l` is often a hack for context extension (weakening).
2. **Replace with the new rule** — Update the hint tactics.
3. **Renumber carefully** — If the new approach uses fewer steps, all subsequent step number references in tactics MUST be updated. This is the most error-prone part.
4. **Update hint text** — Explain the new approach.
5. **Update both locales** — en/ and zh/ must stay in sync.

**Scripting hint updates:** For complex renumbering across many hints, use a Python script to manipulate the JSON programmatically (see the `extend` rule implementation for examples).

**When NOT to update:** If the `and-intro + and-elim` pattern is genuinely used to decompose an actual conjunction (not just merge contexts), leave it alone.

---

## Verification checklist

After implementing a new rule, verify:

### Rust
- [ ] `cargo test` — all tests pass
- [ ] `cargo check --target wasm32-unknown-unknown` — compiles for WASM target
- [ ] The rule function returns `None` for invalid inputs (test edge cases)
- [ ] Error messages in `session.rs` are clear and actionable

### WASM
- [ ] `wasm-pack build --target web --release` succeeds
- [ ] New bindings are exported (check `pkg/plato_lib.d.ts`)

### Frontend
- [ ] TypeScript compiles: `vue-tsc --noEmit`
- [ ] Dev server starts: `pnpm --filter plato-site dev`
- [ ] The command appears in autocomplete (type the first few letters)
- [ ] Accepting the autocomplete inserts the correct text
- [ ] The command appears in HelpModal > Commands (locked or unlocked)
- [ ] The command appears in `/data/en/tactics.json` and `/data/zh/tactics.json`
- [ ] NLG text renders correctly when the command is executed
- [ ] Both en and zh locales have all strings

### Integration (manual browser test)
- [ ] Type `(my-cmd ...)` in the REPL — it executes and produces the expected judgement
- [ ] The step counter increments
- [ ] The proof output shows the correct context and conclusion
- [ ] Goal resolution still works after using the new command
- [ ] The unlocked problem's victory flow triggers the tactic unlock
- [ ] The tactic appears in the collected tactics sidebar after unlock

### Problem set
- [ ] Updated problems load without JSON parse errors
- [ ] Hint step numbers are consistent (no references to non-existent steps)
- [ ] Both locales have the same number of hints and consistent structure
- [ ] The problem is still solvable following the guides

---

## Summary: Files touched

For a typical new rule, expect to modify **17-20 files**:

| # | File | Change |
|---|------|--------|
| 1 | `packages/plato-lib/src/rules/*.rs` | Add rule function |
| 2 | `packages/plato-lib/src/rules/mod.rs` | Register module (if new file) |
| 3 | `packages/plato-lib/src/inference.rs` | Add `DeductionRule` variant + `deduce()` arm |
| 4 | `packages/plato-lib/src/parser/command.rs` | Add `Command` variant + parser + `meta()` |
| 5 | `packages/plato-lib/src/parser/session.rs` | Add `execute()` arm + help text |
| 6 | `packages/plato-lib/src/lib.rs` | Add `Prover` method |
| 7 | `packages/plato-site/src/data/en/tactics.json` | Add entry |
| 8 | `packages/plato-site/src/data/zh/tactics.json` | Add entry |
| 9 | `packages/plato-site/src/data/en/nlg.ts` | Add NLG template |
| 10 | `packages/plato-site/src/data/zh/nlg.ts` | Add NLG template |
| 11 | `packages/plato-site/src/i18n/locales/en.ts` | Add `help.commands` key |
| 12 | `packages/plato-site/src/i18n/locales/zh.ts` | Add `help.commands` key |
| 13 | `packages/plato-site/src/components/HelpModal.vue` | Add to command group |
| 14 | `packages/plato-site/src/composables/useAutocomplete.ts` | Add to `STATIC` array |
| 15-17 | Section `section.json` files (×3) | Add to `allowedTactics` |
| 18-19 | Problem unlock (en + zh) | Add tactic name to `unlocks` array |
| 20+ | Problem updates (optional, both locales) | Replace old patterns |
