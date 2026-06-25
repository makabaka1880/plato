import { computed, ref, watch, type Ref } from 'vue'

export interface LiveMeta {
  cmdName: string
  params: Record<string, string>
  /** LaTeX previews for step parameters, e.g. { n: "1.\\; \\{A\\} \\vdash A" } */
  stepPreviews: Record<string, string>
}

export function useLiveParse(input: Ref<string>, session: Ref<any>, stepLatex: (n: number) => string) {
  const liveMeta = ref<LiveMeta | null>(null)
  const parseError = ref<string | null>(null)
  const pending = ref(false)

  watch(input, (val) => {
    const trimmed = val.trim()
    const s = session.value
    if (!s || !trimmed) {
      liveMeta.value = null
      parseError.value = null
      return
    }
    pending.value = true
    try {
      const raw = s.parse_meta(trimmed)
      if (raw) {
        const params: Record<string, string> = JSON.parse(raw.params_json)
        const stepPreviews: Record<string, string> = {}
        // Build LaTeX previews for step-number params
        for (const [k, v] of Object.entries(params)) {
          const num = parseInt(v, 10)
          if (!isNaN(num) && num > 0) {
            const latex = stepLatex(num)
            if (latex) stepPreviews[k] = latex
          }
        }
        liveMeta.value = { cmdName: raw.cmd_name, params, stepPreviews }
        parseError.value = null
        return
      }
      // Not a command — try formula parse
      const latex = s.formulaLatex(trimmed)
      if (latex && !latex.startsWith('Error') && !latex.includes('expected')) {
        liveMeta.value = { cmdName: 'formula', params: { F: trimmed }, stepPreviews: {} }
        parseError.value = null
      } else {
        liveMeta.value = null
        parseError.value = latex || 'Cannot parse'
      }
    } catch {
      liveMeta.value = null
      parseError.value = 'Cannot parse'
    } finally {
      pending.value = false
    }
  }, { immediate: true })

  return { liveMeta, parseError, pending }
}
