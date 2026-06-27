# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Plato is a gamified natural deduction proof assistant — users prove logical statements step-by-step using inference rules in an s-expression REPL. It is a pnpm monorepo with two packages:

- **`plato-lib`** — Rust library compiled to WASM (`wasm-pack`) implementing the natural deduction engine (formulas, contexts, judgements, inference rules, S-expression parser).
- **`plato-site`** — Vue 3 + Vite + TypeScript SPA providing the UI: REPL, guided proof problems, hint system, tactic collection, custom problem loader.

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

### Routing

Uses `vue-router` with hash history (`createWebHashHistory`). Routes defined in `src/router.ts`:

| Route | View | Notes |
|---|---|---|
| `/` | `HomeView` | |
| `/section/:sectionId/level/:idx` | `LevelView` | Unified route — handles both discovery and problem levels |
| `/custom` | `CustomProblemView` | |
| `/about` | `AboutView` | Renders locale-specific `ABOUT.md` |
| `/locked` | `LockedView` | Shown when a level is not yet unlocked |
| `/section/:id/discovery` | — | Legacy redirect to `/section/:id/level/0` |
| `/section/:id/problem/:idx` | — | Legacy redirect (problem index → level index + 1) |
| `/problem/:idx` | — | Legacy redirect (global index → level route) |
| `/:pathMatch(.*)*` | `NotFoundView` | Catch-all |

### Navigation flow

A section is a linear sequence of **levels** — a mixed array of discovery dialogues and proof problems. The player navigates through them with prev/next buttons. A single `LevelView` component handles both types: discovery levels render `DiscoveryDialog`, problem levels render the proof prompt and `ProofRepl`. Level 0 is always a discovery. Progress is tracked by level index (0-based within the section), persisted as `highestCompletedLevel` in the progress Pinia store.

### Sections & data

Problems are organized into sections (e.g., `propositional`, `first-order`, `modal`). Each section lives under `src/data/{locale}/sections/{section-id}/`:

```
sections/
  propositional/
    section.json        # SectionMeta: nameI18nKey, logicMode, allowedTactics, order
    discovery.json      # DiscoveryData: title, lines (speaker + text)
    problems/
      01-identity.json  # Problem: description, premise, goal, guides, hints, unlocks
      ...
```

`src/data/index.ts` uses `import.meta.glob` to auto-discover sections, levels, glossary, tactics, and NLG data across locales. Key exports: `loadSections()`, `getSection()`, `getNextSection()`, `resolveGlobalIndex()`, `loadGlossary()`, `loadTactics()`. Also provides `problemIdxToLevelIdx()` and `levelIdxToProblemIdx()` for converting between 0-based problem indices (skipping discoveries) and the raw level array index.

`src/types.ts` defines the core types: `Problem`, `Hint`, `Tactic`, `SectionMeta`, `DiscoveryLine`, `DiscoveryData`, `Section`.

### Logic modes

Each section has a `logicMode` in its `section.json` — either `"pl"` (propositional logic, FOL off) or `"fol"` (first-order logic, FOL on). Individual problems can override via their own `logicMode` field. The mode is displayed in the NavBar as a colored chip:

- `--color-fol-on` (`#22c55e`, green) — FOL ON
- `--color-fol-off` (`#f97316`, orange) — FOL OFF

The mode controls which tactics are relevant; quantifier tactics (forall-intro, forall-elim, exists-intro, exists-elim) only apply in FOL mode.

### i18n

`vue-i18n` with `en` (default) and `zh` locales. Locale is persisted in `localStorage` key `plato-locale`. The `InlineLatex` component renders problem description/hint text with inline markup: `$latex$`, `**bold**`, `` `code` ``, and `[glossary id|display text]` links that open the HelpModal glossary tab.

### State (Pinia stores)

| Store | What it holds | Persistence |
|---|---|---|
| `progress.ts` | `highestSolved` index | `localStorage` key `plato-highest` |
| `tactics.ts` | `collected: string[]` of tactic names | `localStorage` key `plato-tactics` |
| `preferences.ts` | `viewMode: 'tex' \| 'text'`, `locale: string` | `locale` in `localStorage` key `plato-locale` |
| `roadmap.ts` | `entries: RoadmapEntry[]` — proof history (sectionId, sectionIdx, description, goal, proofLines) | `localStorage` key `plato-roadmap` |
| `discovery.ts` | `viewed: Record<string, boolean>` — which section discoveries have been seen; `position: Record<string, number>` — saved scroll/line position | `localStorage` key `plato-discoveries` |

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

