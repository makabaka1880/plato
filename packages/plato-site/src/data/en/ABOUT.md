# About Plato

> *He was not only the best of men, but also the wisest.*

Plato is a proof assistant disguised as a puzzle game. It is named after the philosopher who believed that true knowledge comes from reasoning — not from being told.

I built this because I wanted a place where logic feels like play. Most proof assistants are intimidating: they speak in arcane syntax, demand hundreds of pages of reading, and assume you already have a PhD. Plato starts from the other end. You type `(assume A)`. You watch a new line appear. Something clicks. You type a little more. Suddenly you're thinking in implications.

The idea isn't to teach you *about* logic — it's to teach you to *do* logic. And the best way to learn is by getting your hands dirty, making mistakes, and slowly developing an instinct for what a proof feels like.

## How it works

You are shown a statement to prove. You type commands in an s-expression REPL — `(assume A)`, `(->-elim 2 1)`, `(and-intro 1 3)` — and the engine checks each step against the rules of natural deduction. Guide cards walk you through your first proofs. Hints appear when you're lost. Every solved problem unlocks new tactics, building your toolkit one rule at a time.

There are **61 problems** and **24 tactics** distributed across three sections — Propositional Logic (38 problems), First-Order Logic (15 problems), and Modal Logic / System K (8 problems). 

To keep the momentum going, the game features a dense, interleaved learning pattern. Instead of dropping massive walls of theory on you all at once, short **discovery dialogues** — illustrated conversations between historical logicians (Plato & Aristotle, De Morgan & Frege, Aristotle & the Master) — are woven directly into the progression. A concept is introduced via dialogue, and you immediately reinforce it with relevant exercises before moving to the next insight. Each step forward unlocks new tactics, complete with a LaTeX inference rule and natural-language description.

Plato's engine supports two **axiom sets** — PL and FOL — displayed as a colored chip in the navbar (green for FOL on, orange for FOL off). See below for why this separation matters.

You can also **load your own problems** from a URL or a JSON file — the engine doesn't care where the goal came from.

## Why two axiom sets?

Plato lives at the intersection of three logical systems, and the way they interact — or refuse to interact — is part of what makes the design interesting.

**Propositional logic (PL)** is the simplest of the three. You reason about atomic propositions connected by $\lnot$, $\land$, $\lor$, $\to$, $\top$, and $\bot$. It's decidable: a truth table can settle any propositional statement in finite time. This is the bedrock.

**First-order logic (FOL)** adds quantifiers ($\forall$, $\exists$) and lets you talk about objects and their properties — "all men are mortal," "Socrates is a man." FOL is semi-decidable: if a formula is valid, there's a proof; but if it isn't, no algorithm can always tell you so. Still, for practical natural deduction, FOL is well-behaved and thoroughly understood.

**Modal logic (System K)** adds $\Box$ (necessity) and $\Diamond$ (possibility) on top of propositional logic. System K is decidable — in fact, you can translate it into a fragment of first-order logic (the *standard translation* into possible-world semantics), so it's no more dangerous than FOL.

The trouble starts when you try to combine them. **First-order modal logic (FOML)** — FOL plus $\Box$ and $\Diamond$ — is a different beast entirely. Quantifiers and modal operators don't play nice together. Consider the Barcan formula:

$$
\forall x \Box P(x) \to \Box \forall x P(x)
$$

Should this hold? It depends. If you think objects can pop into or out of existence as you move between possible worlds (varying domains), the answer is no. If you believe the domain of all possible objects is fixed (constant domains), the answer is yes. There is no single canonical semantics — different philosophical commitments about what "exists" means across possible worlds produce irreconcilably different logics. Worse, even the monadic fragment of FOML (only unary predicates) is undecidable — unlike monadic FOL, which is decidable. The interaction between quantifiers and modalities is so rich that it pushes the system beyond what any finite algorithm can tame.

Plato's solution is simple: **don't combine them.** The propositional and modal sections run under PL (you get $\Box$ and $\Diamond$ but not quantifiers). The first-order section runs under FOL (you get $\forall$ and $\exists$ but not modal operators). Each system stays within its own decidable territory, and the axiom set chip in the navbar tells you at a glance which toolkit you're holding.

## Why natural deduction?

Because natural deduction mirrors how humans actually reason.

In most logic classes, you learn truth tables. You fill out rows, you check columns. It works, but it doesn't feel like *thinking*. Natural deduction is different: you assume things, you derive conclusions, you discharge assumptions. Each step corresponds to a real piece of reasoning — "if I knew *p*, then I'd know *q*." You're not computing a truth value; you're building an argument.

## A note on difficulty

Natural deduction is legitimately hard. There is no algorithm for constructing a proof — you have to develop a feel for it. Problems that look simple can be genuinely challenging. If you get stuck, that's normal. Step away, come back tomorrow, and you'll often see the path you missed. This is not a tutorial you breeze through; it's a skill you build over time.

## Tech

- **Engine**: Rust, compiled to WebAssembly via `wasm-pack`. The entire logical kernel — formulas, contexts, judgements, inference rules, an s-expression parser — lives in ~3,000 lines of Rust.
- **Frontend**: Vue 3 + TypeScript + Vite. Single-page app with vue-router (hash history) and page transitions.
- **Rendering**: KaTeX for math, JetBrains Mono for everything else.
- **i18n**: Full English and Chinese support, including bilingual glossaries for every technical term and a Chinese natural-language generation layer for proof step explanations.

## Credits

Built by [makabaka1880](https://blog.makabaka1880.xyz) for WAIC 2026. Special thanks to Gerhard Gentzen, who invented natural deduction in 1935, and to every player who sat through "I am, therefore I am."

Plato is open source under the MIT license. [Contribute on GitHub.](https://github.com/makabaka1880/plato)
