# Creating Problems for Plato

Plato is an interactive natural-deduction proof assistant. This guide explains how to write new problems in JSON format — no programming required. You'll learn the formula syntax, the available commands, and how to design guides and hints that teach effectively.

## Table of Contents

1. [Quick start](#quick-start)
2. [The `Problem` schema](#the-problem-schema)
3. [Formula syntax](#formula-syntax)
4. [Axiom sets: PL vs FOL](#axiom-sets-pl-vs-fol)
5. [Tactic reference](#tactic-reference)
6. [Text markup](#text-markup)
7. [Writing guides](#writing-guides)
8. [Writing hints](#writing-hints)
9. [Unlocking tactics](#unlocking-tactics)
10. [Complete example](#complete-example)
11. [Section config](#section-config)
12. [Design principles](#design-principles)

---

## Quick start

1. Create a JSON file under the appropriate section: `packages/plato-site/src/data/{locale}/sections/{section}/problems/`.
2. Name it `NN-slug.json` where `NN` is a two-digit sort key (e.g. `17-my-problem.json`).
3. It is auto-discovered — no imports to update. File order is alphabetical within each section; the sort key determines the problem index (0-based).
4. The section's `section.json` defines the default **axiom set** (PL or FOL) and the **allowed tactics** for all problems in that section. Individual problems can override the axiom set if needed.
5. You must create the JSON in **both** locale folders (`en/` and `zh/`) unless you are only targeting one language.

**Quick checklist for a valid problem:**

- `description`: a short, playful sentence (rendered with inline markup)
- `goal`: an s-expression formula the student must prove
- `premise`: array of s-expression formulas given as starting facts (can be `[]`)
- Either `guides` or `hints` must be non-empty (not both — pick one approach)
- `unlocks`: array of tactic name strings this problem teaches

---

## The `Problem` schema

Every problem is a JSON object:

```json
{
  "description": "witty one-liner",
  "goal":        "<s-expression>",
  "premise":     [],
  "logicMode":   "fol",
  "guides":      [ ... ],
  "hints":       [ ... ],
  "unlocks":     [ ... ]
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `description` | `string` | Yes | A short sentence shown before the proof. Supports [text markup](#text-markup). |
| `goal` | `string` | Yes | The formula to prove, as an s-expression. Must match the final proof step structurally. |
| `premise` | `string[]` | Yes | Formulas given as starting facts. Use `[]` if there are none. |
| `logicMode` | `"pl"` or `"fol"` | No | Override the section's axiom set. Omit to use the section default. See [Axiom sets](#axiom-sets-pl-vs-fol). |
| `guides` | `Hint[]` | Yes* | Step-by-step walkthrough (shown one at a time). Use **or** hints, not both. |
| `hints` | `Hint[]` | Yes* | Progressive nudges available on demand. Use **or** guides, not both. |
| `unlocks` | `string[]` | Yes | Tactic names this problem unlocks when solved. Use `[]` for practice problems. |

\* Either `guides` or `hints` must be provided. They should not both be non-empty for the same problem — pick one approach.

**Notes:**
- The `id` field is assigned automatically by filename ordering — do **not** include it in the JSON.
- For built-in section problems, omit `logicMode` unless you need to override the section default.

### `description` (string)

A short, playful sentence shown before the student begins. Rendered with the [inline markup parser](#text-markup).

**Examples:**
- `"I am, therefore I am."`
- `"Some truths follow from following truths."`

### `goal` (string)

The formula the student must prove, written as an **s-expression** (see [Formula syntax](#formula-syntax)). This is the conclusion — the right-hand side of $\Gamma \vdash \varphi$.

**Examples:**
- `"(-> A B)"` — prove $A \to B$
- `"(and (or A B) (-> A C))"` — prove $(A \lor B) \land (A \to C)$

When the student types the final step, the engine checks that the step's conclusion matches this string **structurally** (same s-expression tree, ignoring whitespace). Conjunction and disjunction are commutative — `(and A B)` equals `(and B A)`.

### `premise` (array of strings)

Formulas that are **given** — the student must `assume` them one by one before they can be used. These are displayed as "PREMISE" labels above the goal.

**Examples:**
- `[]` — nothing given; the student makes all assumptions
- `["A", "B"]` — $A$ and $B$ are both given
- `["A", "(-> A B)"]` — $A$ and $A \to B$ are given

### Hint object

```json
{
  "text": "Explain what to do next. Supports $latex$, **bold**, and `code`.",
  "tactic": "(assume A)"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `text` | `string` | Yes | Prose explanation. Supports [text markup](#text-markup). |
| `tactic` | `string` or `null` | No | The exact command to type. If omitted or `null`, shows an "OK" button instead of locking input. |

- **In guides**: a `tactic` locks the input until the student types the exact match. No `tactic` shows "OK" to advance.
- **In hints**: a `tactic` shows a "fill in" button that inserts the command into the REPL. No `tactic` shows only prose.

---

## Formula syntax

Formulas are written as **s-expressions** (Lisp-style parenthesized prefix notation). The outermost parentheses are required for compound formulas.

### Atomic formulas

| String | Meaning |
|---|---|
| `A`, `B`, `p`, `q`, any bare word | Propositional variable (atom) |
| `T`, `⊤`, `top`, `true` | Truth (verum): $\top$ |
| `_|_`, `⊥`, `bottom`, `false` | Falsity (contradiction): $\bot$ |

Atom names can contain any character except whitespace, `(`, and `)`. Convention: use single uppercase letters (`A`, `B`, `C`, `P`, `Q`).

### Propositional connectives

| Constructor | Aliases | Example | Meaning |
|---|---|---|---|
| `(and p q)` | `∧` | `(and A B)` | Conjunction: $A \land B$ |
| `(or p q)` | `∨` | `(or A B)` | Disjunction: $A \lor B$ |
| `(-> p q)` | `→`, `imp`, `⊃` | `(-> A B)` | Implication: $A \to B$ |
| `(not p)` | `¬`, `neg` | `(not A)` | Negation: $\lnot A$ |

### Modal operators (PL mode only)

| Constructor | Aliases | Example | Meaning |
|---|---|---|---|
| `(box p)` | `□`, `nec` | `(box A)` | Necessity: $\Box A$ |
| `(diamond p)` | `◇`, `dia`, `poss` | `(diamond A)` | Possibility: $\Diamond A$ |

### First-order formulas (FOL mode only)

First-order logic introduces **terms** (things that refer to objects) and **predicates** (things that say something about objects). A predicate is written as `(App P t)` — "apply predicate $P$ to term $t$."

#### Terms

A term can be:

- A **variable** — a fresh symbol introduced by `(fix x)`, e.g. `x`, `y`, `z`
- A **constant** — any bare word not introduced by `fix`, e.g. `socrates`, `a`, `b`, `zero`
- A **function application** — `(App f t)` for unary functions, `(App (App f x) y)` for nested. Function symbols follow the same syntax as predicate symbols; the engine distinguishes them by context.

Variables and constants are just bare atoms — there is no syntactic marker distinguishing them. A symbol is a variable if it was introduced by `fix` in the current proof; otherwise it is treated as a constant.

#### Predicate application

Predicates are applied to terms using nested `App` (currying). A unary predicate takes one term; a binary predicate takes two terms via nested `App`; ternary takes three, and so on.

| Arity | Syntax | Meaning |
|---|---|---|
| Unary | `(App P x)` | $P(x)$ — "x has property P" |
| Binary | `(App (App R x) y)` | $R(x, y)$ — "x stands in relation R to y" |
| Ternary | `(App (App (App T x) y) z)` | $T(x, y, z)$ |

**Examples:**
```
; "Socrates is mortal"
(App Mortal socrates)

; "All men are mortal"
(forall x (-> (App Man x) (App Mortal x)))

; "Someone loves everyone"
(exists x (forall y (App (App Loves x) y)))
```

Predicate and function symbols follow the same naming rules as propositional atoms: any characters except whitespace, `(`, and `)`.

| Constructor | Aliases | Example | Meaning |
|---|---|---|---|
| `(forall x body)` | `∀` | `(forall x (App P x))` | Universal: $\forall x.\,P(x)$ |
| `(exists x body)` | `∃` | `(exists x (App P x))` | Existential: $\exists x.\,P(x)$ |
| `(App P t)` | — | `(App P x)` | Predicate application: $P(x)$ |

### Nesting

Formulas can be arbitrarily nested:

```
(-> (and (or A B) (-> A C)) (or C B))
```

This means: $(A \lor B) \land (A \to C) \;\to\; C \lor B$

### LaTeX rendering

The engine converts formulas to LaTeX automatically. Compound sub-expressions are wrapped in parentheses for unambiguous display:

- `(-> (and A B) A)` → $(A \land B) \to A$
- `(-> (or A B) (or B A))` → $(A \lor B) \to (B \lor A)$

### Equality and commutativity

When verifying proofs, the engine checks formula equality:

| Connective | Commutative? | Meaning |
|---|---|---|
| `and` | **Yes** | `(and A B)` = `(and B A)` |
| `or` | **Yes** | `(or A B)` = `(or B A)` |
| `->` | **No** | `(-> A B)` ≠ `(-> B A)` |

---

## Axiom sets: PL vs FOL

Plato's engine supports two axiom sets that determine which logical operations are available. A colored chip in the navbar (blue for FOL on, orange for FOL off) tells the student which set is active.

### Why two separate sets?

The separation exists for a reason. Plato lives at the intersection of three logical systems:

- **Propositional logic (PL)** — reasoning about $\lnot$, $\land$, $\lor$, $\to$, $\top$, $\bot$. Decidable (a truth table can settle any statement).
- **First-order logic (FOL)** — adds $\forall$, $\exists$, and predicate application. Semi-decidable, well-behaved for natural deduction.
- **Modal logic (System K)** — adds $\Box$ and $\Diamond$. Decidable on its own (can be translated into a fragment of FOL via possible-world semantics).

The problem is **first-order modal logic (FOML)** — combining FOL with $\Box$ and $\Diamond$. Quantifiers and modal operators interact in ways that make the combined system undecidable (even the monadic fragment, unlike monadic FOL which is decidable). There is no single canonical semantics — different philosophical commitments about domains across possible worlds produce irreconcilably different logics.

Plato's solution: **don't combine them.** Each section picks one mode and stays within it.

### PL mode (`"logicMode": "pl"`)

Available: all propositional connectives ($\lnot$, $\land$, $\lor$, $\to$, $\top$, $\bot$) plus modal operators ($\Box$, $\Diamond$).

Disabled: quantifiers ($\forall$, $\exists$), predicate application (`App`), term variables (`fix`).

Used by: **Propositional Logic** section and **Modal Logic** section.

### FOL mode (`"logicMode": "fol"`)

Available: all propositional connectives plus quantifiers ($\forall$, $\exists$), predicate application (`App`), and term variables (`fix`).

Disabled: modal operators ($\Box$, $\Diamond$).

Used by: **First-Order Logic** section.

### Setting the axiom set

- **Section default**: set in `section.json` via `"logicMode": "pl"` or `"logicMode": "fol"`. All problems in the section inherit this unless they override.
- **Per-problem override**: add `"logicMode": "pl"` or `"logicMode": "fol"` to a problem JSON. This is rarely needed for built-in problems but is useful for custom/standalone problems.
- **Custom problems** (loaded via URL or file): should always include `logicMode` since there's no section to provide a default. If omitted, the engine defaults to `"fol"`.

---

## Tactic reference

This is the complete list of commands the proof engine accepts. Every tactic uses **1-based step numbers** — the first proof step is step 1, the second is step 2, etc.

### assume

```
(assume F)
```

Creates a new assumption: $\{F\} \vdash F$. The fundamental way to bring a formula into the context. Every proof starts by assuming premises or the antecedent of an implication.

**Example:** `(assume A)` → step 1: $\{A\} \vdash A$

### extend

```
(extend F N)
```

**Weakening.** Adds formula $F$ to step N's context without changing the conclusion. If step N proves $\Gamma \vdash p$, this produces $\Gamma, F \vdash p$.

Aliases: `weaken`

**Example:** `(extend B 1)` — step 1 proves $\{A\} \vdash A$ → produce $\{A, B\} \vdash A$

### and-intro

```
(and-intro N M)
```

Combines steps N and M into a conjunction. Both steps' contexts are merged.

**Example:** `(and-intro 1 2)` — step 1 proves $A$, step 2 proves $B$ → produce $A \land B$

### and-elim-l / and-elim-r

```
(and-elim-l N)
(and-elim-r N)
```

Extract the left or right half of a conjunction from step N.

**Example:** `(and-elim-l 1)` — step 1 proves $A \land B$ → produce $A$

### or-intro-l / or-intro-r

```
(or-intro-l N F)
(or-intro-r N F)
```

Introduce a disjunction. `or-intro-l` puts step N's conclusion on the **left** with formula F on the right. `or-intro-r` puts formula F on the **left** with step N's conclusion on the right.

**Example:** `(or-intro-l 1 B)` — step 1 proves $A$ → produce $A \lor B$
**Example:** `(or-intro-r 1 A)` — step 1 proves $B$ → produce $A \lor B$

### or-elim

```
(or-elim N M K)
```

**Proof by cases.** Step N must be a disjunction $p \lor q$. Steps M and K must both prove the **same** conclusion $r$ — one from a context containing $p$, the other from a context containing $q$. Produces $r$ (contexts merged).

**Example:** `(or-elim 1 3 5)` — step 1 is $A \lor B$, step 3 proves $C$ from $A$, step 5 proves $C$ from $B$ → produce $C$

### ->-intro

```
(->-intro F N)
```

**Implication introduction.** Discharges formula $F$ from step N's context, wrapping the conclusion as $F \to \text{conclusion}$.

Aliases: `imp-intro`, `→-intro`

**Example:** `(->-intro A 2)` — step 2 has $\{A, B\} \vdash C$ → produce $\{B\} \vdash A \to C$

### ->-elim

```
(->-elim N M)
```

**Modus ponens.** Step N must be $p \to q$. Step M must be $p$. Produces $q$.

Aliases: `imp-elim`, `→-elim`, `modus-ponens`, `mp`

**Example:** `(->-elim 2 1)` — step 2 is $A \to B$, step 1 is $A$ → produce $B$

### not-intro

```
(not-intro F N M)
```

**Reductio ad absurdum.** Formula $F$ must be present in both steps' contexts. Steps N and M must reach contradictory conclusions (e.g. one proves $p$, the other proves $\lnot p$). Produces $\lnot F$ with $F$ discharged.

Aliases: `¬-intro`, `neg-intro`, `raa`

### not-elim

```
(not-elim N)
```

Step N must be $\lnot p$. Produces $\bot$ (falsity).

Aliases: `¬-elim`, `neg-elim`

### dne

```
(dne N)
```

**Double negation elimination.** Step N must be $\lnot \lnot p$. Produces $p$.

Aliases: `dneg-elim`, `¬¬-elim`, `double-neg-elim`

### ex-falso

```
(ex-falso N F)
```

**Principle of explosion.** From a contradiction (step N proves $\bot$), derive **any** formula $F$.

Aliases: `efq`, `explosion`

### top-intro

```
(top-intro)
(top-intro N)
```

**Truth introduction.** $\top$ is always provable. Without argument: $\emptyset \vdash \top$ (empty context). With argument N: produces $\top$ in step N's context.

### show

```
(show N)
```

Re-displays step N. Does not create a new proof step.

Aliases: `print`, `p`

### subst

```
(subst N (A F)...)
```

**Uniform substitution.** Replace every occurrence of atom A with formula F in step N's conclusion. Multiple `(atom formula)` pairs may be given.

Aliases: `substitute`

**Example:** `(subst 1 (P (and A B)))` — step 1 proves $P \to P$ → produce $(A \land B) \to (A \land B)$

### fix

```
(fix x)
```

Introduce a fresh term variable. Produces $\{x\} \vdash x$. The variable must be fresh (not free in any existing assumption). Available only in **FOL mode**.

**Example:** `(fix x)` → step 1: $\{x\} \vdash x$

### forall-intro

```
(forall-intro x N)
```

**Universal generalisation.** From step N, which proves $\varphi$ for an arbitrary variable $x$ (not free in other assumptions), produce $\forall x.\,\varphi$. Available only in **FOL mode**.

Aliases: `∀-intro`, `forall`

**Example:** `(forall-intro x 3)` — step 3 proves $P(x)$ from $\{x\}$ → produce $\forall x.\,P(x)$

### forall-elim

```
(forall-elim N t)
```

**Universal instantiation.** Step N must be $\forall x.\,\varphi$. Substitute term $t$ for $x$ to produce $\varphi[t/x]$. Available only in **FOL mode**.

Aliases: `∀-elim`

**Example:** `(forall-elim 1 socrates)` — step 1 is $\forall x.\,\text{Mortal}(x)$ → produce $\text{Mortal}(\text{socrates})$

### exists-intro

```
(exists-intro N t x)
```

**Existential generalisation.** From step N (proving $\varphi[t/x]$ for term $t$), produce $\exists x.\,\varphi$. Available only in **FOL mode**.

Aliases: `∃-intro`

**Example:** `(exists-intro 2 socrates x)` — step 2 proves $\text{Mortal}(\text{socrates})$ → produce $\exists x.\,\text{Mortal}(x)$

### exists-elim

```
(exists-elim N M x)
```

**Existential witness elimination.** Step N proves $\exists x.\,\varphi$. Step M proves $\psi$ under the assumption that a fresh witness $x$ satisfies $\varphi$. $x$ must not appear free in $\psi$ or any open assumption. Available only in **FOL mode**.

Aliases: `∃-elim`

**Example:** `(exists-elim 1 3 y)` — step 1 gives $\exists x.\,P(x)$, step 3 proves $Q$ from $\{y, P(y)\}$ → produce $Q$

### box-intro

```
(box-intro N)
```

**Necessitation.** Step N must prove $P$ with an **empty context** (no open assumptions). Produces $\Box P$. Available only in **PL mode** (modal section).

Aliases: `□-intro`, `nec`

### box-elim

```
(box-elim N M)
```

**K axiom.** Step N proves $\Box(P \to Q)$. Step M proves $\Box P$. Produces $\Box Q$. Necessity distributes over implication. Available only in **PL mode** (modal section).

Aliases: `□-elim`, `k`

### diamond-def

```
(diamond-def N)
```

**Diamond definition (unfold).** Step N proves $\Diamond P$. Produces $\lnot\Box\lnot P$. Available only in **PL mode** (modal section).

Aliases: `◇-def`, `dia-def`

### diamond-def-rev

```
(diamond-def-rev N)
```

**Diamond definition (fold).** Step N proves $\lnot\Box\lnot P$. Produces $\Diamond P$. Available only in **PL mode** (modal section).

Aliases: `◇-def-rev`, `dia-def-rev`

---

## Text markup

The `text` field in guides, hints, and the `description` field support inline markup parsed by the frontend:

| Syntax | Rendered as |
|---|---|
| `$latex$` | Inline KaTeX math, e.g. `$A \land B$` |
| `$$latex$$` | Display-style KaTeX (centered, larger), e.g. `$$\forall x \Box P(x) \to \Box \forall x P(x)$$` |
| `**bold**` | **Bold text**, supports nested markup inside |
| `` `code` `` | Monospace code, usually for tactic names or step numbers |
| `[id\|display]` | Glossary link — opens HelpModal glossary tab scrolled to entry `id`. e.g. `[modus-ponens\|modus ponens]` |
| `[id]` | Glossary link — same as above but uses the id as display text, e.g. `[contradiction]` |

**Important:** In inline LaTeX (`$...$`), use double backslashes for LaTeX commands: `$A \\land B$`, `$\\vdash$`, `$\\Gamma$`, `$\\Box$`.

**Examples:**
```
"The goal is $A \\to B$. Use `->-intro` to **discharge** the [assumption|assumption]."
```

---

## Writing guides

Guides are for problems that **introduce a new tactic**. The student is walked through step by step. Each guide card appears one at a time before the student types.

### Rules

1. The first guide should explain the goal in plain language. No tactic (use `"tactic": null`).
2. Each subsequent guide introduces one concept or one proof step.
3. A guide **with** a `tactic` locks the input — the student must type the exact match to proceed.
4. A guide **without** a `tactic` (or `"tactic": null`) shows an "OK" button; the student reads and clicks to advance.
5. The final guide should conclude the proof and celebrate the achievement.
6. **Never mix** — a problem with guides should have `"hints": []`.

### Example (from an implication-elimination problem)

```json
{
  "guides": [
    {
      "text": "We're proving $A \\land B \\to A$. In words: if both $A$ and $B$ are true, then certainly $A$ is true. The logic is hard to argue with.",
      "tactic": null
    },
    {
      "text": "As always with an implication, we start by assuming the left-hand side. This time the left-hand side is a conjunction: $A \\land B$.",
      "tactic": "(assume (and A B))"
    },
    {
      "text": "Now we have $A \\land B$ at step 1. The rule `and-elim-l` extracts the **left** half of a conjunction — the $A$ from $A \\land B$.",
      "tactic": "(and-elim-l 1)"
    },
    {
      "text": "We set out to get $A$ under the assumption $A \\land B$, and we have it. Use `->-intro` to discharge the assumption and close the proof. Notice the first argument is $(A \\land B)$ — the formula you discharge doesn't have to be a bare atom.",
      "tactic": "(->-intro (and A B) 2)"
    }
  ]
}
```

### Tone

- Conversational, second person ("we", "our job").
- Slightly witty, but never at the expense of clarity.
- Introduce notation gently: "The symbol $\\vdash$ is pronounced 'entails'."
- Acknowledge when something is new: "This time the left-hand side is a conjunction…"

---

## Writing hints

Hints are for **synthesis/practice problems** where the student should solve independently but can request help.

### Rules

1. Hints appear one at a time — each new lightbulb appears when the previous hint is opened.
2. Early hints should point in the right direction without giving the tactic.
3. Middle hints should narrow the approach.
4. Later hints should give the exact tactic.
5. The final hint should give the final tactic.
6. Each hint builds on the assumption that the student has followed the previous hints.
7. **Never mix** — a problem with hints should have `"guides": []`.

### Step numbers in hint text

When writing hint text, refer to step numbers **as they will be at that point in the proof**. Steps are 1-based and increment with each successful tactic. Calculate them carefully — a wrong step number in hint text will confuse students.

### Example

```json
{
  "hints": [
    {
      "text": "The goal is $(A \\land (A \\to B)) \\to B$. We're given $A$ and the promise $A \\to B$ bundled together. If you can extract both, modus ponens takes care of the rest. Start by assuming the conjunction.",
      "tactic": null
    },
    {
      "text": "Assume the bundled promise: $A \\land (A \\to B)$.",
      "tactic": "(assume (and A (-> A B)))"
    },
    {
      "text": "Peel off the left half — that's $A$.",
      "tactic": "(and-elim-l 1)"
    },
    {
      "text": "Now the right half — that's $A \\to B$, the promise itself.",
      "tactic": "(and-elim-r 1)"
    },
    {
      "text": "You have the promise at step 3 and the key at step 2. Use modus ponens to unlock $B$.",
      "tactic": "(->-elim 3 2)"
    },
    {
      "text": "Discharge the assumption to wrap up.",
      "tactic": "(->-intro (and A (-> A B)) 4)"
    }
  ]
}
```

---

## Unlocking tactics

Each problem can unlock zero or more tactics via the `unlocks` array. When the student solves the problem, these tactic names are added to their collection (persisted in `localStorage`). The full metadata — description, syntax, example, and inference rule LaTeX — lives in `tactics.json`, not in the problem file.

```json
"unlocks": ["->-elim"]
```

### Progression guidelines

- **One tactic per 1–2 problems.** Don't dump five new rules at once.
- **Follow every introduction with a synthesis problem** that uses the new tactic alongside previously learned ones, without introducing anything new (`"unlocks": []`).
- **Unlock symmetric rules together** — `and-elim-l` and `and-elim-r` in one problem, `or-intro-l` and `or-intro-r` in one problem. The student may only use one in a guided problem, but knowing the other exists is important.

### Recommended introduction order

**Propositional Logic section** (PL mode):

1. `assume`, `->-intro` — the foundational moves
2. `and-intro` — building conjunctions
3. (synthesis) — combine assume, ->-intro, and-intro
4. `and-elim-l`, `and-elim-r` — breaking conjunctions
5–7. (practice problems)
8. `->-elim` — modus ponens
9–10. (practice problems)
11. `or-intro-l`, `or-intro-r` — building disjunctions
12–13. (practice problems)
14. `or-elim` — proof by cases
15. (practice)
16. (grand combo) — all propositional tactics
17+. `not-intro`, `not-elim`, `dne`, `ex-falso`, `extend`, `subst`, `top-intro`

**First-Order Logic section** (FOL mode):

- `fix` — introduce fresh term variables
- `forall-intro`, `forall-elim` — universal quantifier rules
- `exists-intro`, `exists-elim` — existential quantifier rules

**Modal Logic section** (PL mode, with modal operators):

- `box-intro` — necessitation (requires empty context: $\emptyset \vdash P$)
- `box-elim` — K axiom: $\Box(P \to Q) \to (\Box P \to \Box Q)$
- `diamond-def` — unfold $\Diamond$ via $\lnot\Box\lnot$
- `diamond-def-rev` — fold $\lnot\Box\lnot$ back to $\Diamond$

---

## Complete example

Here is a full problem file that introduces `->-elim` (modus ponens):

```json
{
    "description": "Some truths follow others. Let one follow.",
    "goal": "B",
    "premise": ["A", "(-> A B)"],
    "guides": [
        {
            "text": "We're given two things: $A$ is true, and $A \\to B$ is true. Our goal is to reach $B$, with no extra fluff. This is as direct as reasoning gets.",
            "tactic": null
        },
        {
            "text": "First, bring the premise $A$ into the context. If the world tells you it's true, you may as well write it down.",
            "tactic": "(assume A)"
        },
        {
            "text": "Now bring in the second premise: $A \\to B$. This says that whenever $A$ holds, $B$ follows.",
            "tactic": "(assume (-> A B))"
        },
        {
            "text": "You have $A \\to B$ at step 2 and $A$ at step 1. The rule `->-elim` — also known as **modus ponens** — says: if you know an implication and you also know its left-hand side, you get the right-hand side. Apply it now.",
            "tactic": "(->-elim 2 1)"
        },
        {
            "text": "And $B$ is yours. The proof ends here — no implication to discharge, because the premises were given, not assumed for the sake of argument. Just a clean conclusion.",
            "tactic": null
        }
    ],
    "hints": [],
    "unlocks": ["->-elim"]
}
```

---

## Section config

Each section folder contains a `section.json` that defines the section's metadata:

```json
{
  "nameI18nKey": "sections.firstOrder",
  "logicMode": "fol",
  "order": 1,
  "allowedTactics": [
    "assume", "extend",
    "ex-falso", "top-intro", "subst", "fix",
    "->-intro", "->-elim",
    "and-intro", "and-elim-l", "and-elim-r",
    "or-intro-l", "or-intro-r", "or-elim",
    "not-intro", "not-elim", "dne",
    "forall-intro", "forall-elim",
    "exists-intro", "exists-elim"
  ]
}
```

| Field | Type | Description |
|---|---|---|
| `nameI18nKey` | `string` | i18n key for the section name, e.g. `"sections.propositional"` |
| `logicMode` | `"pl"` or `"fol"` | **Default axiom set.** PL: propositional connectives + modal operators. FOL: propositional connectives + quantifiers + predicate application + term variables. Can be overridden per-problem. |
| `order` | `number` | Linear progression order (0, 1, 2…). Sections unlock sequentially — all problems in all prior sections must be completed before the next section becomes accessible. |
| `allowedTactics` | `string[]` | The exact set of commands enabled in this section. Filters autocomplete, the tactic sidebar, and the help modal. Tactics not listed here are hidden even if the student has collected them from earlier sections. |

### Adding a new section

1. Create a folder at `src/data/{locale}/sections/{id}/` for each locale.
2. Add `section.json` with the appropriate `logicMode`, `order`, and `allowedTactics`.
3. Add `discovery.json` — a dialogue between characters introducing the section's concepts (see existing sections for the format: an array of `{ "speaker": "...", "text": "..." }` lines under a `"title"`).
4. Add a `problems/` subfolder with numbered JSON files.
5. No code changes required — sections are auto-discovered via `import.meta.glob`.

### Per-problem axiom set override

For **custom / standalone problems** (loaded via URL or file), always include `logicMode` in the problem JSON since there's no section to provide a default:

```json
{
  "description": "A modal logic proof",
  "goal": "(box A)",
  "premise": ["A"],
  "logicMode": "pl",
  "guides": [],
  "hints": [],
  "unlocks": []
}
```

For **built-in section problems**, omit `logicMode` unless you need a specific problem to operate under a different axiom set than its section (rare, but supported).

---

## Design principles

1. **Scaffold, then remove.** Guided problems hold the student's hand. Synthesis problems make them think. The final combo challenge tests everything.

2. **One new idea at a time.** Each guide problem introduces one inference rule, not three.

3. **Explain the "why", not just the "what".** Don't just state the rule — explain what it *means* in plain language.

4. **Proofs should be clean.** Each problem's intended solution should feel elegant. If a problem requires 15 steps of contortion, redesign it.

5. **Prefer implication-shaped goals.** Problems of the form $P \to Q$ are pedagogically clean — assume $P$, prove $Q$, discharge. They also work naturally with empty premises.

6. **Context is invisible but important.** The engine tracks contexts (sets of assumptions) behind the scenes. `->-intro` discharges one formula; `or-elim` unions three contexts; `extend` adds a formula without changing the conclusion. Be aware of what's in scope at each step.

7. **Check commutativity.** `and` and `or` are commutative — `(and A B)` matches `(and B A)`. Implication is not. This affects how the engine compares the student's final step against your `goal`.

8. **Test your problem.** Run the app and solve the problem yourself. Verify every guide/hint tactic works, step numbers are correct, and the goal resolves properly.
