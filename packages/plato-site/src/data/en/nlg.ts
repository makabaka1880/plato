export default {
    assume: 'If we accept {F}, then we have {conclusion}.',
    fix: 'Take any {x}, and we get {conclusion}.',
    subst: 'What line {n} proved doesn\'t depend on the names of things — replace them and we still get {conclusion}.',
    show: 'Look back at step {n}.',
    parse: 'Here\'s the formula {F}.',

    '->-intro': 'From assuming {F} we reached the conclusion at line {n}, so we can now say {conclusion}.',
    '->-elim': 'Line {n} says that if its condition holds, the consequence follows. Line {m} gives us that condition. So {conclusion}.',

    'and-intro': 'Lines {n} and {m} each give us a piece; together they make {conclusion}.',
    'and-elim-l': 'Line {n} asserts two things; the first is enough for {conclusion}.',
    'and-elim-r': 'Line {n} asserts two things; the second is enough for {conclusion}.',

    'or-intro-l': 'Line {n} already holds, so it\'s still true even if we tack on {F} — giving us {conclusion}.',
    'or-intro-r': 'Line {n} already holds, so we can drop a {F} in front — giving us {conclusion}.',
    'or-elim': 'We\'re at a fork. Line {n} presents the two possibilities; lines {m} and {k} each lead to the same place, so either way we get {conclusion}.',

    'not-intro': 'Lines {n} and {m} contradict each other. The culprit must be {F}, so we reject it, and get {conclusion}.',
    'not-elim': 'Line {n} warns us that a certain path leads to trouble. Following it through, we arrive at {conclusion}.',
    dne: 'If denying something twice still leaves it standing — as line {n} shows — then it must be true: {conclusion}.',
    'ex-falso': 'Line {n} gives us an impossibility. From that, anything follows — so we may as well have {conclusion}.',

    'forall-intro': 'Line {n} talks about {x} without any special assumptions, so it holds for every {x}: {conclusion}.',
    'forall-elim': 'If it\'s true for everything — and line {n} says it is — then it\'s certainly true for {t}: {conclusion}.',
    'exists-intro': 'Line {n} gives us a concrete example in {t}, so we know something out there answers to {x}: {conclusion}.',
    'exists-elim': 'Line {n} tells us that something answering to {x} exists. Using the witness from line {m}, we conclude {conclusion}.',

    'box-intro': 'Line {n} was proved without leaning on any worldly assumptions, so it holds necessarily: {conclusion}.',
    'box-elim': 'Line {n} says necessity respects implication, and line {m} supplies the antecedent under necessity — so {conclusion}.',
        'top-intro': 'Truth is always provable, no questions asked. So: {conclusion}.',
    'diamond-def-rev': 'Folding the double negation in line {n} back through the definition gives us {conclusion}.',
    'diamond-def': 'Unfolding the possibility in line {n} through its definition gives us {conclusion}.',
}
