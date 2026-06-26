# About Plato

> *He was not only the best of men, but also the wisest.*

Plato is a proof assistant disguised as a puzzle game. It is named after the philosopher who believed that true knowledge comes from reasoning — not from being told.

I built this because I wanted a place where logic feels like play. Most proof assistants are intimidating: they speak in arcane syntax, demand hundreds of pages of reading, and assume you already have a PhD. Plato starts from the other end. You type `(assume A)`. You watch a new line appear. Something clicks. You type a little more. Suddenly you're thinking in implications.

The idea isn't to teach you *about* logic — it's to teach you to *do* logic. And the best way to learn is by getting your hands dirty, making mistakes, and slowly developing an instinct for what a proof feels like.

## How it works

You are shown a statement to prove. You type commands in an s-expression REPL — `(assume A)`, `(->-elim 2 1)`, `(and-intro 1 3)` — and the engine checks each step against the rules of natural deduction. Guide cards walk you through your first proofs. Hints appear when you're lost. Every solved problem unlocks new tactics, building your toolkit one rule at a time.

There are **57 problems** across three sections — Propositional Logic, First-Order Logic, and Modal Logic (System K). Each section operates in its own **axiom set** (PL or FOL, shown in the NavBar), which determines which commands are available: propositional connectives only, or with quantifiers, or with modal operators. Problems range from the law of identity (three steps, two minutes) to the K axiom of modal logic and the distribution of necessity over implication.

You can also **load your own problems** from a URL or a JSON file — the engine doesn't care where the goal came from.

## Why natural deduction?

Because natural deduction mirrors how humans actually reason.

In most logic classes, you learn truth tables. You fill out rows, you check columns. It works, but it doesn't feel like *thinking*. Natural deduction is different: you assume things, you derive conclusions, you discharge assumptions. Each step corresponds to a real piece of reasoning — "if I knew *p*, then I'd know *q*." You're not computing a truth value; you're building an argument.

## A note on difficulty

Natural deduction is legitimately hard. There is no algorithm for constructing a proof — you have to develop a feel for it. Problems that look simple can be genuinely challenging. If you get stuck, that's normal. Step away, come back tomorrow, and you'll often see the path you missed. This is not a tutorial you breeze through; it's a skill you build over time.

## Tech

- **Engine**: Rust, compiled to WebAssembly via `wasm-pack`. The entire logical kernel — formulas, contexts, judgements, inference rules, an s-expression parser — lives in ~3,000 lines of Rust.
- **Frontend**: Vue 3 + TypeScript + Vite. Single-page app with no router — just a discriminated union and page transitions.
- **Rendering**: KaTeX for math, JetBrains Mono for everything else.
- **i18n**: Full English and Chinese support, including bilingual glossaries for every technical term.

## Credits

Built by [makabaka1880](https://blog.makabaka1880.xyz) for WAIC 2026. Special thanks to Gerhard Gentzen, who invented natural deduction in 1935, and to every player who sat through "I am, therefore I am."

Plato is open source under the MIT license. [Contribute on GitHub.](https://github.com/makabaka1880/plato)
