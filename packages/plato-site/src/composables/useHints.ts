import { ref, computed, type ComputedRef } from 'vue'

export function useHints(count: () => number) {
  const hintRevealed = ref(count() > 0 ? 1 : 0)
  const openHints = ref<Set<number>>(new Set())
  const anyHintOpen: ComputedRef<boolean> = computed(() => openHints.value.size > 0)

  function toggle(i: number) {
    const next = new Set(openHints.value)
    if (next.has(i)) {
      next.delete(i)
    } else {
      next.add(i)
      if (i === hintRevealed.value - 1 && hintRevealed.value < count()) {
        setTimeout(() => { hintRevealed.value++ }, 400)
      }
    }
    openHints.value = next
  }

  function dismiss(i: number) {
    const next = new Set(openHints.value)
    next.delete(i)
    openHints.value = next
  }

  function reset() {
    hintRevealed.value = count() > 0 ? 1 : 0
    openHints.value = new Set()
  }

  return { hintRevealed, openHints, anyHintOpen, toggleHint: toggle, dismissHint: dismiss, reset }
}
