# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Plato is a gamified natural deduction proof assistant — users prove logical statements step-by-step using inference rules in an s-expression REPL. It is a pnpm monorepo with two packages:

- **`plato-lib`** — Rust library compiled to WASM (`wasm-pack`) implementing the natural deduction engine (formulas, contexts, judgements, inference rules, S-expression parser).
- **`plato-site`** — Vue 3 + Vite + TypeScript SPA providing the UI: REPL, 38 guided proof problems, hint system, and tactic collection.

## Commands

```bash
# First-time setup & after Rust changes
pnpm run build:wasm          # Build plato-lib to WASM (must run before dev/build of plato-site)

# Development (from root or packages/plato-site)
pnpm --filter plato-site dev # Start Vite dev server with HMR
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

No vue-router. `App.vue` uses a `ref<Page>` discriminated union (`{ type: 'home' }` | `{ type: 'problem'; idx: number }`) with `<Transition>` for page switching. Views emit events (`@start`, `@continue`, `@next`, `@prev`, `@home`) that `App.vue` handles to change pages.

### State (Pinia stores)

| Store | What it holds | Persistence |
|---|---|---|
| `progress.ts` | `highestSolved` index | `localStorage` key `plato-highest` |
| `tactics.ts` | `Map<string, Tactic>` of collected tactics | In-memory (lost on reload) |
| `preferences.ts` | `viewMode: 'tex' \| 'text'` | In-memory |

A debug cheat `window.__plato_unlockAll__` unlocks all problems and tactics (exposed in `main.ts`).

### WASM boundary

The Rust crate compiles to WASM via `wasm-pack build --target web --release`. JS-side composables (`useProofSession`, `useProblemLatex`) lazy-load it:

```ts
const mod = await import('plato-lib')
await mod.default()  // WASM init
```

The `Session` class is the main JS API: `execute(tactic)`, `stepLatex(n)` / `stepText(n)`, `formulaLatex(s)`, `isGoalResolved(goal)`, `tacticEquals(a, b)`.

### Adding a new problem

Create a JSON file in `src/data/problems/` matching the `Problem` type (`src/types.ts`). The `import.meta.glob` in `index.ts` auto-discovers it. Filename sort order determines the problem number (IDs are assigned at runtime, 1-based).

Problem JSON schema: `{ description, premise: string[], goal: string, guides: Hint[], hints: Hint[], unlocks: Tactic[] }`. Guide cards are shown as an enforced step-by-step walkthrough; hints are optional reveals. See existing problems for the S-expression format.

### Adding a new inference rule

Requires coordinated changes:
- **Rust**: Add the rule implementation in `plato-lib/src/rules/`, register it in the parser (`parser/command.rs`) so the s-expression command is recognized, and expose any new JS bindings in `lib.rs`.
- **Frontend**: Add the command entry to `HelpModal.vue` (grouped by category), and potentially update the problem JSON files that should teach it.

### Component tree

```
App.vue
├── HomeView.vue
└── ProblemView.vue
    ├── PreferenceModal.vue
    ├── ProofRepl.vue
    │   ├── GuideCard.vue
    │   ├── HintCard.vue
    │   └── HelpModal.vue
    └── TacticSidebar.vue
        └── TacticCard.vue (×N)
```

### Responsive breakpoint

TacticSidebar switches from inline sidebar to floating button + slide-in overlay at `<= 820px`.

### Styling

CSS custom properties in `style.css` define the design system. Single monospace font stack (JetBrains Mono). KaTeX for math rendering. Yellow background for hint cards.

### CI/CD

GitHub Actions (`deploy.yml`) builds WASM + site and deploys to GitHub Pages on push to `main`.

## Testing

No test framework is configured on the frontend. The Rust crate has parser tests in `packages/plato-lib/src/parser/tests.rs` runnable with `cargo test`.
