<script setup lang="ts">
import { ref, nextTick, computed } from 'vue'
import type { Hint } from '@/types'
import { usePreferencesStore } from '@/stores/preferences'
import Katex from '@/components/Katex.vue'
import GuideCard from './GuideCard.vue'
import HintCard from './HintCard.vue'
import { useProofSession } from '@/composables/useProofSession'
import { useGuideProgression } from '@/composables/useGuideProgression'
import { useHints } from '@/composables/useHints'

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
  insertTactic, isGoalResolved, SessionClass,
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

// ── execute ───────────────────────────────────────────────────────
function run() {
  const expr = input.value.trim()
  if (!expr) return
  input.value = ''

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
  inpEl.value?.focus()
}

defineExpose({ insertTactic })
</script>

<template>
  <div v-if="!ready" class="loading">loading wasm…</div>
  <div v-else class="repl">
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
          ref="inpEl" v-model="input" @keydown.enter="run"
          :disabled="guideInputDisabled"
          :placeholder="visibleGuide?.tactic ?? '...'"
          class="repl-input"
        />
        <button class="submit-btn" @click="run" :disabled="guideInputDisabled" title="submit">
          →
        </button>
      </div>
    </div>
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
</style>
