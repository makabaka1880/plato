export interface Problem {
    id: number
    description: string
    premise: string[]
    goal: string
    guides: Hint[]
    hints: Hint[]
    unlocks: Tactic[]
}

export interface Hint {
    text: string
    tactic: string | undefined
}

export interface Tactic {
    /** Display name, e.g. "→-intro" */
    name: string
    /** KaTeX inference rule tree, e.g. "\\frac{\\Gamma, p \\vdash q}{\\Gamma \\vdash p \\to q}" */
    rule: string
    /** What this rule means */
    description: string
}