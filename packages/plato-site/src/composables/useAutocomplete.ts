import { ref, watch, nextTick, type Ref, type ComputedRef } from 'vue'

export interface Suggestion {
  label: string
  insert: string
  description: string
  kind: 'command' | 'formula' | 'step' | 'atom'
  /** KaTeX preview for step suggestions */
  latex?: string
}

const STATIC: Suggestion[] = [
  // ── commands ──
  { label: 'assume', insert: 'assume ', description: 'Assume a formula', kind: 'command' },
  { label: 'extend', insert: 'extend ', description: 'Weakening — extend context with extra assumption', kind: 'command' },
  { label: 'fix', insert: 'fix ', description: 'Introduce a fresh term variable', kind: 'command' },
  { label: 'show', insert: 'show ', description: 'Re-print a step', kind: 'command' },
  { label: 'subst', insert: 'subst ', description: 'Substitute atoms with formulas', kind: 'command' },
  { label: '->-intro', insert: '->-intro ', description: 'Implication introduction', kind: 'command' },
  { label: '->-elim', insert: '->-elim ', description: 'Modus ponens', kind: 'command' },
  { label: 'and-intro', insert: 'and-intro ', description: 'Conjunction introduction', kind: 'command' },
  { label: 'and-elim-l', insert: 'and-elim-l ', description: 'Left conjunction elimination', kind: 'command' },
  { label: 'and-elim-r', insert: 'and-elim-r ', description: 'Right conjunction elimination', kind: 'command' },
  { label: 'or-intro-l', insert: 'or-intro-l ', description: 'Left disjunction introduction', kind: 'command' },
  { label: 'or-intro-r', insert: 'or-intro-r ', description: 'Right disjunction introduction', kind: 'command' },
  { label: 'or-elim', insert: 'or-elim ', description: 'Proof by cases', kind: 'command' },
  { label: 'not-intro', insert: 'not-intro ', description: 'Negation introduction (reductio)', kind: 'command' },
  { label: 'not-elim', insert: 'not-elim ', description: 'Negation elimination', kind: 'command' },
  { label: 'dne', insert: 'dne ', description: 'Double negation elimination', kind: 'command' },
  { label: 'ex-falso', insert: 'ex-falso ', description: 'Ex falso quodlibet', kind: 'command' },
  { label: 'top-intro', insert: 'top-intro ', description: 'Truth introduction (⊤)', kind: 'command' },
  { label: 'forall-intro', insert: 'forall-intro ', description: 'Universal generalisation', kind: 'command' },
  { label: 'forall-elim', insert: 'forall-elim ', description: 'Universal instantiation', kind: 'command' },
  { label: 'exists-intro', insert: 'exists-intro ', description: 'Existential generalisation', kind: 'command' },
  { label: 'exists-elim', insert: 'exists-elim ', description: 'Existential witness elimination', kind: 'command' },
  { label: 'box-intro', insert: 'box-intro ', description: 'Necessity introduction', kind: 'command' },
  { label: 'box-elim', insert: 'box-elim ', description: 'Necessity elimination (K axiom)', kind: 'command' },
  { label: 'diamond-def', insert: 'diamond-def ', description: 'Unfold possibility via definition', kind: 'command' },
  { label: 'diamond-def-rev', insert: 'diamond-def-rev ', description: 'Fold back to possibility', kind: 'command' },
  // ── formula connectives ──
  { label: '->', insert: '-> ', description: 'Implication', kind: 'formula' },
  { label: 'and', insert: 'and ', description: 'Conjunction', kind: 'formula' },
  { label: 'or', insert: 'or ', description: 'Disjunction', kind: 'formula' },
  { label: 'not', insert: 'not ', description: 'Negation', kind: 'formula' },
  { label: 'forall', insert: 'forall ', description: 'Universal quantifier', kind: 'formula' },
  { label: 'exists', insert: 'exists ', description: 'Existential quantifier', kind: 'formula' },
  { label: 'App', insert: 'App ', description: 'Predicate application', kind: 'formula' },
  { label: 'box', insert: 'box ', description: 'Necessity modality (□)', kind: 'formula' },
  { label: 'diamond', insert: 'diamond ', description: 'Possibility modality (◇)', kind: 'formula' },
  { label: '_|_', insert: '_|_', description: 'Contradiction / falsity', kind: 'formula' },
]

/**
 * Extract the token at the cursor: scan backwards until hitting a space or paren.
 * Returns `{ token, start }` where `start` is the index in `value` where the token begins.
 */
function tokenAtCursor(value: string, cursorPos: number): { token: string; start: number } | null {
  let start = cursorPos
  while (start > 0 && value[start - 1] !== ' ' && value[start - 1] !== '(' && value[start - 1] !== ')') {
    start--
  }
  const token = value.slice(start, cursorPos)
  if (token.length === 0) return null
  return { token, start }
}

