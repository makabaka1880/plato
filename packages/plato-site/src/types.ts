export interface Problem {
    id: number
    description: string
    premise: string[]
    goal: string
    guides: Hint[]
    hints: Hint[]
    unlocks: string[]
    /** Logic mode override. If absent, the section default governs. */
    logicMode?: 'fol' | 'pl'
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
    /** Concrete syntax, e.g. "(->-intro F N)" */
    syntax: string
    /** A short worked example, e.g. "From $A$ we proved $B$, so we get $A \\to B$." */
    example: string
}

// ── Section types ──────────────────────────────────────────────────────

/** Metadata about a section, loaded from section.json */
export interface SectionMeta {
    nameI18nKey: string
    logicMode: 'pl' | 'fol'
    allowedTactics: string[]
    order: number
}

/** A single line in a discovery dialogue */
export interface DiscoveryLine {
    speaker: string,
    text: string,
    sid: string,
    image: string | undefined,
}

/** Structure of a discovery.json file */
export interface DiscoveryData {
    title: string
    lines: DiscoveryLine[]
}

/** A single level within a section — either a discovery dialogue or a proof problem. */
export type Level =
    | { type: 'discovery'; data: DiscoveryData }
    | { type: 'problem'; data: Problem }

/** Fully assembled section at runtime */
export interface Section {
    id: string
    meta: SectionMeta
    levels: Level[]
}
