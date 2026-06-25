import { ref, watch, nextTick, type Ref, type ComputedRef } from 'vue'

export interface Suggestion {
  label: string
  insert: string
  description: string
  kind: 'command' | 'formula' | 'step'
  /** KaTeX preview for step suggestions */
  latex?: string
}

const STATIC: Suggestion[] = [
  // ── commands ──
  { label: 'assume', insert: 'assume ', description: 'Assume a formula', kind: 'command' },
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
  { label: 'forall-intro', insert: 'forall-intro ', description: 'Universal generalisation', kind: 'command' },
  { label: 'forall-elim', insert: 'forall-elim ', description: 'Universal instantiation', kind: 'command' },
  { label: 'exists-intro', insert: 'exists-intro ', description: 'Existential generalisation', kind: 'command' },
  { label: 'exists-elim', insert: 'exists-elim ', description: 'Existential witness elimination', kind: 'command' },
  // ── formula connectives ──
  { label: '->', insert: '-> ', description: 'Implication', kind: 'formula' },
  { label: 'and', insert: 'and ', description: 'Conjunction', kind: 'formula' },
  { label: 'or', insert: 'or ', description: 'Disjunction', kind: 'formula' },
  { label: 'not', insert: 'not ', description: 'Negation', kind: 'formula' },
  { label: 'forall', insert: 'forall ', description: 'Universal quantifier', kind: 'formula' },
  { label: 'exists', insert: 'exists ', description: 'Existential quantifier', kind: 'formula' },
  { label: 'App', insert: 'App ', description: 'Predicate application', kind: 'formula' },
  { label: '_|_', insert: '_|_', description: 'Contradiction / falsity', kind: 'formula' },
]

/**
 * Find the innermost unclosed-paren fragment around the cursor.
 * Example: for `(assume (an )` with cursor at `|`, returns `(an` (the inner paren).
 * Returns `{ start, end, fragment }` where fragment is the text after `(` before cursor,
 * or null if no matching paren found.
 */
function parseParenFragment(value: string, cursorPos: number): { start: number; end: number; fragment: string; replaceStart: number } | null {
  let depth = 0
  for (let i = cursorPos - 1; i >= 0; i--) {
    const ch = value[i]
    if (ch === ')') {
      depth++
    } else if (ch === '(') {
      if (depth > 0) {
        depth--
      } else {
        const afterParen = value.slice(i + 1)
        const closeIdx = afterParen.indexOf(')')
        const text = closeIdx === -1 ? afterParen.slice(0, cursorPos - i - 1) : afterParen.slice(0, closeIdx)
        // Extract the last space-delimited token
        const lastSpace = text.lastIndexOf(' ')
        const fragment = lastSpace === -1 ? text : text.slice(lastSpace + 1)
        if (!fragment) return null
        // replaceStart: where the fragment starts in the original string.
        // When there's a space, we replace just the last token (not from `(`).
        const replaceStart = lastSpace === -1 ? i : i + 1 + lastSpace + 1
        return { start: i, end: cursorPos, fragment, replaceStart }
      }
    }
  }
  return null
}

export function useAutocomplete(
  input: Ref<string>,
  inpEl: Ref<HTMLInputElement | null>,
  stepPreviews: ComputedRef<Record<string, string>>,
) {
  const suggestions = ref<Suggestion[]>([])
  const activeIndex = ref(0)
  const visible = ref(false)
  let matchStart = 0
  let matchEnd = 0

  function computeSuggestions(val: string) {
    const el = inpEl.value
    const cursor = el?.selectionStart ?? val.length
    const info = parseParenFragment(val, cursor)
    if (!info) return null
    matchStart = info.start
    matchEnd = info.end
    const rawFragment = info.fragment.trim()
    const fragment = rawFragment.toLowerCase()

    if (fragment === '') {
      return { suggestions: STATIC, activeIdx: 0 }
    }

    let filtered = STATIC.filter(s => s.label.toLowerCase().startsWith(fragment))
    let activeIdx = 0

    if (/^[A-Za-z]$/.test(rawFragment) && fragment.length === 1 && !filtered.some(s => s.label.toLowerCase() === fragment)) {
      filtered = [{ label: rawFragment, insert: rawFragment + ' ', description: 'Insert Atom', kind: 'formula' }, ...filtered]
    }

    if (/^\d+$/.test(fragment)) {
      const previews = stepPreviews.value
      const stepSugs: Suggestion[] = []
      for (const [n, latex] of Object.entries(previews)) {
        if (n.startsWith(fragment)) {
          stepSugs.push({ label: n, insert: n + ' ', description: 'Step ' + n, kind: 'step', latex })
        }
      }
      filtered = [...stepSugs, ...filtered]
    }

    return { suggestions: filtered, activeIdx }
  }

  let lastLen = 0
  function refresh() {
    const val = input.value
    const res = computeSuggestions(val)
    if (!res) {
      visible.value = false
      suggestions.value = []
      return
    }
    suggestions.value = res.suggestions
    activeIndex.value = res.activeIdx
    visible.value = res.suggestions.length > 0
  }

  watch(input, (val) => {
    const wentUp = val.length >= lastLen
    lastLen = val.length
    if (!visible.value && !wentUp) return
    refresh()
  }, { immediate: true })

  function onCursorCheck() {
    refresh()
  }

  function accept() {
    const s = suggestions.value[activeIndex.value]
    if (!s) return
    const el = inpEl.value
    const val = input.value
    const cursorPos = el?.selectionStart ?? val.length

    const info = parseParenFragment(val, cursorPos)
    if (!info) return

    // replaceStart === info.start means replace from `(` as before (the command name itself).
    // Otherwise replace only the last token (e.g. a step number argument).
    const prependParen = info.replaceStart === info.start
    const before = val.slice(0, info.replaceStart)
    const after = val.slice(cursorPos)
    const insertion = (prependParen ? '(' : '') + s.insert
    input.value = before + insertion + after

    const cursor = before.length + insertion.length
    if (el) {
      nextTick(() => el.setSelectionRange(cursor, cursor))
    }
    visible.value = false
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

  function dismiss() {
    visible.value = false
  }

  return { suggestions, activeIndex, visible, accept, onKeydown, dismiss, refresh, onCursorCheck }
}
