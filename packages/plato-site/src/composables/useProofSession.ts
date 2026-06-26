import { ref, watch, onMounted, computed, type Ref, type ComputedRef } from 'vue'

interface Entry {
  cmd: string
  text: string
  error: boolean
  step: number | null
  meta: { cmdName: string; params: Record<string, string> } | null
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

export function useProofSession(logicMode?: Ref<'fol' | 'pl'>, goal?: Ref<string | undefined>) {
  const ready = ref(false)
  const input = ref('')
  const entries = ref<Entry[]>([])
  let session: any
  let stepCount = 0

  function stepLatex(n: number): string {
    return session.step_latex(n) ?? ''
  }

  function run(expr: string): { result: string; step: number | null } {
    let meta: Entry['meta'] = null
    try {
      const rawMeta = session.parse_meta(expr)
      if (rawMeta) {
        meta = {
          cmdName: rawMeta.cmd_name,
          params: JSON.parse(rawMeta.params_json),
        }
      }
    } catch { /* meta parse failure is non-fatal */ }

    try {
      const prevLen: number = session.len()
      const result: string = session.execute(expr)
      const error = result.startsWith('Error:')
      const addedStep = session.len() > prevLen
      const step = !error && addedStep ? ++stepCount : null
      if (step !== null && meta) {
        const concl = session.stepFormulaLatex(step)
        if (concl) meta.params.conclusion = concl
      }
      entries.value.push({ cmd: expr, text: result, error, step, meta: error ? null : meta })
      return { result, step }
    } catch (err: any) {
      entries.value.push({
        cmd: expr,
        text: 'Error: ' + err.message,
        error: true,
        step: null,
        meta: null,
      })
      return { result: 'Error: ' + err.message, step: null }
    }
  }

  function reset() {
    entries.value = []
    stepCount = 0
    session.free()
    session = new SessionClass()
    if (logicMode) {
      session.setMode(logicMode.value)
    }
  }

  function insertTactic(tactic: string) {
    input.value = tactic
  }

  function isGoalResolved(g: string): boolean {
    return session.isGoalResolved(g)
  }

  const SessionClassRef = ref<any>(null)
  const sessionRef = ref<any>(null)
  onMounted(async () => {
    await ensureWasm()
    session = new (SessionClass as any)()
    SessionClassRef.value = SessionClass
    sessionRef.value = session
    if (logicMode) {
      session.setMode(logicMode.value)
    }
    ready.value = true
  })

  // Sync logic mode with WASM session on toggle
  if (logicMode) {
    watch(logicMode, (mode) => {
      session?.setMode(mode)
    })
  }

  return {
    ready, input, entries,
    stepLatex, run, reset, insertTactic, isGoalResolved,
    SessionClass: SessionClassRef as Ref<any>,
    session: sessionRef as Ref<any>,
  }
}
