# Creating Problems for Plato

Plato is an interactive natural-deduction proof assistant. This guide explains how to write new problems in JSON format — no programming required.

## Table of Contents

1. [Quick start](#quick-start)
2. [File structure](#file-structure)
3. [The `Problem` schema](#the-problem-schema)
4. [Formula syntax](#formula-syntax)
5. [Tactic reference](#tactic-reference)
6. [Writing guides](#writing-guides)
7. [Writing hints](#writing-hints)
8. [Unlocking tactics](#unlocking-tactics)
9. [Complete example](#complete-example)
10. [Design principles](#design-principles)

---

## Quick start

1. Create a new JSON file under the appropriate section: `packages/plato-site/src/data/{locale}/sections/{section}/problems/`.
2. Name it `NN-slug.json` where `NN` is a two-digit number within the section (e.g. `17-my-problem.json`).
3. It will be auto-discovered — no imports to update. The file order is alphabetical within each section.
4. The section's `section.json` defines the **axiom set** (PL or FOL) and allowed tactics for all problems in that section.

## File structure

Every problem is a JSON object with these top-level keys:

```
{
  "description": "witty one-liner",
  "goal":        "<s-expression>",
  "premise":     [],
  "guides":      [ ... ],
  "hints":       [ ... ],
  "unlocks":     [ ... ]
}
```

The description, goal, premise, guides, hints, and unlocks fields are spelled out below. **Note:** the `id` field is assigned automatically by filename ordering within the section — do not include it in the JSON. The `logicMode` field is **removed**; it is governed by the section's `section.json` (see [Section config](#section-config)).

---

## The `Problem` schema

### `description` (string)

A short, playful sentence shown to the student before they begin the proof. This is rendered using the inline markup parser (see [Text markup](#text-markup)).

**Examples:**
- `"I am, therefore I am."`
- `"Some truths follow from following truths."`

### `goal` (string)

The formula the student must prove, written as an **s-expression** (see [Formula syntax](#formula-syntax) below). This is the right-hand side of the sequent — the conclusion.

**Examples:**
- `"(-> A B)"` — prove $A \to B$
- `"(and (or A B) (-> A C))"` — prove $(A \lor B) \land (A \to C)$

If the student types the final step of the proof correctly, the engine checks that the step's proposition matches this string **structurally** (same s-expression tree, ignoring whitespace). Conjunction and disjunction are commutative — `(and A B)` equals `(and B A)`.

The goal is also displayed as LaTeX to the student. The UI calls `Session.formulaLatex(goal)` to render it.

### `premise` (array of strings)

Formulas that are **given** — the student must `assume` them one by one before they can be used. These are displayed above the goal in the UI as "PREMISE" labels. If there are no premises, use an empty array `[]`.

**Examples:**
- `[]` — nothing given; the student makes all assumptions
- `["A", "B"]` — $A$ and $B$ are both given
- `["A", "(-> A B)"]` — $A$ and $A \to B$ are given

Each entry is rendered via `Session.formulaLatex()`, so they can be any valid formula s-expression.

### `guides` (array of Hint objects)

A **step-by-step walkthrough**. The guide cards appear one at a time before the student types. Each guide can optionally lock the input — if a guide has a `tactic` field, the student must type that exact tactic to proceed. Guides with no tactic show an "OK" button instead.

**When to use:** problems that introduce a new tactic — the student is learning it for the first time and benefits from hand-holding.

### `hints` (array of Hint objects)

**Progressive nudges** available on demand. Hints are revealed one at a time via lightbulb buttons. The student can open multiple hints and dismiss them individually. Hints with a tactic offer a "fill in" button that inserts the tactic into the input; hints without a tactic only show prose.

**When to use:** synthesis/practice problems where the student should figure things out independently.

### `unlocks` (array of Tactic objects)

The new inference rules introduced by this problem. When the student solves the problem, these tactics are added to their tactics sidebar. See [Unlocking tactics](#unlocking-tactics) below.

### Hint object

```json
{
  "text": "Explain what the student should do next. Supports **bold**, $latex$, and `code`.",
  "tactic": "(assume A)"
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `text` | `string` | Yes | Prose explanation. Supports text markup. |
| `tactic` | `string` or `null` | No | The exact command the student should type. If omitted or `null`, shows an "OK" button. |

### Tactic object

```json
{
  "name": "→-intro",
  "rule": "\\frac{\\Gamma,\\; p \\vdash q}{\\Gamma \\vdash p \\to q}",
  "description": "If from assuming $p$ you can reach $q$, then $p$ implies $q$."
}
```

| Field | Type | Description |
|---|---|---|
| `name` | `string` | Display name, e.g. `"→-intro"`, `"∧-intro"` |
| `rule` | `string` | KaTeX inference rule in fraction notation |
| `description` | `string` | Plain-language explanation of what the rule means |

The `rule` field is rendered by the KaTeX math renderer. Use standard LaTeX math notation inside the string:
- `\frac{numerator}{denominator}` — inference rule bar
- `\Gamma`, `\Delta` — context variables
- `\vdash` — turnstile (entails)
- `\to`, `\land`, `\lor`, `\lnot` — logical connectives
- `\\;` — spacing
- `\\{` and `\\}` — literal braces

---

## Formula syntax

Formulas are written as **s-expressions** (Lisp-style prefixed parenthesized notation). The outermost parentheses are always required for compound formulas.

### Atomic formulas

| String | Meaning |
|---|---|
| `A`, `B`, `p`, `q`, any bare word | Propositional variable (atom) |
| `T`, `⊤` | Truth (verum) |
| `_|_`, `⊥`, `bottom`, `false` | Falsity (contradiction) |

Atom names can contain any character except whitespace, `(`, and `)`. So `A`, `B`, `I`, `P_1`, `foo-bar` are all valid atoms. By convention, use single uppercase letters (`A`, `B`, `C`, `I`).

### Compound formulas (binary connectives)

| Constructor | Aliases | Example | Meaning |
|---|---|---|---|
| `(and p q)` | `∧` | `(and A B)` | Conjunction: $A \land B$ |
| `(or p q)` | `∨` | `(or A B)` | Disjunction: $A \lor B$ |
| `(-> p q)` | `→`, `imp`, `⊃` | `(-> A B)` | Implication: $A \to B$ |

### Compound formulas (unary connective)

| Constructor | Aliases | Example | Meaning |
|---|---|---|---|
| `(not p)` | `neg`, `¬` | `(not A)` | Negation: $\lnot A$ |

### Nesting

Formulas can be arbitrarily nested:

```
(-> (and (or A B) (-> A C)) (or C B))
```

This means: $(A \lor B) \land (A \to C) \;\to\; C \lor B$

### LaTeX rendering

The engine automatically converts formulas to LaTeX. Compound sub-expressions are wrapped in `\left(...\right)` for unambiguous display. For example:

- `(-> (and A B) A)` → $\left(A \land B\right) \to A$
- `(-> (or A B) (or B A))` → $\left(A \lor B\right) \to \left(B \lor A\right)$

### Equality and commutativity

The engine checks formula equality when verifying proofs:

| Connective | Commutative? | Meaning |
|---|---|---|
| `and` | **Yes** | `(and A B)` and `(and B A)` are considered equal |
| `or` | **Yes** | `(or A B)` and `(or B A)` are considered equal |
| `->` | **No** | `(-> A B)` ≠ `(-> B A)` — order matters |

---

## Tactic reference

This is the complete list of commands the proof engine accepts. Every tactic uses **1-based step numbers** — the first proof step is step 1, the second is step 2, etc.

### assume

```
(assume F)
```

Creates a new assumption: $\{F\} \vdash F$. This is the fundamental way to bring a formula into the context. Every proof starts by assuming premises or the antecedent of an implication.

**Example:** `(assume A)` → step 1: $\{A\} \vdash A$

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

Insert a disjunction. `or-intro-l` puts step N's conclusion on the **left** with formula F on the right. `or-intro-r` puts formula F on the **left** with step N's conclusion on the right.

**Example:** `(or-intro-l 1 B)` — step 1 proves $A$ → produce $A \lor B$
**Example:** `(or-intro-r 1 A)` — step 1 proves $B$ → produce $A \lor B$

### or-elim

```
(or-elim N M K)
```

**Proof by cases.** Step N must be a disjunction $p \lor q$. Steps M and K must both prove the **same** conclusion $r$ — one from a context containing $p$, the other from a context containing $q$. This combines all three contexts and yields $r$.

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

### ->-into

```
(->-into N)
```

Unpacks an implication. Step N must be $p \to q$. Moves $p$ into the context: $\{ \dots, p \} \vdash q$.

Aliases: `imp-into`, `→-into`

### not-intro

```
(not-intro F N M)
```

**Reductio ad absurdum.** Formula $F$ must be in both steps' contexts. Steps N and M must reach contradictory conclusions (e.g. one proves $p$, the other proves $\lnot p$). Produces $\lnot F$ with $F$ discharged.

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

**Principle of explosion.** Step N must be $\bot$ (falsity/contradiction). From a contradiction, derive **any** formula $F$.

Aliases: `efq`, `explosion`

### show

```
(show N)
```

Re-displays step N. Does not create a new proof step.

Aliases: `print`, `p`

### help

```
help
?
(help)
(?)
```

Opens the command reference panel. Does not create a proof step.

### fix

```
(fix x)
```

Introduce a fresh term variable into the context — the term analogue of `(assume P)`. Produces `{x} ⊢ x`. The variable must be fresh (not free in any existing assumption).

**Example:** `(fix x)` → step 1: `{x} ⊢ x`

### forall-intro

```
(forall-intro x N)
```

**Universal generalisation.** From step N, which proves φ for an arbitrary variable x (not free in other assumptions), produce ∀x.φ. This is the introduction rule for the universal quantifier.

**Example:** `(forall-intro x 3)` — step 3 proves φ(x) from `{x}` → produce ∀x.φ

### forall-elim

```
(forall-elim N t)
```

**Universal instantiation.** Step N must be ∀x.φ. Substitute term t for x to produce φ[t/x]. What holds for everything holds for any particular thing.

**Example:** `(forall-elim 1 socrates)` — step 1 is ∀x.Mortal(x) → produce Mortal(socrates)

### exists-intro

```
(exists-intro N t x)
```

**Existential generalisation.** From step N (proving φ[t/x] for a concrete term t), produce ∃x.φ. You provide the step, the example term, and a fresh variable name.

**Example:** `(exists-intro 2 socrates x)` — step 2 proves Mortal(socrates) → produce ∃x.Mortal(x)

### exists-elim

```
(exists-elim N M x)
```

**Existential witness elimination.** Step N proves ∃x.φ. Step M proves ψ under the assumption that a fresh witness x satisfies φ. The witness x must not appear free in the conclusion ψ or any open assumption.

**Example:** `(exists-elim 1 3 y)` — step 1 gives ∃x.P(x), step 3 proves Q from `{y, P(y)}` → produce Q

### subst

```
(subst N (A F)...)
```

**Uniform substitution.** Replace every occurrence of atom A with formula F in step N's conclusion. Multiple (atom formula) pairs may be given. The proof structure is independent of the particular symbols used.

**Example:** `(subst 1 (P (and A B)))` — step 1 proves P → P → produce (A∧B) → (A∧B)

### top-intro

```
(top-intro)
```

**Truth introduction.** ⊤ (top/truth) is always provable, with no premises required. An axiom.

### box-intro

```
(box-intro N)
```

**Necessitation.** Step N must prove P with an **empty context** (no assumptions). Produces □P. This is the introduction rule for necessity.

Available only in PL mode (modal section). **Requires:** `∅ ⊢ P`.

### box-elim

```
(box-elim N M)
```

**K axiom.** Step N proves □(P → Q). Step M proves □P. Produces □Q. Necessity distributes over implication.

Available only in PL mode (modal section).

### diamond-def

```
(diamond-def N)
```

**Diamond definition (unfold).** Step N proves ◇P. Produces ¬□¬P.

```
(diamond-def-rev N)
```

**Diamond definition (fold).** Step N proves ¬□¬P. Produces ◇P.

---

## Text markup

The `text` field in guides and hints supports inline markup parsed by the frontend:

| Syntax | Rendered as |
|---|---|
| `$latex$` | Inline KaTeX math, e.g. `$A \land B$` |
| `**bold**` | **Bold text** |
| `` `code` `` | Monospace code, usually for tactic names or step numbers |

**Examples:**
```
"The goal is $A \\to B$. Use `->-intro` to **discharge** the assumption."
```

Render inline LaTeX math with double backslashes for commands: `$A \\land B$`, `$\\vdash$`, `$\\Gamma$`.

---

## Writing guides

Guides are for problems that **introduce a new tactic**. The student is walked through step by step.

### Rules

1. The first guide should explain the goal in plain language.
2. Each subsequent guide introduces one concept or one proof step.
3. A guide with a `tactic` field makes that the **only** thing the student can type — the input is locked until they type the exact match.
4. A guide without a `tactic` (or with `"tactic": null`) shows an "OK" button; the student reads it and clicks to advance.
5. The final guide should conclude the proof and celebrate the achievement.
6. **Never mix** — a problem with guides should have `"hints": []`.

### Example (from problem 04)

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
      "text": "We set out to get $A$ under the assumption $A \\land B$, and we have it. Use `->-intro` to discharge the assumption and close the proof. Notice the first argument is $(A \\land B)$ — the formula you discharge doesn't have to be a bare atom, it can be any formula.",
      "tactic": "(->-intro (and A B) 2)"
    }
  ]
}
```

### Tone

- Conversational, second person ("we", "our job").
- Slightly witty, but never at the expense of clarity.
- Introduce notation gently: "The symbol $\vdash$ is pronounced 'entails'."
- Acknowledge when something is new: "This time the left-hand side is a conjunction…"

---

## Writing hints

Hints are for **synthesis/practice problems** where the student should solve independently but can request help.

### Rules

1. Hints appear one at a time (new lightbulb) when the previous hint is opened.
2. Early hints should point in the right direction without giving the tactic.
3. Middle hints should narrow the approach.
4. Later hints should give the exact tactic.
5. The final hint should give the final tactic.
6. Each hint builds on the assumption that the student has followed the previous hints.
7. **Never mix** — a problem with hints should have `"guides": []`.

### Example (from problem 10)

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
      "text": "Peel off the left half — that's $A$, the key.",
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

### Hint step numbers

When writing hint text, refer to step numbers **as they will be at that point in the proof**. The step numbers are 1-based and increment with each successful tactic. Calculate them carefully — a wrong step number in hint text will confuse students.

---

## Unlocking tactics

Each problem can unlock zero or more new tactics. The unlocks are added to the student's tactics sidebar (persisted in memory for the session).

### When to introduce a new tactic

- **One tactic per 1-2 problems.** Don't dump five new rules at once.
- **Follow every introduction with a synthesis problem** that uses the new tactic alongside previously learned ones, without introducing anything new (`"unlocks": []`).
- **Unlock both symmetric rules together** — `and-elim-l` and `and-elim-r` in one problem, `or-intro-l` and `or-intro-r` in one problem. The student only uses one in the guided problem, but knowing the other exists is important.

### Rule LaTeX conventions

Use natural deduction notation:

```
\frac{\Gamma \vdash P}{\Gamma \vdash Q}
```

- `\Gamma` for the primary context
- `\Delta` for a secondary/merged context
- `\vdash` for the turnstile
- `\to`, `\land`, `\lor`, `\lnot` for connectives
- `\cup` for context union
- `\;` for spacing
- `\\{` and `\\}` for literal braces (needed because `{` and `}` are JSON-special, but in this case they appear inside a LaTeX string where the engine handles the escaping differently — prefer `\{` and `\}` in the KaTeX markup)

### Progression

The recommended order for introducing tactics across the three sections:

**Propositional Logic section** (PL axiom set):
1. `assume`, `→-intro` — the foundational moves
2. `∧-intro` — building conjunctions
3. (synthesis) — combine assume, →-intro, ∧-intro
4. `∧-elim-l`, `∧-elim-r` — breaking conjunctions
5–7. (practice problems)
8. `→-elim` — modus ponens
9–10. (practice problems)
11. `∨-intro-l`, `∨-intro-r` — building disjunctions
12–13. (practice problems)
14. `∨-elim` — proof by cases
15. (practice)
16. (grand combo) — all prop tactics
17+. negation (`not-intro`, `not-elim`, `dne`, `ex-falso`), `subst`, `top-intro`

**First-Order Logic section** (FOL axiom set):
- `fix` — introduce fresh term variables (like `assume` for terms)
- `App` — predicate application, `(App P x)` for unary, `(App (App R x) y)` for nested
- `∀-intro`, `∀-elim` — universal quantifier rules
- `∃-intro`, `∃-elim` — existential quantifier rules

**Modal Logic section** (PL axiom set, with modal operators):
- `box-intro` — necessitation (requires empty context: `∅ ⊢ P`)
- `box-elim` — K axiom: `□(P → Q) → (□P → □Q)`
- `diamond-def` — unfold/fold `◇` via `¬□¬`

---

## Complete example

Here is a full problem file that introduces `→-elim` (modus ponens):

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
    "unlocks": [
        {
            "name": "→-elim",
            "rule": "\\frac{\\Gamma \\vdash p \\to q \\quad \\Delta \\vdash p}{\\Gamma \\cup \\Delta \\vdash q}",
            "description": "If $p$ implies $q$, and you have $p$, then you have $q$. The classic syllogism — *modus ponens*."
        }
    ]
}
```

---

## Design principles

1. **Scaffold, then remove.** Guided problems hold the student's hand. Synthesis problems make them think. The final combo challenge tests everything.

2. **One new idea at a time.** Each guide problem introduces one inference rule, not three.

3. **Explain the "why", not just the "what".** Don't just state the rule — explain what it *means* in plain language.

4. **Proofs should be clean.** Each problem's intended solution should feel elegant. If a problem requires 15 steps of contortion, redesign it.

5. **Prefer implication-shaped goals.** Problems of the form $P \to Q$ are pedagogically clean — assume $P$, prove $Q$, discharge. They also work naturally with empty premises.

6. **Context is invisible but important.** The engine tracks contexts (sets of assumptions) behind the scenes. `->-intro` discharges one formula; `or-elim` unions three contexts. Be aware of what's in scope at each step.

7. **Test your problem.** The fastest way to verify a problem is to run the app and solve it yourself. Make sure every guide/hint tactic works, step numbers are correct, and the goal resolves properly.

---

## Section config

Each section folder contains a `section.json` defining the axiom set and available tactics:

```json
{
  "nameI18nKey": "sections.firstOrder",
  "logicMode": "fol",
  "order": 1,
  "allowedTactics": [
    "assume", "ex-falso", "top-intro", "subst", "fix",
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
| `logicMode` | `"pl"` or `"fol"` | **Axiom set.** PL: propositional connectives only (plus modal operators in the modal section). FOL: propositional connectives + quantifiers + predicate application + term variables. The mode is displayed in the NavBar and gates which commands are accepted. |
| `order` | `number` | Linear progression order (0, 1, 2…). Sections are unlocked sequentially — completing all problems in one section reveals the next. |
| `allowedTactics` | `string[]` | The exact set of commands available in this section. This filters the autocomplete, tactic sidebar, and help modal. Tactics not listed here are disabled even if the student has collected them. |

### Axiom set (PL vs FOL)

Plato has two axiom sets that determine which logical operations are available:

- **PL (Propositional Logic):** `¬`, `∧`, `∨`, `→`, `⊤`, `⊥` are always available. In the modal section, `□` and `◇` are additionally available.
- **FOL (First-Order Logic):** All of PL plus `∀`, `∃`, predicate application `(App P t)`, and term variables `(fix x)`. Modal operators are disabled in FOL mode.

The axiom set is set per-section in `section.json`. Individual problems no longer carry a `logicMode` field — the section default governs. This ensures all problems within a section operate under the same rules, and the axiom set label (PL or FOL) shown in the NavBar accurately reflects what the student can use.

### Adding a new section

1. Create a folder at `src/data/{locale}/sections/{id}/` for each locale.
2. Add `section.json` with the appropriate `logicMode` and `allowedTactics`.
3. Add `discovery.json` — a dialogue between characters introducing the section's concepts (see existing sections for format).
4. Add a `problems/` subfolder with numbered JSON files.
5. No code changes required — sections are auto-discovered via `import.meta.glob`.
