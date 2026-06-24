import { ref, onMounted, computed, type Ref, type ComputedRef } from 'vue'

interface Entry {
  cmd: string
  text: string
  error: boolean
  step: number | null
}

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

export function useProofSession(goal?: Ref<string | undefined>) {
  const ready = ref(false)
  const input = ref('')
  const entries = ref<Entry[]>([])
  let session: any
  let stepCount = 0

  function stepLatex(n: number): string {
    return session.step_latex(n) ?? ''
  }

  function run(expr: string): { result: string; step: number | null } {
    try {
      const result: string = session.execute(expr)
      const error = result.startsWith('Error:')
      const step = error ? null : ++stepCount
      entries.value.push({ cmd: expr, text: result, error, step })
      return { result, step }
    } catch (err: any) {
      entries.value.push({
        cmd: expr,
        text: 'Error: ' + err.message,
        error: true,
        step: null,
      })
      return { result: 'Error: ' + err.message, step: null }
    }
  }

  function reset() {
    entries.value = []
    stepCount = 0
    session.free()
    session = new SessionClass()
  }

  function insertTactic(tactic: string) {
    input.value = tactic
  }

  function isGoalResolved(g: string): boolean {
    return session.isGoalResolved(g)
  }

  const SessionClassRef = ref<any>(null)
  onMounted(async () => {
    await ensureWasm()
    session = new (SessionClass as any)()
    SessionClassRef.value = SessionClass
    ready.value = true
  })

  return {
    ready, input, entries,
    stepLatex, run, reset, insertTactic, isGoalResolved,
    SessionClass: SessionClassRef as Ref<any>,
  }
}
