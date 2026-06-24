<script setup lang="ts">
import { ref, nextTick, computed } from 'vue'
import type { Hint } from '@/types'
import { usePreferencesStore } from '@/stores/preferences'
import Katex from '@/components/Katex.vue'
import GuideCard from './GuideCard.vue'
import HintCard from './HintCard.vue'
import HelpModal from './HelpModal.vue'
import { useProofSession } from '@/composables/useProofSession'
import { useGuideProgression } from '@/composables/useGuideProgression'
import { useHints } from '@/composables/useHints'
import { useBracketPairing } from '@/composables/useBracketPairing'

const props = defineProps<{
  goalLatex: string
  premiseLatex?: string
  goal?: string
  premise?: string
  guides: Hint[]
  hints: Hint[]
}>()

const emit = defineEmits<{ stepTaken: []; solved: []; openPrefs: [] }>()

const prefs = usePreferencesStore()
const inpEl = ref<HTMLInputElement | null>(null)

const {
  ready, input, entries,
  stepLatex, run: sessionRun, reset: sessionReset,
  insertTactic: _insertTactic, isGoalResolved, SessionClass,
} = useProofSession()

const rawText = computed(() =>
  entries.value.map(e => `> ${e.cmd}\n${e.text}`).join('\n')
)

const {
  guideIdx, visibleGuide, advanceGuide, reset: guideReset,
} = useGuideProgression(() => props.guides)

const {
  hintRevealed, openHints, anyHintOpen,
  toggleHint, dismissHint, reset: hintsReset,
} = useHints(() => props.hints.length)

// ── guide input enforcement ───────────────────────────────────────
const guideInputDisabled = computed(() =>
  visibleGuide.value !== null && !visibleGuide.value.tactic
)

// ── help modal ──────────────────────────────────────────────────────
const showHelp = ref(false)

// ── execute ───────────────────────────────────────────────────────
function run() {
  const expr = input.value.trim()
  if (!expr) return
  input.value = ''

  // Intercept help / ? / (help) — show the help panel instead of sending to WASM.
  const norm = expr.replace(/[()]/g, '').trim()
  if (norm === 'help' || norm === '?') {
    showHelp.value = true
    scrollDown()
    return
  }

  const g = visibleGuide.value
  if (g && g.tactic && !SessionClass.value.tacticEquals(g.tactic, expr)) {
    entries.value.push({
      cmd: expr,
      text: `Error: expected \`${g.tactic}\``,
      error: true,
      step: null,
    })
    scrollDown()
    return
  }

  const { result, step } = sessionRun(expr)
  if (!result.startsWith('Error:') && step !== null) {
    const next = new Set(openHints.value)
    let changed = false
    for (const i of next) {
      if (props.hints[i].tactic && SessionClass.value.tacticEquals(props.hints[i].tactic!, expr)) {
        next.delete(i)
        changed = true
      }
    }
    if (changed) openHints.value = next

    advanceGuide()
    emit('stepTaken')

    if (props.goal && isGoalResolved(props.goal)) {
      emit('solved')
    }
  }
  scrollDown()
}

function scrollDown() {
  nextTick(() => {
    document.getElementById('judgements-end')?.scrollIntoView({ block: 'end' })
  })
}

function reset() {
  sessionReset()
  guideReset()
  hintsReset()
  history.value = []
  historyIdx.value = -1
  savedDraft = ''
  inpEl.value?.focus()
}

const { onKeydown: onBracketKeydown } = useBracketPairing(input, inpEl)

// ── command history ───────────────────────────────────────────────
const history = ref<string[]>([])
const historyIdx = ref(-1)
let savedDraft = ''

