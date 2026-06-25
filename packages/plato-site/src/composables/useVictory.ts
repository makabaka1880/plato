import { ref } from 'vue'
import confetti from 'canvas-confetti'
import type { Tactic } from '@/types'
import { useTacticsStore } from '@/stores/tactics'

export function useVictory() {
  const solved = ref(false)

  function fire(unlocks: Tactic[]) {
    solved.value = true
    const store = useTacticsStore()
    for (const t of unlocks) {
      store.add(t.name)
    }
    confetti({ particleCount: 120, spread: 100, origin: { y: 0.5 } })
    setTimeout(() => confetti({ particleCount: 80, spread: 80, origin: { y: 0.4, x: 0.3 } }), 200)
    setTimeout(() => confetti({ particleCount: 80, spread: 80, origin: { y: 0.4, x: 0.7 } }), 400)
  }

  return { solved, fire }
}
