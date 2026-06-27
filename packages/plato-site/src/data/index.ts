import type { Problem, Tactic, Section, SectionMeta, DiscoveryData, Level } from '@/types'

/** Omit `id` from the JSON schema — it's assigned here. */
type ProblemData = Omit<Problem, 'id'>

/** Raw level file on disk — discriminated by `type`. */
type LevelFileData =
    | ({ type: 'discovery' } & DiscoveryData)
    | ({ type: 'problem' } & ProblemData)

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

/** All level files (discoveries & problems), keyed like `./en/sections/propositional/levels/000-discovery.json` */
const levelModules = import.meta.glob<{ default: LevelFileData }>(
    './*/sections/*/levels/*.json',
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

/** Extract section ID from a path like `./en/sections/propositional/levels/000-discovery.json` */
function extractSectionId(path: string): string {
    // path: "./en/sections/{id}/section.json" or "./en/sections/{id}/levels/..."
    const parts = path.split('/')
    // section.json: parts[parts.length-2] = section id
    // levels/foo.json: parts[parts.length-3] = section id
    if (parts[parts.length - 1] === 'section.json') return parts[parts.length - 2]!
    return parts[parts.length - 3]!
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

        // Load all level files for this section (sorted by filename = level order)
        const levelPrefix = useFallback
            ? `${fallbackPrefix}${id}/levels/`
            : `${prefix}${id}/levels/`
        const levelPaths = Object.keys(levelModules)
            .filter(p => p.startsWith(levelPrefix))
            .sort()

        let problemId = 0
        const levels: Level[] = levelPaths.map(path => {
            const raw = levelModules[path]!.default
            if (raw.type === 'discovery') {
                const { type, ...data } = raw
                return { type: 'discovery' as const, data }
            }
            // problem
            problemId++
            const { type, ...data } = raw
            return {
                type: 'problem' as const,
                data: { id: problemId, ...data },
            }
        })

        return { id, meta, levels }
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

// ── Level index <-> problem index mapping ─────────────────────────────

/** Count the number of problem-type levels in a section. */
export function sectionProblemCount(section: Section): number {
    return section.levels.filter(l => l.type === 'problem').length
}

/**
 * Convert a 0-based problem index to the corresponding level index.
 * Returns null if the problem index is out of range.
 */
export function problemIdxToLevelIdx(section: Section, problemIdx: number): number | null {
    let count = 0
    for (let i = 0; i < section.levels.length; i++) {
        if (section.levels[i]!.type === 'problem') {
            if (count === problemIdx) return i
            count++
        }
    }
    return null
}

/**
 * Convert a level index to the corresponding 0-based problem index.
 * Returns null if the level is not a problem (e.g. it's a discovery).
 */
export function levelIdxToProblemIdx(section: Section, levelIdx: number): number | null {
    if (levelIdx < 0 || levelIdx >= section.levels.length) return null
    const level = section.levels[levelIdx]
    if (level?.type !== 'problem') return null
    let count = 0
    for (let i = 0; i < levelIdx; i++) {
        if (section.levels[i]!.type === 'problem') count++
    }
    return count
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
