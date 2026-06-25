import type { Problem, Tactic, Section, SectionMeta, DiscoveryData } from '@/types'

/** Omit `id` from the JSON schema — it's assigned here. */
type ProblemData = Omit<Problem, 'id'>

export interface GlossaryEntry {
    id: string
    term: string
    key: string
    definition: string
    intuitive: string
}

// ── Glob imports ──────────────────────────────────────────────────────

/** Section metadata, keyed like `./en/sections/propositional/section.json` */
const sectionMetaModules = import.meta.glob<{ default: SectionMeta }>(
    './*/sections/*/section.json',
    { eager: true },
)

/** Section-scoped problems, keyed like `./en/sections/propositional/problems/01-identity.json` */
const sectionProblemModules = import.meta.glob<{ default: ProblemData }>(
    './*/sections/*/problems/*.json',
    { eager: true },
)

/** Discovery dialogues */
const discoveryModules = import.meta.glob<{ default: DiscoveryData }>(
    './*/sections/*/discovery.json',
    { eager: true },
)

/** All locales' glossaries */
const glossaryModules = import.meta.glob<{ default: GlossaryEntry[] }>(
    './*/glossary.json',
    { eager: true },
)

/** All locales' tactics registries */
const tacticsModules = import.meta.glob<{ default: Tactic[] }>(
    './*/tactics.json',
    { eager: true },
)

/** All locales' NLG templates */
const nlgModules = import.meta.glob<{ default: Record<string, string> }>(
    './*/nlg.ts',
    { eager: true },
)

// ── Section-aware loading ─────────────────────────────────────────────

/** Extract section ID from a path like `./en/sections/propositional/section.json` */
function extractSectionId(path: string): string {
    // path: "./en/sections/{id}/section.json"
    const parts = path.split('/')
    return parts[parts.length - 2]!
}

/** Load all sections for a locale. Falls back to `en`. */
export function loadSections(locale = 'en'): Section[] {
    const prefix = `./${locale}/sections/`
    const fallbackPrefix = `./en/sections/`

    // Discover section IDs from section.json files
    let metaPaths = Object.keys(sectionMetaModules).filter(p => p.startsWith(prefix))
    const useFallback = metaPaths.length === 0
    if (useFallback) {
        metaPaths = Object.keys(sectionMetaModules).filter(p => p.startsWith(fallbackPrefix))
    }

    // Build sections list
    const sections: Section[] = metaPaths.map(metaPath => {
        const id = extractSectionId(metaPath)
        const meta = sectionMetaModules[metaPath]!.default

        // Load problems for this section
        const problemPrefix = useFallback
            ? `${fallbackPrefix}${id}/problems/`
            : `${prefix}${id}/problems/`
        const problemPaths = Object.keys(sectionProblemModules)
            .filter(p => p.startsWith(problemPrefix))
            .sort()

        const problems: Problem[] = problemPaths.map((path, index) => ({
            id: index + 1, // 1-based within section
            ...sectionProblemModules[path]!.default,
        }))

        // Load discovery
        const discoveryPath = useFallback
            ? `${fallbackPrefix}${id}/discovery.json`
            : `${prefix}${id}/discovery.json`
        const discovery: DiscoveryData = discoveryModules[discoveryPath]?.default
            ?? discoveryModules[`${fallbackPrefix}${id}/discovery.json`]?.default
            ?? { title: id, lines: [] }

        return { id, meta, problems, discovery }
    })

    // Sort by order
    sections.sort((a, b) => a.meta.order - b.meta.order)
    return sections
}

/** Look up a single section by ID. */
export function getSection(id: string, locale = 'en'): Section | undefined {
    return loadSections(locale).find(s => s.id === id)
}

/** Look up the next section after the given one (by order). */
export function getNextSection(currentId: string, locale = 'en'): Section | undefined {
    const sections = loadSections(locale)
    const idx = sections.findIndex(s => s.id === currentId)
    if (idx < 0 || idx >= sections.length - 1) return undefined
    return sections[idx + 1]
}

/**
 * Map a legacy global problem index (0-based) to section + within-section index.
 * This handles the original 57-problem ordering before sections were introduced.
 */
export function resolveGlobalIndex(
    globalIdx: number,
    locale = 'en',
): { sectionId: string; sectionIdx: number } | null {
    const sections = loadSections(locale)
    let offset = 0
    for (const section of sections) {
        if (globalIdx < offset + section.problems.length) {
            return { sectionId: section.id, sectionIdx: globalIdx - offset }
        }
        offset += section.problems.length
    }
    return null
}

// ── Legacy flat access (for backward compat) ──────────────────────────

/** Get all problems across all sections, with section metadata attached. */
export function loadAllProblems(locale = 'en'): (Problem & { sectionId: string; sectionIdx: number })[] {
    return loadSections(locale).flatMap(section =>
        section.problems.map(p => ({
            ...p,
            sectionId: section.id,
            sectionIdx: p.id - 1, // 0-based within section
        }))
    )
}

/** @deprecated Use loadSections() instead. */
export function loadProblems(locale = 'en'): Problem[] {
    return loadAllProblems(locale)
}

// ── Glossary ──────────────────────────────────────────────────────────

/** Load glossary for a locale. Falls back to `en`. */
export function loadGlossary(locale = 'en'): GlossaryEntry[] {
    const key = `./${locale}/glossary.json`
    if (glossaryModules[key]) return glossaryModules[key]!.default
    return glossaryModules['./en/glossary.json']!.default
}

/** Convenience: eagerly load `en` glossary. */
export const glossary = loadGlossary('en')

// ── Tactics ───────────────────────────────────────────────────────────

/** Load tactics registry for a locale. Falls back to `en`. */
export function loadTactics(locale = 'en'): Tactic[] {
    const key = `./${locale}/tactics.json`
    if (tacticsModules[key]) return tacticsModules[key]!.default
    return tacticsModules['./en/tactics.json']!.default
}

/** Look up a single tactic by name for the given locale. */
export function lookupTactic(name: string, locale = 'en'): Tactic | null {
    const all = loadTactics(locale)
    return all.find(t => t.name === name) ?? null
}

// ── NLG ───────────────────────────────────────────────────────────────

/** Load NLG templates for a locale. Falls back to `en`. */
export function loadNlg(locale = 'en'): Record<string, string> {
    const key = `./${locale}/nlg.ts`
    if (nlgModules[key]) return nlgModules[key]!.default
    return nlgModules['./en/nlg.ts']!.default
}

/** Convenience: eagerly load `en` NLG. */
export const nlg = loadNlg('en')

/** Convenience: eagerly load `en` sections. */
export const sections = loadSections('en')
