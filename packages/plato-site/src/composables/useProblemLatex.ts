import { ref, onMounted, type Ref } from 'vue'
import type { Problem } from '@/types'

let SessionClass: any
let loaded = false

async function ensureWasm() {
  if (!loaded) {
    const mod = await import('plato-lib')
    await mod.default()
    SessionClass = mod.Session
    loaded = true
  }
}

export function useProblemLatex(problem: Ref<Problem | undefined | null>) {
  const goalLatex = ref(problem.value ? '...' : '')
  const premiseLatex = ref('')

  function update() {
    if (SessionClass && problem.value) {
      goalLatex.value = SessionClass.formulaLatex(problem.value.goal)
      if (problem.value.premise) {
        premiseLatex.value = problem.value.premise
          .split(/\s+/)
          .filter(Boolean)
          .map((a: string) => SessionClass.formulaLatex(a))
          .join(', ')
      } else {
        premiseLatex.value = ''
      }
    }
  }

  onMounted(async () => {
    await ensureWasm()
    update()
  })

  return { goalLatex, premiseLatex, updateLatex: update }
}
