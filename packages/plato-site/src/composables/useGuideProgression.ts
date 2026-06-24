import { ref, computed, type Ref, type ComputedRef } from 'vue'
import type { Hint } from '@/types'

export function useGuideProgression(guides: () => Hint[]) {
  const guideIdx = ref(0)
  const visibleGuide: ComputedRef<Hint | null> = computed(() => {
    const gs = guides()
    return guideIdx.value < gs.length ? gs[guideIdx.value] : null
  })

  function advanceGuide() {
    if (guideIdx.value < guides().length) guideIdx.value++
  }

  function reset() {
    guideIdx.value = 0
  }

  return { guideIdx, visibleGuide, advanceGuide, reset }
}
