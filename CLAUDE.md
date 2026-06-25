# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Plato is a gamified natural deduction proof assistant — users prove logical statements step-by-step using inference rules in an s-expression REPL. It is a pnpm monorepo with two packages:

- **`plato-lib`** — Rust library compiled to WASM (`wasm-pack`) implementing the natural deduction engine (formulas, contexts, judgements, inference rules, S-expression parser).
- **`plato-site`** — Vue 3 + Vite + TypeScript SPA providing the UI: REPL, 45 guided proof problems, hint system, tactic collection, custom problem loader.

## Commands

```bash
# First-time setup & after Rust changes
pnpm run build:wasm          # Build plato-lib to WASM (must run before dev/build of plato-site)

# Development (from root or packages/plato-site)
pnpm --filter plato-site dev     # Start Vite dev server with HMR
pnpm --filter plato-site build   # Type-check + production build
pnpm --filter plato-site preview # Preview production build locally

# Lint & format
pnpm --filter plato-site lint     # oxlint + eslint
pnpm --filter plato-site format   # oxfmt src/

# Rust (from packages/plato-lib)
cd packages/plato-lib && cargo test   # Run Rust parser tests
```

Node requirement: `^22.18.0 || >=24.12.0`. Package manager: `pnpm` (workspaces).

## Architecture

### Navigation

No vue-router. `App.vue` uses a `ref<Page>` discriminated union with three variants:
```ts
type Page =
    | { type: 'home' }
    | { type: 'problem'; idx: number }
    | { type: 'custom' }
```

Views emit events (`@start`, `@continue`, `@next`, `@prev`, `@home`, `@go-custom`) that `App.vue` handles to change pages. `<Transition>` provides page transitions.

### i18n

`vue-i18n` with `en` (default) and `zh` locales. Locale is persisted in `localStorage` key `plato-locale`. The `InlineLatex` component renders problem description/hint text with inline markup: `$latex$`, `**bold**`, `` `code` ``, and `[glossary id|display text]` links that open the HelpModal glossary tab.

### State (Pinia stores)

| Store | What it holds | Persistence |
|---|---|---|
| `progress.ts` | `highestSolved` index | `localStorage` key `plato-highest` |
| `tactics.ts` | `collected: string[]` of tactic names | `localStorage` key `plato-tactics` |
| `preferences.ts` | `viewMode: 'tex' \| 'text'`, `locale: string` | `locale` in `localStorage` key `plato-locale` |
| `roadmap.ts` | `entries: RoadmapEntry[]` — proof history (idx, description, goal, proofLines) | `localStorage` key `plato-roadmap` |

A debug cheat `window.__plato_unlockAll__` unlocks all problems and tactics (exposed in `main.ts`).

### WASM boundary

The Rust crate compiles to WASM via `wasm-pack build --target web --release`. JS-side composables lazy-load it:

```ts
const mod = await import('plato-lib')
await mod.default()  // WASM init
```

The `Session` class is the main JS API: `execute(tactic)`, `stepLatex(n)` / `stepText(n)`, `formulaLatex(s)`, `isGoalResolved(goal)`, `tacticEquals(a, b)`. The `useProofSession` composable owns the WASM session lifecycle and provides the reactive `entries`, `run`, `reset`, `stepLatex`, etc. that `ProofRepl` consumes.

### Inline text parsing

`src/utils/inline-parse.ts` parses hint/guide text into `Segment[]` with these types:

| Pattern | Type | Example |
|---|---|---|
| `[id\|display]` | `glossary` | `[context\|context]` — opens glossary entry `context`, shows "context" |
| `[id]` | `glossary` | `[modus-ponens]` — shows and links the term |
| `$...$` | `latex` | `$A \to B$` |
| `**...**` | `bold` | Supports nested segments inside |
| `` `...` `` | `code` | `(->-elim 1 2)` |

`InlineLatex.vue` renders these segments, emitting `glossaryClick` events for glossary links. These bubble up to `ProofRepl` → `ProblemView` which opens `HelpModal` on the glossary tab scrolled to the target term.

### Adding a new problem

Create a JSON file in `src/data/en/problems/` and `src/data/zh/problems/` matching the `Problem` type (`src/types.ts`). The `import.meta.glob` in `src/data/index.ts` auto-discovers it. Filename sort order determines the problem number (IDs are assigned at runtime, 1-based).

Problem JSON schema: `{ description, premise: string[], goal: string, guides: Hint[], hints: Hint[], unlocks: Tactic[] }`. Each `Hint` has `{ text: string, tactic: string | undefined }`. Guide cards are an enforced step-by-step walkthrough (shown one at a time); hints are optional reveals. Descriptions and hint/guide text use the inline markup syntax described above.

### Adding a new inference rule

Requires coordinated changes:
- **Rust**: Add the rule implementation in `plato-lib/src/rules/`, register it in the parser (`parser/command.rs`) so the s-expression command is recognized, and expose any new JS bindings in `lib.rs`.
- **Frontend**: Add the command entry to `HelpModal.vue` (grouped by category), add it to `tactics.json` for each locale, and potentially update the problem JSON files that should teach it.

### Component tree

```
App.vue
├── HomeView.vue
├── ProblemView.vue
│   ├── PreferenceModal.vue
│   ├── ProofRepl.vue
│   │   ├── GuideCard.vue
│   │   ├── HintCard.vue
│   │   └── HelpModal.vue
│   ├── TacticSidebar.vue
│   │   └── TacticCard.vue (×N)
│   └── RoadmapModal.vue
└── CustomProblemView.vue
    ├── PreferenceModal.vue
    ├── ProofRepl.vue
    ├── TacticSidebar.vue
    └── (no RoadmapModal — custom problems don't write to roadmap)
```

### Responsive breakpoint

`TacticSidebar` switches from inline sidebar to floating button + slide-in overlay at `<= 820px`.

### Styling

CSS custom properties in `style.css` define the design system. Single monospace font stack (JetBrains Mono). KaTeX for math rendering. Yellow background for hint cards. Glossary links in body text are styled with dotted underline and open the HelpModal glossary tab.

### CI/CD

GitHub Actions (`deploy.yml`) builds WASM + site and deploys to GitHub Pages on push to `main`.

## Testing

No test framework is configured on the frontend. The Rust crate has parser tests in `packages/plato-lib/src/parser/tests.rs` runnable with `cargo test`.
