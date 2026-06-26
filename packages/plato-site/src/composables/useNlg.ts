import { loadNlg } from '@/data'

/** Keys that hold LaTeX formula values — wrapped in $...$ for KaTeX rendering. */
const FORMULA_KEYS = new Set(['F', 'conclusion'])

export interface StepMeta {
  cmdName: string
  params: Record<string, string>
}

/** Render a single proof step's NLG text from its structured metadata. */
export function renderNlg(cmdName: string, params: Record<string, string>, nlg: Record<string, string>): string {
  const tpl = nlg[cmdName]
  if (!tpl) return ''
  return tpl.replace(/\{(\w+)\}/g, (_, key: string) => {
    const val = params[key] ?? `{${key}}`
    if (FORMULA_KEYS.has(key)) return '$' + val + '$'
    return val
  })
}

/** Render a whole proof from structured steps into NLG lines. */
export function renderProofLines(steps: StepMeta[], locale = 'en'): string[] {
  const nlg = loadNlg(locale)
  return steps.map((s, i) => `${i + 1}. ${renderNlg(s.cmdName, s.params, nlg)}`)
}