function onKeydown(e: KeyboardEvent) {
  onBracketKeydown(e)
  if (e.key === 'Enter' && !e.defaultPrevented) {
    const cmd = input.value.trim()

    // ── empty input: fill in the most prominent tactic ──────────
    if (!cmd) {
      const g = visibleGuide.value
      if (g?.tactic) {
        insertTactic(g.tactic)
        return
      }
      for (const i of openHints.value) {
        const h = props.hints[i]
        if (h?.tactic) {
          insertTactic(h.tactic)
          return
        }
      }
      for (const i of openHints.value) {
        if (!props.hints[i]?.tactic) {
          dismissHint(i)
          return
        }
      }
      return
    }

    if (history.value.length === 0 || history.value[history.value.length - 1] !== cmd) {
      history.value.push(cmd)
    }
    historyIdx.value = history.value.length
    savedDraft = ''
    run()
    return
  }
  if (e.key === 'ArrowUp') {
    e.preventDefault()
    if (historyIdx.value === history.value.length) {
      savedDraft = input.value
    }
    if (historyIdx.value > 0) {
      historyIdx.value--
      input.value = history.value[historyIdx.value]
    }
    return
  }
  if (e.key === 'ArrowDown') {
    e.preventDefault()
    if (historyIdx.value < history.value.length - 1) {
      historyIdx.value++
      input.value = history.value[historyIdx.value]
    } else if (historyIdx.value === history.value.length - 1) {
      historyIdx.value = history.value.length
      input.value = savedDraft
    }
    return
  }
}

/** Enter on the repl container: dismiss text-only guides/hints when input is disabled. */
function onReplKeydown(e: KeyboardEvent) {
  if (e.key !== 'Enter') return
  // let the input's own handler deal with this
  if (e.target === inpEl.value) return
  if (guideInputDisabled.value) {
    e.preventDefault()
    advanceGuide()
  }
}

function insertTactic(tactic: string) {
  _insertTactic(tactic)
  nextTick(() => inpEl.value?.focus())
}

defineExpose({ insertTactic })
</script>

<template>
  <div v-if="!ready" class="loading">loading wasm…</div>
  <div v-else class="repl" @keydown="onReplKeydown">
    <div class="goal">
      <div v-if="props.premiseLatex" class="premise-line">
        <Katex :expr="props.premiseLatex" />
      </div>
      <Katex :expr="props.goalLatex" />
    </div>

    <div v-if="prefs.viewMode === 'tex'" class="judgements">
      <div v-for="(e, i) in entries" :key="i" class="entry">
        <div class="cmd">&gt; {{ e.cmd }}</div>
        <div v-if="e.error" class="err">{{ e.text }}</div>
        <div v-else class="step">
          <Katex :expr="stepLatex(e.step!)" />
        </div>
      </div>
      <div id="judgements-end" />
    </div>

    <pre v-else class="judgements raw">{{ rawText }}</pre>

    <div class="mid-spacer" />

    <div class="guide-slot">
      <Transition name="fade-down">
        <GuideCard
          v-if="visibleGuide"
          :key="guideIdx"
          :guide="visibleGuide"
          @insert="insertTactic"
          @dismiss="advanceGuide()"
        />
      </Transition>
    </div>

    <div v-if="props.hints.length" class="hint-bar">
      <div class="bulbs">
        <button
          v-for="i in hintRevealed" :key="i"
          class="bulb"
          :class="{
            active: openHints.has(i - 1),
            pulse: i === hintRevealed && !openHints.has(i - 1),
          }"
          @click="toggleHint(i - 1)"
          :title="openHints.has(i - 1) ? 'hide hint ' + i : 'show hint ' + i"
        >💡</button>
      </div>

      <div v-if="anyHintOpen" class="hint-cards">
        <template v-for="(h, i) in props.hints" :key="i">
          <Transition name="fade-down">
            <HintCard
              v-if="openHints.has(i)"
              :hint="h"
              :pulse="true"
              @insert="insertTactic"
              @dismiss="dismissHint(i)"
            />
          </Transition>
        </template>
      </div>
    </div>

    <div class="input-wrap">
      <div class="input-row">
        <input
          ref="inpEl" v-model="input" @keydown="onKeydown"
          :disabled="guideInputDisabled"
          :placeholder="visibleGuide?.tactic ?? '...'"
          class="repl-input"
        />
        <button class="submit-btn" @click="run" :disabled="guideInputDisabled" title="submit">
          →
        </button>
      </div>
      <div class="help-hint">type <code>help</code> or <code>?</code> to see available commands</div>
    </div>

    <HelpModal v-if="showHelp" @close="showHelp = false" />
  </div>
