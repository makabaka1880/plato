/**
 * Composable that adds parenthesis auto-pairing to an `<input>` element.
 *
 * - Typing `(` inserts `()` and places the cursor between them.
 * - Typing `)` when the cursor is immediately before an existing `)` just
 *   moves the cursor past it (no double-insert).
 * - Backspacing when the cursor sits between an empty `()` pair deletes both.
 * - When text is selected and `(` is typed, the selection is wrapped.
 *
 * Usage:
 * ```ts
 * const { onKeydown } = useBracketPairing(input, inpEl)
 * // <input v-model="input" ref="inpEl" @keydown="onKeydown" />
 * ```
 */

import { type Ref } from 'vue'

const OPEN = '('
const CLOSE = ')'

export function useBracketPairing(
  _input: Ref<string>,
  inpEl: Ref<HTMLInputElement | null>,
) {
  function apply(el: HTMLInputElement, value: string, cursor: number) {
    el.value = value
    // Notify Vue's v-model so the reactive `input` ref stays in sync.
    el.dispatchEvent(new Event('input', { bubbles: true }))
    el.setSelectionRange(cursor, cursor)
  }

  function onKeydown(e: KeyboardEvent) {
    const el = inpEl.value
    if (!el) return

    const start = el.selectionStart ?? 0
    const end = el.selectionEnd ?? 0
    const val = el.value

    // ── open bracket ──────────────────────────────────────────────
    if (e.key === OPEN) {
      e.preventDefault()

      if (start !== end) {
        // wrap selection
        const selected = val.slice(start, end)
        apply(el, val.slice(0, start) + OPEN + selected + CLOSE + val.slice(end), start + 1 + selected.length)
      } else {
        // insert pair, cursor between
        apply(el, val.slice(0, start) + OPEN + CLOSE + val.slice(end), start + 1)
      }
      return
    }

    // ── close bracket ─────────────────────────────────────────────
    if (e.key === CLOSE) {
      // if the next character is already a close-paren, just skip over it
      if (start === end && val.charAt(start) === CLOSE) {
        e.preventDefault()
        // no value change — just move the cursor; still dispatch so vue stays in sync
        el.setSelectionRange(start + 1, start + 1)
      }
      return
    }

    // ── backspace over empty pair ──────────────────────────────────
    if (e.key === 'Backspace') {
      if (start === end && start > 0 && start < val.length) {
        if (val.charAt(start - 1) === OPEN && val.charAt(start) === CLOSE) {
          e.preventDefault()
          apply(el, val.slice(0, start - 1) + val.slice(start + 1), start - 1)
        }
      }
      return
    }
  }

  return { onKeydown }
}