Create a JSON file in the appropriate section's `problems/` directory under each locale (e.g., `src/data/en/sections/propositional/problems/` and `src/data/zh/sections/propositional/problems/`). The `import.meta.glob` in `src/data/index.ts` auto-discovers it. Filename sort order determines the problem index within the section (0-based).

Problem JSON schema: `{ description, premise: string[], goal: string, guides: Hint[], hints: Hint[], unlocks: Tactic[] }`. Each `Hint` has `{ text: string, tactic: string | undefined }`. Guide cards are an enforced step-by-step walkthrough (shown one at a time); hints are optional reveals. Descriptions and hint/guide text use the inline markup syntax described above.

### Adding a new section

Create a new directory under `src/data/en/sections/` (and `zh/sections/`) with at minimum `section.json` and `problems/`. Optionally add `discovery.json`. The section must have a unique `order` in `section.json` and an `i18nKey` for localization.

### Adding a new inference rule

Requires coordinated changes:
- **Rust**: Add the rule implementation in `plato-lib/src/rules/`, register it in the parser (`parser/command.rs`) so the s-expression command is recognized, and expose any new JS bindings in `lib.rs`.
- **Frontend**: Add the command entry to `HelpModal.vue` (grouped by category), add it to `tactics.json` for each locale, and potentially update the problem JSON files that should teach it.

### Component tree

```
App.vue
├── HomeView.vue
├── LevelView.vue
│   ├── NavBar.vue              (with axiom-set popup chip on problem levels)
│   ├── DiscoveryDialog.vue     (discovery levels)
│   ├── PreferenceModal.vue
│   ├── HelpModal.vue           (triggered from NavBar ? button)
│   ├── ProofRepl.vue           (problem levels)
│   │   ├── GuideCard.vue
│   │   └── HintCard.vue
│   ├── TacticSidebar.vue
│   │   └── TacticCard.vue (×N)
│   └── RoadmapModal.vue
├── CustomProblemView.vue
│   ├── PreferenceModal.vue
│   ├── ProofRepl.vue
│   └── TacticSidebar.vue
├── AboutView.vue
│   ├── NavBar.vue
│   ├── PreferenceModal.vue
│   └── HelpModal.vue
├── LockedView.vue
└── NotFoundView.vue
```

`LevelView` is the workhorse — handles both discovery and problem levels via a discriminated union on the level type. Discovery levels show a progress bar and `DiscoveryDialog`; problem levels show the proof prompt, `ProofRepl`, victory overlay, and axiom chip popup. `NavBar.vue` (logo, slot, `?` help button, GitHub link, preferences button) is used in LevelView, AboutView, and CustomProblemView. `HelpModal.vue` has three tabs (Commands, Notations, Glossary).

### Responsive breakpoint

`TacticSidebar` switches from inline sidebar to floating button + slide-in overlay at `<= 820px`.

### Styling

CSS custom properties are defined in `src/style.scss`. Single monospace font stack (JetBrains Mono). KaTeX for math rendering. Yellow background for hint cards. Glossary links in body text are styled with dotted underline and open the HelpModal glossary tab.

Notable custom properties beyond the typical foreground/background set:

| Variable | Value | Usage |
|---|---|---|
| `--color-fol-on` | `#22c55e` | FOL ON chip in NavBar / HelpModal |
| `--color-fol-off` | `#f97316` | FOL OFF chip in NavBar / HelpModal |
| `--color-glossary-flash` | `#cb9559` | Flash highlight when deep-linking to glossary term |
| `--color-hint-bg` | `#fefef0` | Hint card background |
| `--color-hint-border` | `#f0f0d0` | Hint card border |

### CI/CD

GitHub Actions (`deploy.yml`) builds WASM + site and deploys to GitHub Pages on push to `main`.

## Testing

No test framework is configured on the frontend. The Rust crate has parser tests in `packages/plato-lib/src/parser/tests.rs` runnable with `cargo test`.