export function useAutocomplete(
  input: Ref<string>,
  inpEl: Ref<HTMLInputElement | null>,
  stepPreviews: ComputedRef<Record<string, string>>,
  allowedTactics?: Ref<string[] | null>,
) {
  const suggestions = ref<Suggestion[]>([])
  const activeIndex = ref(0)
  const visible = ref(false)
  let tokenStart = 0
  let suppressNext = false

  function refresh() {
    if (suppressNext) { suppressNext = false; return }
    const el = inpEl.value
    const val = input.value
    const cursor = el?.selectionStart ?? val.length

    const tok = tokenAtCursor(val, cursor)
    if (!tok || tok.token.length === 0) {
      if (visible.value) { visible.value = false; suggestions.value = [] }
      return
    }
    tokenStart = tok.start
    const raw = tok.token
    const lower = raw.toLowerCase()

    let filtered = STATIC.filter(s => s.label.toLowerCase().startsWith(lower))

    // Filter by allowed tactics if provided
    if (allowedTactics?.value) {
      const allowed = new Set(allowedTactics.value)
      // Tactic name sets for connectives/formula constructs
      const needsQuantifier = new Set(['forall', 'exists', 'App', 'fix'])
      const needsModal = new Set(['box', 'diamond'])
      filtered = filtered.filter(s => {
        if (s.kind === 'command' || s.kind === 'atom') {
          // For 'fix', check if it's in allowed tactics
          if (s.label === 'fix') return allowed.has('fix')
          // For other commands, check directly
          return allowed.has(s.label)
        }
        if (s.kind === 'formula') {
          // Gate formula connectives by tactic availability
          if (needsQuantifier.has(s.label)) return allowed.has('forall-intro') || allowed.has('exists-intro')
          if (needsModal.has(s.label)) return allowed.has('box-intro') || allowed.has('diamond-def')
          // Propositional connectives are always available if assume is
          return allowed.has('assume')
        }
        // Step suggestions are always shown
        return true
      })
    }

    let activeIdx = 0

    // Single ASCII letter → prepend atom suggestion
    if (/^[A-Za-z]$/.test(raw) && raw.length === 1 && !filtered.some(s => s.label.toLowerCase() === lower)) {
      filtered = [{ label: raw, insert: raw + ' ', description: 'Insert Atom', kind: 'atom' }, ...filtered]
    }

    // Numeric → step suggestions
    if (/^\d+$/.test(raw)) {
      const previews = stepPreviews.value
      const stepSugs: Suggestion[] = []
      for (const [n, latex] of Object.entries(previews)) {
        if (n.startsWith(raw)) {
          stepSugs.push({ label: n, insert: n + ' ', description: 'Step ' + n, kind: 'step', latex })
        }
      }
      filtered = [...stepSugs, ...filtered]
    }

    if (filtered.length === 0) {
      if (visible.value) { visible.value = false; suggestions.value = [] }
      return
    }
    suggestions.value = filtered
    activeIndex.value = activeIdx
    visible.value = true
  }

  watch(input, () => refresh(), { immediate: true })

  function accept() {
    const s = suggestions.value[activeIndex.value]
    if (!s) return
    const el = inpEl.value
    const val = input.value
    const cursor = el?.selectionStart ?? val.length

    const before = val.slice(0, tokenStart)
    const after = val.slice(cursor)
    input.value = before + s.insert + after

    const newCursor = before.length + s.insert.length
    if (el) {
      nextTick(() => el.setSelectionRange(newCursor, newCursor))
    }
    visible.value = false
    suppressNext = true
  }

  function onKeydown(e: KeyboardEvent) {
    if (!visible.value) return
    const len = suggestions.value.length
    if (len === 0) return

    switch (e.key) {
      case 'Tab':
        e.preventDefault()
        accept()
        break
      case 'ArrowDown':
        e.preventDefault()
        activeIndex.value = (activeIndex.value + 1) % len
        break
      case 'ArrowUp':
        e.preventDefault()
        activeIndex.value = (activeIndex.value - 1 + len) % len
        break
      case 'Escape':
        e.preventDefault()
        visible.value = false
        break
      case 'Enter':
        e.preventDefault()
        accept()
        break
    }
  }

  function dismiss() { visible.value = false }
  /** Signal that the cursor has moved via click — dismiss the menu. */
  function onCursorMove() {
    if (visible.value) {
      visible.value = false
      suggestions.value = []
    }
  }

  return { suggestions, activeIndex, visible, accept, onKeydown, dismiss, onCursorMove }
}