</template>

<style scoped>
.loading { padding: 32px; font-size: 13px; }
.repl { display: flex; flex-direction: column; height: 100%; font-size: 14px; }

.goal {
  text-align: center;
  padding: clamp(36px, 8vh, 72px) 20px clamp(12px, 2vh, 20px);
  font-size: clamp(22px, 3.5vw, 36px);
  flex-shrink: 0;
}
.premise-line {
  font-size: clamp(14px, 2vw, 18px);
  color: var(--color-muted);
  margin-bottom: 6px;
}

/* ── output area ────────────────── */
.judgements {
  flex: 1; overflow-y: auto; min-height: 0;
  padding: 0 clamp(32px, 12vw, 160px);
  font-size: 14px; line-height: 1.8;
}
.entry { padding: 4px 0; }
.cmd { font-size: 11px; color: #bbb; }
.step { font-size: 15px; }
.raw { font-family: inherit; white-space: pre-wrap; }
.err { color: var(--color-error); }

.mid-spacer { flex-shrink: 0; height: clamp(16px, 3vh, 32px); }

/* ── guide slot ─────────────────── */
.guide-slot {
  flex-shrink: 0; display: flex; justify-content: center;
  padding: 0 clamp(32px, 12vw, 160px);
}

/* ── hints ──────────────────────── */
.hint-bar { flex-shrink: 0; padding: 8px clamp(32px, 12vw, 160px) 0; }
.bulbs { display: flex; gap: 6px; justify-content: center; }
.bulb {
  background: none; border: 1px solid var(--color-border-light); border-radius: 100%;
  font-size: 16px; width: 34px; height: 34px; cursor: pointer;
  display: flex; align-items: center; justify-content: center;
  opacity: 0.45; transition: all 0.2s;
}
.bulb:hover { opacity: 0.8; border-color: #aaa; }
.bulb.active { opacity: 1; border-color: var(--color-primary-hover); }
.bulb.pulse { animation: bulbPulse 1.2s ease-in-out infinite; }

@keyframes bulbPulse {
  0%, 100% { transform: scale(1); opacity: 0.5; }
  50%      { transform: scale(1.12); opacity: 0.9; border-color: #888; }
}

.hint-cards { margin-top: 8px; display: flex; flex-direction: column; align-items: center; }

/* ── transitions ────────────────── */
.fade-down-enter-active { transition: all 0.35s ease; }
.fade-down-leave-active { transition: all 0.2s ease; }
.fade-down-enter-from { opacity: 0; transform: translateY(-8px); }
.fade-down-leave-to   { opacity: 0; }

/* ── input ──────────────────────── */
.input-wrap {
  flex-shrink: 0;
  padding: clamp(12px, 2vh, 20px) clamp(32px, 12vw, 160px) clamp(40px, 10vh, 80px);
}
.input-row { display: flex; align-items: center; gap: 8px; }
.repl-input {
  flex: 1; font-family: inherit; font-size: 15px;
  padding: 6px 0; border: none;
  border-bottom: 1px solid var(--color-border-strong); outline: none; background: transparent;
}
.repl-input:focus { border-bottom-color: var(--color-primary-hover); }
.repl-input:disabled { opacity: 0.3; cursor: not-allowed; }

.submit-btn {
  background: none; border: 1px solid var(--color-border-light);
  border-radius: 100%; width: 32px; height: 32px;
  font-size: 16px; cursor: pointer;
  display: flex; align-items: center; justify-content: center;
  color: var(--color-muted); flex-shrink: 0;
  transition: all 0.15s;
}
.submit-btn:hover { border-color: var(--color-primary); color: var(--color-primary); }
.submit-btn:disabled { opacity: 0.2; cursor: not-allowed; }

.help-hint {
  text-align: center; font-size: 11px; color: var(--color-border-strong);
  margin-top: 6px; opacity: 0.5;
}
.help-hint code {
  font-family: inherit; font-size: 11px;
  padding: 0 3px; border: 1px solid var(--color-border);
  border-radius: 2px; background: var(--color-subtle-bg);
}
</style>
