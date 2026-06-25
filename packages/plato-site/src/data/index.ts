import type { Problem, Tactic } from '@/types'

/** Omit `id` from the JSON schema — it's assigned here. */
type ProblemData = Omit<Problem, 'id'>

export interface GlossaryEntry {
  id: string
  term: string
  key: string
  definition: string
  intuitive: string
}

/** All locales' problems, keyed by path like `./en/problems/01-identity.json` */
const problemModules = import.meta.glob<{ default: ProblemData }>(
  './*/problems/*.json',
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

/** Load problems for a locale. Falls back to `en`. */
export function loadProblems(locale = 'en'): Problem[] {
  const prefix = `./${locale}/problems/`
  const paths = Object.keys(problemModules)
    .filter(p => p.startsWith(prefix))
    .sort()
  const source = paths.length > 0 ? paths : Object.keys(problemModules).filter(p => p.startsWith('./en/problems/')).sort()
  return source.map((path, index) => ({
    id: index + 1,
    ...problemModules[path]!.default,
  }))
}

/** Load glossary for a locale. Falls back to `en`. */
export function loadGlossary(locale = 'en'): GlossaryEntry[] {
  const key = `./${locale}/glossary.json`
  if (glossaryModules[key]) return glossaryModules[key]!.default
  return glossaryModules['./en/glossary.json']!.default
}

/** Convenience: eagerly load `en` problems. */
export const problems = loadProblems('en')
/** Load NLG templates for a locale. Falls back to `en`. */
export function loadNlg(locale = 'en'): Record<string, string> {
  const key = `./${locale}/nlg.ts`
  if (nlgModules[key]) return nlgModules[key]!.default
  return nlgModules['./en/nlg.ts']!.default
}

/** Convenience: eagerly load `en` glossary. */
export const glossary = loadGlossary('en')
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

/** Convenience: eagerly load `en` NLG. */
export const nlg = loadNlg('en')

