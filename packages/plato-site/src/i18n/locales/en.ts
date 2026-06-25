export default {
    common: {
        ok: 'OK',
        close: '×',
        dismiss: 'dismiss',
        done: 'Done',
        loading: 'Loading WASM…',
        submit: 'submit',
        error: 'Error',
    },

    home: {
        title: '? ⊢ Plato',
        continue: 'Continue',
        startFresh: '→ Start Fresh',
        customProblem: '→ Go custom',
        contribute: 'Contribute',
        about: 'About',
        story: 'Behind the Scenes',
    },

    custom: {
        title: 'Custom Problem',
        desc: 'Load a problem or a problem set from a URL or a JSON file.',
        urlPlaceholder: 'https://example.com/problem.json',
        loadUrl: 'Load from URL',
        loadFile: 'Load from File',
        back: '← Back',
        invalidJson: 'Invalid JSON. Make sure it matches the problem format.',
        fetching: 'Fetching…',
        agreed: 'Agree',
        singleLoaded: '1 problem loaded.',
        setLoaded: '{n} problems loaded.',
        viewSpec: 'Problem format spec ↗',
    },

    problem: {
        notFound: 'Problem not found.',
        logo: 'Plato',
        preferences: 'Preferences',
        makeMeBelieve: 'MAKE ME BELIEVE',
        premise: 'PREMISE',
        goal: 'GOAL',
        agree: 'Agree',
        victory: 'I Believe You.',
        nextProblem: 'Next Problem →',
        backHome: 'Back Home',
        prev: '← Prev',
        next: 'Next →',
    },

    repl: {
        cmdPrefix: '> ',
        placeholder: '...',
        helpHint: 'Type {cmd} for commands and typing tips',
        tacticExpected: 'Error: expected {tactic}',
        hideHint: 'hide hint {n}',
        showHint: 'show hint {n}',
    },

    help: {
        tabs: {
            commands: 'Commands',
            notations: 'Notations',
            glossary: 'Glossary',
        },
        unlocked: '{n} Unlocked',
        footer: 'Press {key} or click outside to close',

        groups: {
            basics: 'Basics',
            implication: 'Implication (→)',
            conjunction: 'Conjunction (∧)',
            disjunction: 'Disjunction (∨)',
            negation: 'Negation (¬)',
            quantifiers: 'Quantifiers (∀, ∃)',
        },

        commands: {
            assume: 'Assume a formula, adds {F} ⊢ F',
            fix: 'Introduce a term variable {x} ⊢ x (like assume, but for terms)',
            subst: 'Substitute atoms with formulas uniformly in step N',
            show: 'Re-print step N',
            parse: 'Parse a formula — prints its structure',
            '->-intro': 'Discharge formula F from step N to form an implication',
            '->-elim': 'Modus ponens — from p → q and p, derive q',
            'and-intro': 'Combine steps N and M into a conjunction',
            'and-elim-l': 'Extract the left half of a conjunction',
            'and-elim-r': 'Extract the right half of a conjunction',
            'or-intro-l': 'From step N (proves p), form p ∨ F',
            'or-intro-r': 'From step N (proves q), form F ∨ q',
            'or-elim': 'Proof by cases — from p ∨ q and two subproofs reaching r, conclude r',
            'not-intro': 'Reductio ad absurdum — from assuming F you reach a contradiction, so ¬F',
            'not-elim': 'From ¬p derive p → ⊥',
            dne: 'Double negation elimination — from ¬¬p derive p',
            'ex-falso': 'From ⊥ derive anything (principle of explosion)',
            'forall-intro': 'Universal generalisation — discharge x from N (x not free in other assumptions)',
            'forall-elim': 'Universal instantiation — from ∀x.φ, get φ[t/x]',
            'exists-intro': 'Existential generalisation — from φ(t), form ∃x.φ (generalise term t to variable x)',
            'exists-elim': 'Witness elimination — N proves ∃x.φ, M proves ψ under witness x, x not free in conclusion',
        },

        notations: {
            typingSymbols: 'Typing Symbols',
            formulaSyntax: 'Formula Syntax',
            variablesTerms: 'Variables &amp; Terms',
            syntaxIntro: 'Formulas use <b>s-expression</b> (prefix) notation. Each connective wraps its sub-formulas in parentheses.',
            negation: 'Negation',
            implication: 'Implication',
            conjunction: 'Conjunction',
            disjunction: 'Disjunction',
            universal: 'Universal',
            existential: 'Existential',
            varDesc: '<b>Uppercase</b> letters (<code>A</code>, <code>P</code>, <code>Q</code>) are propositional variables or predicates. <b>Lowercase</b> letters (<code>x</code>, <code>y</code>, <code>t</code>) are term variables.',
            predApp: 'Predicate application: <code>(App P x)</code> — the predicate <code>P</code> applied to term <code>x</code>. Nested: <code>(App (App R x) y)</code> for a binary relation.',
        },
    },

    tactics: {
        title: 'Tactics Collected',
        toggle: 'tactics',
        empty: 'Solve problems to collect tactics',
    },

    preferences: {
        title: 'Preferences',
        proofOutput: 'Proof Output',
        language: 'Language',
        tex: 'TeX',
        text: 'Text',
    },

    roadmap: {
        title: 'Your Journey',
        empty: 'Solve your first problem to begin the journey.',
        node: 'Problem {n}',
        proof: 'Proof',
    },

    footer: {
        rights: 'All Rights Reserved',
        author: 'Makabaka1880',
    },

    notFound: {
        title: '404',
        subtitle: 'This page does not exist.',
        back: 'Take me home',
    },

    locked: {
        title: 'Locked',
        subtitle: 'Problem {n} is not yet unlocked.',
        hint: 'Solve the previous problems to unlock it.',
        back: 'Back',
    },
}
