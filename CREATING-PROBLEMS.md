# Creating Levels for Plato

Plato is an interactive natural-deduction proof assistant. This guide explains how to write new levels in JSON format — no programming required. You'll learn the formula syntax, the available commands, the two axiom sets, and how to design guide cards and hints that teach effectively.

## Table of Contents

1. [Quick start](#quick-start)
2. [How levels work](#how-levels-work)
3. [The `Problem` schema](#the-problem-schema)
4. [Formula syntax](#formula-syntax)
5. [Axiom sets: PL vs FOL](#axiom-sets-pl-vs-fol)
6. [Tactic reference](#tactic-reference)
7. [Text markup](#text-markup)
8. [Writing guides](#writing-guides)
9. [Writing hints](#writing-hints)
10. [Writing discoveries](#writing-discoveries)
11. [Unlocking tactics](#unlocking-tactics)
12. [Complete example](#complete-example)
13. [Section config](#section-config)
14. [Design principles](#design-principles)

---

## Quick start

1. Create a JSON file in a section's `levels/` directory:
   ```
   packages/plato-site/src/data/{locale}/sections/{section}/levels/NNN-slug.json
   ```
2. The `NNN` is a zero-padded 3-digit index — the file sort order is the level order:
   - `000-discovery.json` — the section's discovery dialogue (always level 0)
   - `001-identity.json` — the first proof level (level 1)
   - `002-another-discovery.json` — another discovery (level 2)
   - …and so on.
3. All files in `levels/` are auto-discovered via `import.meta.glob` — no imports to update.
4. You must create the file in **both** locale folders (`en/` and `zh/`) unless you are only targeting one language.
5. The section's `section.json` defines the default axiom set and allowed tactics.

**Quick checklist for a proof level:**

- `"type": "problem"`
- `"description"`: a short, playful sentence (supports inline markup)
- `"goal"`: an s-expression formula to prove
- `"premise"`: array of s-expression formulas (can be `[]`)
- `"guides"` or `"hints"`: pick one approach, not both
- `"unlocks"`: array of tactic names (use `[]` for practice levels)

**Quick checklist for a discovery level:**

- `"type": "discovery"`
- `"title"`: a heading for the dialogue
- `"lines"`: array of `{ "speaker": "...", "text": "...", "sid": "...", "image": "..." }` objects

---

## How levels work

A section is a linear sequence of **levels**. Level 0 is always a discovery dialogue. Level 1+ are proof problems. The section's `section.json` governs all levels unless a level overrides.

When the player enters a section for the first time, they see level 0 (the discovery). On subsequent visits they jump straight to the first unsolved problem level. Solving a level unlocks the next one.

The route pattern uses a single unified route for all levels:

```
/section/:sectionId/level/:idx
```

where `:idx` is a 0-based index into the section's `levels` array. Level 0 is always a discovery. Level 1+ may be either discovery or problem — the `LevelView` component checks the type and renders accordingly. The player navigates linearly through all levels with prev/next buttons.

```ts
type Level =
    | { type: 'discovery'; data: DiscoveryData }
    | { type: 'problem'; data: Problem }
```

Both discovery and problem levels are part of the same timeline — they are the same "level" from the player's perspective.

---

## The `Problem` schema

A proof level is a JSON object:

```json
{
  "type": "problem",
  "description": "witty one-liner",
  "goal": "<s-expression>",
  "premise": [],
  "logicMode": "fol",
  "guides": [ ... ],
  "hints": [ ... ],
  "unlocks": [ ... ]
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | `"problem"` | Yes | Identifies this as a proof level. |
| `description` | `string` | Yes | A short sentence shown before the proof. Supports [text markup](#text-markup). |
| `goal` | `string` | Yes | The formula to prove, as an s-expression. Must match the final proof step structurally. |
| `premise` | `string[]` | Yes | Formulas given as starting facts. Use `[]` if there are none. |
| `logicMode` | `"pl"` or `"fol"` | No | Override the section's axiom set. Omit to use the section default. |
| `guides` | `Hint[]` | Yes* | Step-by-step walkthrough. Use **or** hints, not both. |
| `hints` | `Hint[]` | Yes* | Progressive nudges on demand. Use **or** guides, not both. |
| `unlocks` | `string[]` | Yes | Tactic names this level unlocks. Use `[]` for practice levels. |

\* Provide `guides` **or** `hints`, not both. Guides are for teaching a new tactic (hand-holding). Hints are for synthesis/practice (student thinks independently).

**Notes:**
- The `id` field for problems is assigned automatically by discovery order — do **not** include it.
- Omit `logicMode` unless you need to override the section default.

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

When the student types the final step, the engine checks that the conclusion matches this string **structurally** (same s-expression tree, ignoring whitespace). Conjunction and disjunction are commutative — `(and A B)` equals `(and B A)`.

### `premise` (array of strings)

Formulas that are **given** — the student must `assume` them one by one before they can be used. Displayed as "PREMISE" labels above the goal.

**Examples:**
- `[]` — nothing given; the student makes their own assumptions
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
| `tactic` | `string` or `null` | No | The exact command to type. Omit or `null` for prose-only. |

- **In guides**: a `tactic` locks the input until the student types the exact match. No `tactic` shows an "OK" button.
- **In hints**: a `tactic` shows a "fill in" button that inserts the command. No `tactic` shows only prose.

---

## Formula syntax

Formulas are written as **s-expressions** (Lisp-style parenthesized prefix notation). Outermost parentheses are required for compound formulas.

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
- A **constant** — any bare word not introduced by `fix`, e.g. `socrates`, `a`, `b`
- A **function application** — `(App f t)` for unary, `(App (App f x) y)` for nested. Function symbols follow the same syntax as predicate symbols; the engine distinguishes them by context.

Variables and constants are both bare atoms — a symbol is a variable if it was introduced by `fix` in the current proof; otherwise it's treated as a constant.

#### Predicate application

Predicates are applied to terms using nested `App` (currying).

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

The engine converts formulas to LaTeX automatically. Compound sub-expressions are wrapped in parentheses:

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

Plato's engine supports two axiom sets. A colored chip in the navbar (blue for FOL on, orange for FOL off) tells the player which set is active.

### Why two separate sets?

Plato lives at the intersection of three logical systems:

- **Propositional logic (PL)** — connectives $\lnot$, $\land$, $\lor$, $\to$, $\top$, $\bot$. Decidable (a truth table can settle any statement).
- **First-order logic (FOL)** — adds $\forall$, $\exists$, and predicate application. Semi-decidable, well-behaved for natural deduction.
- **Modal logic (System K)** — adds $\Box$ and $\Diamond$. Decidable on its own (translatable into a fragment of FOL).

The problem is **first-order modal logic (FOML)** — combining FOL with $\Box$ and $\Diamond$ makes the system undecidable. Even the monadic fragment is undecidable (unlike monadic FOL). There is no single canonical semantics — different commitments about domains across possible worlds produce irreconcilably different logics.

Plato's solution: **don't combine them.** Each section picks one mode and stays within it.

### PL mode (`"logicMode": "pl"`)

Available: all propositional connectives plus modal operators ($\Box$, $\Diamond$).

Disabled: quantifiers ($\forall$, $\exists$), predicate application (`App`), `fix`.

Used by: **Propositional Logic** section and **Modal Logic** section.

### FOL mode (`"logicMode": "fol"`)

Available: all propositional connectives plus quantifiers, predicate application, and `fix`.

Disabled: modal operators ($\Box$, $\Diamond$).

Used by: **First-Order Logic** section.

### Setting the axiom set

- **Section default**: set in `section.json` via `"logicMode"`. All levels inherit it.
- **Per-level override**: add `"logicMode"` to a level JSON. Rare for built-in levels; useful for standalone levels.
- **Standalone levels** (loaded via URL/file): should always include `"logicMode"`. If omitted, defaults to `"fol"`.

---

## Tactic reference

Every command the proof engine accepts. All tactics use **1-based step numbers** — the first proof step is step 1.

### assume

```
(assume F)
```

Creates a new assumption: $\{F\} \vdash F$. The fundamental way to bring a formula into the context.

**Example:** `(assume A)` → step 1: $\{A\} \vdash A$

### extend

```
(extend F N)
```

**Weakening.** Adds formula $F$ to step N's context: if step N proves $\Gamma \vdash p$, produces $\Gamma, F \vdash p$.

Aliases: `weaken`

**Example:** `(extend B 1)` — step 1 proves $\{A\} \vdash A$ → $\{A, B\} \vdash A$

### and-intro

```
(and-intro N M)
```

Combines steps N and M into a conjunction. Contexts are merged.

**Example:** `(and-intro 1 2)` — step 1 proves $A$, step 2 proves $B$ → $A \land B$

### and-elim-l / and-elim-r

```
(and-elim-l N)
(and-elim-r N)
```

Extract the left or right half of a conjunction.

**Example:** `(and-elim-l 1)` — step 1 proves $A \land B$ → $A$

### or-intro-l / or-intro-r

```
(or-intro-l N F)
(or-intro-r N F)
```

Introduce a disjunction. `or-intro-l` puts N's conclusion on the **left** with F on the right. `or-intro-r` puts F on the **left** with N's conclusion on the right.

**Example:** `(or-intro-l 1 B)` — step 1 proves $A$ → $A \lor B$
**Example:** `(or-intro-r 1 A)` — step 1 proves $B$ → $A \lor B$

### or-elim

```
(or-elim N M K)
```

**Proof by cases.** Step N is $p \lor q$. Steps M and K both prove the **same** conclusion $r$ — one from $p$, the other from $q$. Produces $r$.

**Example:** `(or-elim 1 3 5)` — step 1: $A \lor B$, step 3 proves $C$ from $A$, step 5 proves $C$ from $B$ → $C$

### ->-intro

```
(->-intro F N)
```

**Implication introduction.** Discharges $F$ from step N's context: $\Gamma, F \vdash p$ → $\Gamma \vdash F \to p$.

Aliases: `imp-intro`, `→-intro`

**Example:** `(->-intro A 2)` — step 2 has $\{A, B\} \vdash C$ → $\{B\} \vdash A \to C$

### ->-elim

```
(->-elim N M)
```

**Modus ponens.** Step N is $p \to q$. Step M is $p$. Produces $q$.

Aliases: `imp-elim`, `→-elim`, `modus-ponens`, `mp`

**Example:** `(->-elim 2 1)` — step 2: $A \to B$, step 1: $A$ → $B$

### not-intro

```
(not-intro F N M)
```

**Reductio ad absurdum.** $F$ is in both steps' contexts. N and M reach contradictory conclusions. Produces $\lnot F$ with $F$ discharged.

Aliases: `¬-intro`, `neg-intro`, `raa`

### not-elim

```
(not-elim N)
```

Step N is $\lnot p$. Produces $\bot$.

Aliases: `¬-elim`, `neg-elim`

### dne

```
(dne N)
```

**Double negation elimination.** Step N is $\lnot \lnot p$. Produces $p$.

Aliases: `dneg-elim`, `¬¬-elim`, `double-neg-elim`

### ex-falso

```
(ex-falso N F)
```

**Principle of explosion.** From $\bot$ (step N), derive **any** $F$.

Aliases: `efq`, `explosion`

### top-intro

```
(top-intro)
(top-intro N)
```

**Truth introduction.** $\top$ is always provable. No argument: $\emptyset \vdash \top$. With N: $\top$ in N's context.

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

**Uniform substitution.** Replace atom A with formula F in step N's conclusion. Multiple `(atom formula)` pairs allowed.

Aliases: `substitute`

**Example:** `(subst 1 (P (and A B)))` — step 1 proves $P \to P$ → $(A \land B) \to (A \land B)$

### fix

```
(fix x)
```

Introduce a fresh term variable: $\{x\} \vdash x$. Must be fresh (not free in existing assumptions). **FOL mode only.**

**Example:** `(fix x)` → step 1: $\{x\} \vdash x$

### forall-intro

```
(forall-intro x N)
```

**Universal generalisation.** From $\varphi$ proved for arbitrary $x$ (not free in other assumptions), produce $\forall x.\,\varphi$. **FOL mode only.**

Aliases: `∀-intro`, `forall`

**Example:** `(forall-intro x 3)` — step 3 proves $P(x)$ from $\{x\}$ → $\forall x.\,P(x)$

### forall-elim

```
(forall-elim N t)
```

**Universal instantiation.** Step N is $\forall x.\,\varphi$. Substitute term $t$ for $x$. **FOL mode only.**

Aliases: `∀-elim`

**Example:** `(forall-elim 1 socrates)` — step 1: $\forall x.\,\text{Mortal}(x)$ → $\text{Mortal}(\text{socrates})$

### exists-intro

```
(exists-intro N t x)
```

**Existential generalisation.** From $\varphi[t/x]$ (step N), produce $\exists x.\,\varphi$. **FOL mode only.**

Aliases: `∃-intro`

**Example:** `(exists-intro 2 socrates x)` — step 2 proves $\text{Mortal}(\text{socrates})$ → $\exists x.\,\text{Mortal}(x)$

### exists-elim

```
(exists-elim N M x)
```

**Existential witness elimination.** Step N proves $\exists x.\,\varphi$. Step M proves $\psi$ from a fresh witness $x$ satisfying $\varphi$. $x$ must not be free in $\psi$ or open assumptions. **FOL mode only.**

Aliases: `∃-elim`

**Example:** `(exists-elim 1 3 y)` — step 1: $\exists x.\,P(x)$, step 3 proves $Q$ from $\{y, P(y)\}$ → $Q$

### box-intro

```
(box-intro N)
```

**Necessitation.** Step N must prove $P$ with an **empty context**. Produces $\Box P$. **PL mode only** (modal section).

Aliases: `□-intro`, `nec`

### box-elim

```
(box-elim N M)
```

**K axiom.** Step N: $\Box(P \to Q)$. Step M: $\Box P$. Produces $\Box Q$. **PL mode only** (modal section).

Aliases: `□-elim`, `k`

### diamond-def

```
(diamond-def N)
```

**Diamond definition (unfold).** Step N is $\Diamond P$. Produces $\lnot\Box\lnot P$. **PL mode only** (modal section).

Aliases: `◇-def`, `dia-def`

### diamond-def-rev

```
(diamond-def-rev N)
```

**Diamond definition (fold).** Step N is $\lnot\Box\lnot P$. Produces $\Diamond P$. **PL mode only** (modal section).

Aliases: `◇-def-rev`, `dia-def-rev`

---

## Text markup

The `text` field in guides, hints, and the `description` field support inline markup:

| Syntax | Rendered as |
|---|---|
| `$latex$` | Inline KaTeX math, e.g. `$A \land B$` |
| `$$latex$$` | Display-style KaTeX (centered, larger) |
| `**bold**` | **Bold text**, supports nested markup |
| `` `code` `` | Monospace code, usually for tactic names |
| `[id\|display]` | Glossary link — opens HelpModal glossary scrolled to entry `id` |
| `[id]` | Glossary link using `id` as display text |

**Important:** In inline LaTeX, use double backslashes: `$A \\land B$`, `$\\vdash$`, `$\\Gamma$`, `$\\Box$`.

**Example:**
```
"The goal is $A \\to B$. Use `->-intro` to **discharge** the [assumption|assumption]."
```

---

## Writing guides

Guides are for levels that **introduce a new tactic**. The student is walked through step by step. Each guide card appears one at a time before the student types.

### Rules

1. First guide: explain the goal in plain language. No tactic (`"tactic": null`).
2. Each subsequent guide: one concept or one proof step.
3. A guide **with** a `tactic` locks the input until the student types the exact match.
4. A guide **without** a `tactic` shows an "OK" button; the student clicks to advance.
5. Final guide: conclude the proof and celebrate.
6. A level with guides should have `"hints": []`.

### Example

```json
{
  "guides": [
    {
      "text": "We're proving $A \\land B \\to A$. If both $A$ and $B$ are true, then certainly $A$ is true.",
      "tactic": null
    },
    {
      "text": "As always with an implication, we start by assuming the left-hand side: $A \\land B$.",
      "tactic": "(assume (and A B))"
    },
    {
      "text": "Now we have $A \\land B$ at step 1. Use `and-elim-l` to extract the **left** half.",
      "tactic": "(and-elim-l 1)"
    },
    {
      "text": "We have $A$ under the assumption $A \\land B$. Use `->-intro` to discharge and close.",
      "tactic": "(->-intro (and A B) 2)"
    }
  ]
}
```

### Tone

- Conversational, second person ("we", "our job").
- Slightly witty, never at the expense of clarity.
- Introduce notation gently: "The symbol $\\vdash$ is pronounced 'entails'."
- Acknowledge what's new: "This time the left-hand side is a conjunction…"

---

## Writing hints

Hints are for **synthesis/practice levels** where the student should solve independently but can request help.

### Rules

1. Hints appear one at a time — each new lightbulb appears when the previous hint is opened.
2. Early hints: point in the right direction without giving the tactic.
3. Middle hints: narrow the approach.
4. Later hints: give the exact tactic.
5. Final hint: give the final tactic.
6. Each hint assumes the previous hints were followed.
7. A level with hints should have `"guides": []`.

### Step numbers

Refer to step numbers **as they will be at that point in the proof**. Steps are 1-based. Calculate carefully — wrong step numbers confuse.

### Example

```json
{
  "hints": [
    {
      "text": "The goal is $(A \\land (A \\to B)) \\to B$. Extract both parts of the conjunction, then modus ponens.",
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
      "text": "Now the right half — that's $A \\to B$.",
      "tactic": "(and-elim-r 1)"
    },
    {
      "text": "You have $A \\to B$ at step 3 and $A$ at step 2. Use modus ponens.",
      "tactic": "(->-elim 3 2)"
    },
    {
      "text": "Discharge the assumption.",
      "tactic": "(->-intro (and A (-> A B)) 4)"
    }
  ]
}
```

---

## Writing discoveries

A discovery is a short illustrated dialogue between two historical logicians that introduces a section's concepts. It lives as a JSON file in `levels/` with `"type": "discovery"`.

The first level of every section (`000-discovery.json`) should be a discovery. Additional discoveries can be interspersed between proof levels (e.g. to introduce a new concept partway through a section).

### Schema

```json
{
  "type": "discovery",
  "title": "The Nature of Truth",
  "lines": [
    {
      "speaker": "Plato",
      "text": "Aristotle, my dear friend. Let us speak of truth itself.",
      "sid": "plato",
      "image": null
    },
    {
      "speaker": "Aristotle",
      "text": "That a horse is a horse. A thing is itself.",
      "sid": "aristotle",
      "image": null
    }
  ]
}
```

| Field | Type | Description |
|---|---|---|
| `type` | `"discovery"` | Identifies this as a discovery level. |
| `title` | `string` | Heading shown above the dialogue. |
| `lines` | `array` | Dialogue lines, rendered one at a time with a typewriter effect. |
| `lines[].speaker` | `string` | Character name displayed in the UI. |
| `lines[].text` | `string` | What the character says. Supports text markup. |
| `lines[].sid` | `string` | Short speaker ID used for CSS scoping (e.g. `plato`, `aristotle`, `frege`). |
| `lines[].image` | `string` or `null` | Optional portrait image path. `null` for no image. |

---

## Unlocking tactics

Each proof level can unlock tactics via the `unlocks` array. When solved, these tactic names are added to the player's collection (persisted in `localStorage`). Full metadata lives in `tactics.json`.

```json
"unlocks": ["->-elim"]
```

### Progression guidelines

- **One tactic per 1–2 levels.** Don't dump five new rules at once.
- **Follow every introduction with a synthesis level** (`"unlocks": []`) using the new tactic alongside previously learned ones.
- **Unlock symmetric rules together** — `and-elim-l` and `and-elim-r` in one level, `or-intro-l` and `or-intro-r` in one level. The student may only use one but knowing the other exists is important.

### Recommended introduction order

**Propositional Logic section** (PL mode):

1. `assume`, `->-intro` — the foundational moves
2. `and-intro` — building conjunctions
3. (synthesis)
4. `and-elim-l`, `and-elim-r` — breaking conjunctions
5–7. (practice)
8. `->-elim` — modus ponens
9–10. (practice)
11. `or-intro-l`, `or-intro-r` — building disjunctions
12–13. (practice)
14. `or-elim` — proof by cases
15–16. (practice)
17+. `not-intro`, `not-elim`, `dne`, `ex-falso`, `extend`, `subst`, `top-intro`

**First-Order Logic section** (FOL mode):

- `fix` — introduce fresh term variables
- `forall-intro`, `forall-elim` — universal quantifier rules
- `exists-intro`, `exists-elim` — existential quantifier rules

**Modal Logic section** (PL mode):

- `box-intro` — necessitation ($\emptyset \vdash P$)
- `box-elim` — K axiom
- `diamond-def` — unfold $\Diamond$
- `diamond-def-rev` — fold back to $\Diamond$

---

## Complete example

A full level that introduces `->-elim` (modus ponens):

```json
{
    "type": "problem",
    "description": "Some truths follow others. Let one follow.",
    "goal": "B",
    "premise": ["A", "(-> A B)"],
    "guides": [
        {
            "text": "We're given $A$ and $A \\to B$. Our goal is to reach $B$. This is as direct as reasoning gets.",
            "tactic": null
        },
        {
            "text": "Bring the premise $A$ into the context.",
            "tactic": "(assume A)"
        },
        {
            "text": "Now bring in $A \\to B$. Whenever $A$ holds, $B$ follows.",
            "tactic": "(assume (-> A B))"
        },
        {
            "text": "You have $A \\to B$ at step 2 and $A$ at step 1. Use `->-elim` — **modus ponens**.",
            "tactic": "(->-elim 2 1)"
        },
        {
            "text": "And $B$ is yours. No implication to discharge — the premises were given, not assumed for argument.",
            "tactic": null
        }
    ],
    "hints": [],
    "unlocks": ["->-elim"]
}
```

---

## Section config

Each section folder contains a `section.json`:

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
| `nameI18nKey` | `string` | i18n key for the section name |
| `logicMode` | `"pl"` or `"fol"` | Default axiom set. Can be overridden per-level. |
| `order` | `number` | Progression order (0, 1, 2…). Sections unlock sequentially. |
| `allowedTactics` | `string[]` | Commands enabled in this section. Filters autocomplete, sidebar, and help modal. |

### Adding a new section

1. Create `src/data/{locale}/sections/{id}/` for each locale.
2. Add `section.json` with `logicMode`, `order`, and `allowedTactics`.
3. Add a `levels/` subfolder:
   - `000-discovery.json` — the section's discovery dialogue
   - `001-first-problem.json`, `002-second-problem.json`, …
4. No code changes required — auto-discovered via `import.meta.glob`.

### Standalone levels

For levels loaded via URL or file (not part of a section), always include `"logicMode"`:

```json
{
  "type": "problem",
  "description": "A modal logic proof",
  "goal": "(box A)",
  "premise": ["A"],
  "logicMode": "pl",
  "guides": [],
  "hints": [],
  "unlocks": []
}
```

---

## Design principles

1. **Scaffold, then remove.** Guided levels hold the student's hand. Synthesis levels make them think. The combo challenge tests everything.

2. **One new idea at a time.** Each guide level introduces one rule.

3. **Explain the "why".** Don't just state the rule — explain what it *means*.

4. **Proofs should be clean.** If a solution requires 15 steps of contortion, redesign.

5. **Prefer implication-shaped goals.** $P \to Q$ is pedagogically clean — assume $P$, prove $Q$, discharge. Works naturally with empty premises.

6. **Context is invisible but important.** `->-intro` discharges one formula; `or-elim` unions three contexts; `extend` adds a formula. Be aware of what's in scope at each step.

7. **Check commutativity.** `and`/`or` are commutative; `->` is not. Affects goal matching.

8. **Test your level.** Run the app and solve it yourself. Verify every guide/hint tactic works, step numbers are correct, and the goal resolves.
