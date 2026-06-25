<script setup lang="ts">
import { ref, nextTick, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Hint } from '@/types'
import { usePreferencesStore } from '@/stores/preferences'
import { loadNlg } from '@/data'
import InlineLatex from '@/components/InlineLatex.vue'
import Katex from '@/components/Katex.vue'
import GuideCard from './GuideCard.vue'
import HintCard from './HintCard.vue'
import HelpModal from './HelpModal.vue'
import { useProofSession } from '@/composables/useProofSession'
import { useGuideProgression } from '@/composables/useGuideProgression'
import { useHints } from '@/composables/useHints'
import { useBracketPairing } from '@/composables/useBracketPairing'
import { useAutocomplete } from '@/composables/useAutocomplete'
import { useLiveParse } from '@/composables/useLiveParse'

const { t, locale } = useI18n()

const props = defineProps<{
    goalLatex: string
    premiseLatex?: string
    goal?: string
    premise?: string[]
    guides: Hint[]
    hints: Hint[]
    logicMode?: 'fol' | 'pl'
    allowedTactics?: string[]
}>()

const emit = defineEmits<{ stepTaken: []; solved: [proofLines: string[]]; openPrefs: [] }>()

const prefs = usePreferencesStore()
const inpEl = ref<HTMLInputElement | null>(null)

const logicModeRef = computed(() => props.logicMode ?? 'fol' as const)
const {
    ready, input, entries, session,
    stepLatex, run: sessionRun, reset: sessionReset,
    insertTactic: _insertTactic, isGoalResolved, SessionClass,
} = useProofSession(logicModeRef)

const liveParse = useLiveParse(input, session, stepLatex)

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

// ── compiled NLG proof ────────────────────────────────────────────
const proofLines = computed(() =>
  entries.value
    .filter(e => e.step !== null && e.meta)
    .map((e, i) => `${i + 1}. ${nlgText(e.meta!)}`)
)
const guideInputDisabled = computed(() =>
    visibleGuide.value !== null && !visibleGuide.value.tactic
)

// ── help modal ──────────────────────────────────────────────────────
const showHelp = ref(false)
const glossaryTerm = ref<string | null>(null)

function onGlossaryClick(term: string) {
  glossaryTerm.value = term
  showHelp.value = true
}

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
            text: t('repl.tacticExpected', { tactic: g.tactic }),
            error: true,
            step: null,
            meta: null,
        })
        scrollDown()
        return
    }

    const { result, step } = sessionRun(expr)
    if (!result.startsWith('Error:') && step !== null) {
        const next = new Set(openHints.value)
        let changed = false
        for (const i of next) {
            if (props.hints[i]!.tactic && SessionClass.value.tacticEquals(props.hints[i]!.tactic!, expr)) {
                next.delete(i)
                changed = true
            }
        }
        if (changed) openHints.value = next

        advanceGuide()
        emit('stepTaken')

        if (props.goal && isGoalResolved(props.goal)) {
            emit('solved', proofLines.value)
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
const stepPreviews = computed(() => {
  const map: Record<string, string> = {}
  for (const e of entries.value) {
    if (e.step !== null) {
      map[String(e.step)] = stepLatex(e.step)
    }
  }
  return map
})
const allowedTacticsRef = computed(() => props.allowedTactics ?? null)
const autocomplete = useAutocomplete(input, inpEl, stepPreviews, allowedTacticsRef)

// ── command history ───────────────────────────────────────────────
const history = ref<string[]>([])
const historyIdx = ref(-1)
let savedDraft = ''

function onKeydown(e: KeyboardEvent) {
    // Autocomplete takes priority when visible
    if (autocomplete.visible.value) {
      if (['Tab','ArrowUp','ArrowDown','Escape','Enter'].includes(e.key)) {
        autocomplete.onKeydown(e)
        if (e.key !== 'Enter') return
        // Enter: accept suggestion + close dropdown, don't submit
        if (e.defaultPrevented) return
      }
    }
    onBracketKeydown(e)
    if (e.key === 'Enter' && !e.defaultPrevented) {
        // ── live parse error: don't submit ────────────────────────
        if (liveParse.parseError.value) return

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
            input.value = history.value[historyIdx.value]!
        }
        return
    }
    if (e.key === 'ArrowDown') {
        e.preventDefault()
        if (historyIdx.value < history.value.length - 1) {
            historyIdx.value++
            input.value = history.value[historyIdx.value]!
        } else if (historyIdx.value === history.value.length - 1) {
            historyIdx.value = history.value.length
            input.value = savedDraft
        }
        return
    }
}

function insertTactic(tactic: string) {
    _insertTactic(tactic)
    nextTick(() => inpEl.value?.focus())
}

// ── NLG rendering ─────────────────────────────────────────────────
// Keys that hold LaTeX formula values (should be wrapped in $...$ for KaTeX)
const FORMULA_KEYS = new Set(['F', 'conclusion'])

function nlgText(meta: { cmdName: string; params: Record<string, string> }): string {
  const nlg = loadNlg(locale.value)
  const tpl = nlg[meta.cmdName]
  if (!tpl) return ''
  return tpl.replace(/\{(\w+)\}/g, (_, key: string) => {
    const val = meta.params[key] ?? `{${key}}`
    if (FORMULA_KEYS.has(key)) return '$' + val + '$'
    return val
  })
}

defineExpose({ insertTactic })
</script>

<template>
    <div v-if="!ready" class="loading">{{ t('common.loading') }}</div>
    <div v-else class="repl">
        <div class="goal">
            <div v-if="props.premiseLatex" class="premise-line">
                <Katex :expr="props.premiseLatex" />
            </div>
            <Katex :expr="props.goalLatex" />
        </div>

        <div v-if="prefs.viewMode === 'tex'" class="judgements">
            <div v-for="(e, i) in entries" :key="i" class="entry">
                <div class="entry-row">
                    <div class="cmd">&gt; {{ e.cmd }}</div>
                    <div v-if="e.meta" class="nlg"><InlineLatex :text="nlgText(e.meta)" @glossary-click="onGlossaryClick" /></div>
                </div>
                <div v-if="e.error" class="err">{{ e.text }}</div>
                <div v-else-if="e.step !== null" class="step">
                    <Katex :expr="stepLatex(e.step)" />
                </div>
                <div v-else class="step step-text">{{ e.text }}</div>
            </div>
            <div id="judgements-end" />
        </div>

        <pre v-else class="judgements raw">{{ rawText }}</pre>

        <div class="mid-spacer" />

        <div class="guide-slot">
            <GuideCard v-if="visibleGuide" :key="guideIdx" :guide="visibleGuide" @insert="insertTactic"
                @dismiss="advanceGuide()" @glossary-click="onGlossaryClick" />
        </div>

        <div v-if="props.hints.length" class="hint-bar">
            <div class="bulbs">
                <button v-for="i in hintRevealed" :key="i" class="bulb" :class="{
                    active: openHints.has(i - 1),
                    pulse: i === hintRevealed && !openHints.has(i - 1),
                }" @click="toggleHint(i - 1)"
                    :title="openHints.has(i - 1) ? t('repl.hideHint', { n: i }) : t('repl.showHint', { n: i })">💡</button>
            </div>

            <div v-if="anyHintOpen" class="hint-cards">
                <template v-for="(h, i) in props.hints" :key="i">
                    <Transition name="fade-down">
                        <HintCard v-if="openHints.has(i)" :hint="h" :pulse="true" @insert="insertTactic"
                            @dismiss="dismissHint(i)" @glossary-click="onGlossaryClick" />
                    </Transition>
                </template>
            </div>
        </div>

        <div class="input-wrap">
            <div class="input-row">
                <div v-if="autocomplete.visible.value" class="ac-dropdown">
                    <div
                        v-for="(s, i) in autocomplete.suggestions.value.slice(0, 6)"
                        :key="s.label"
                        class="ac-item"
                        :class="{ active: i === autocomplete.activeIndex.value, kindFormula: s.kind === 'formula', kindStep: s.kind === 'step' }"
                        @click="autocomplete.activeIndex.value = i; autocomplete.accept()"
                        @mousedown.prevent
                    >
                        <span class="ac-label">{{ s.label }}</span>
                        <span v-if="s.latex" class="ac-desc"><Katex :expr="s.latex" /></span>
                        <span v-else class="ac-desc">{{ s.description }}</span>
                    </div>
                </div>
                <input ref="inpEl" v-model="input" @keydown="onKeydown" @click="autocomplete.dismiss()" :disabled="guideInputDisabled"
                    :placeholder="visibleGuide?.tactic ?? t('repl.placeholder')" class="repl-input" />
                <button class="submit-btn" @click="run" :disabled="guideInputDisabled || !!liveParse.parseError.value" :title="t('common.submit')">
                    →
                </button>
            </div>
            <div v-if="liveParse.liveMeta.value" class="live-preview">
              <span class="live-tag">{{ liveParse.liveMeta.value.cmdName }}</span>
              <span v-if="liveParse.liveMeta.value.params.F" class="live-param"><InlineLatex :text="'$F$: $' + liveParse.liveMeta.value.params.F + '$'" /></span>
              <template v-for="(latex, key) in liveParse.liveMeta.value.stepPreviews" :key="key">
                <span class="live-param"><InlineLatex :text="'$' + key + '$: $' + latex + '$'" /></span>
              </template>
            </div>
            <div v-else-if="liveParse.parseError.value" class="live-preview live-err">{{ liveParse.parseError.value }}</div>
            <div class="help-hint">
              <i18n-t keypath="repl.helpHint" tag="span">
                <template #cmd><code>help</code></template>
              </i18n-t>
            </div>
        </div>

        <HelpModal v-if="showHelp" :glossary-term="glossaryTerm ?? undefined" :allowed-tactics="props.allowedTactics" @close="showHelp = false; glossaryTerm = null" />
    </div>
</template>

<style scoped>
.loading {
    padding: 32px;
    font-size: 13px;
}

.repl {
    display: flex;
    flex-direction: column;
    height: 100%;
    font-size: 14px;
}

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
    flex: 1;
    overflow-y: auto;
    min-height: 0;
    padding: 0 clamp(32px, 12vw, 160px);
    font-size: 14px;
    line-height: 1.8;
}

.entry {
    padding: 4px 0;
}

.entry-row {
    display: flex;
    flex-wrap: wrap;
    gap: 4px 12px;
    align-items: baseline;
}

.cmd {
    font-size: 11px;
    color: #bbb;
    flex-shrink: 0;
}

.nlg {
    font-size: 11px;
    color: var(--color-muted);
    line-height: 1.55;
}

.step {
    font-size: 15px;
}

.raw {
    font-family: inherit;
    white-space: pre-wrap;
}

.err {
    color: var(--color-error);
}

.mid-spacer {
    flex-shrink: 0;
    height: clamp(16px, 3vh, 32px);
}

/* ── guide slot ─────────────────── */
.guide-slot {
    flex-shrink: 0;
    display: flex;
    justify-content: center;
    padding: 0 clamp(32px, 12vw, 160px);
}

/* ── hints ──────────────────────── */
.hint-bar {
    flex-shrink: 0;
    padding: 8px clamp(32px, 12vw, 160px) 0;
}

.bulbs {
    display: flex;
    gap: 6px;
    justify-content: center;
}

.bulb {
    background: none;
    border: 1px solid var(--color-border-light);
    border-radius: 100%;
    font-size: 16px;
    width: 34px;
    height: 34px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.45;
    transition: all 0.2s;
}

.bulb:hover {
    opacity: 0.8;
    border-color: #aaa;
}

.bulb.active {
    opacity: 1;
    border-color: var(--color-primary-hover);
}

.bulb.pulse {
    animation: bulbPulse 1.2s ease-in-out infinite;
}

@keyframes bulbPulse {

    0%,
    100% {
        transform: scale(1);
        opacity: 0.5;
    }

    50% {
        transform: scale(1.12);
        opacity: 0.9;
        border-color: #888;
    }
}

.hint-cards {
    margin-top: 8px;
    display: flex;
    flex-direction: column;
    align-items: center;
}

/* ── transitions ────────────────── */
.fade-down-enter-active {
    transition: all 0.35s ease;
}

.fade-down-leave-active {
    transition: all 0.2s ease;
}

.fade-down-enter-from {
    opacity: 0;
    transform: translateY(-8px);
}

.fade-down-leave-to {
    opacity: 0;
}

/* ── input ──────────────────────── */
.input-wrap {
    flex-shrink: 0;
    padding: clamp(12px, 2vh, 20px) clamp(32px, 12vw, 160px) clamp(40px, 10vh, 80px);
    position: relative;
}

/* ── autocomplete dropdown ───────── */
.ac-dropdown {
    position: absolute;
    bottom: 100%;
    left: 0;
    right: 0;
    max-height: 220px;
    overflow-y: auto;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 6px 6px 0 0;
    box-shadow: 0 -4px 16px rgba(0,0,0,0.1);
    z-index: 10;
    margin-bottom: 4px;
}
.ac-item {
    display: flex;
    align-items: baseline;
    gap: 10px;
    padding: 6px 10px;
    font-size: 12px;
    cursor: pointer;
    border-bottom: 1px solid var(--color-border);
}
.ac-item:last-child {
    border-bottom: none;
}
.ac-item.active {
    background: var(--color-subtle-bg);
}
.ac-item.kindFormula {
    border-top: 1px solid var(--color-border);
}
.ac-item.kindStep .ac-desc {
    white-space: normal;
    overflow: visible;
    font-size: 10px;
}
.ac-label {
    font-weight: 600;
    color: var(--color-primary-hover);
    flex-shrink: 0;
    min-width: 60px;
}
.ac-desc {
    color: var(--color-muted);
    font-size: 11px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.input-row {
    display: flex;
    align-items: center;
    gap: 8px;
    position: relative;
}

.repl-input {
    flex: 1;
    font-family: inherit;
    font-size: 15px;
    padding: 6px 0;
    border: none;
    border-bottom: 1px solid var(--color-border-strong);
    outline: none;
    background: transparent;
}

.repl-input:focus {
    border-bottom-color: var(--color-primary-hover);
}

.repl-input:disabled {
    opacity: 0.3;
    cursor: not-allowed;
}

.submit-btn {
    background: none;
    border: 1px solid var(--color-border-light);
    border-radius: 100%;
    width: 32px;
    height: 32px;
    font-size: 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-muted);
    flex-shrink: 0;
    transition: all 0.15s;
}

.submit-btn:hover {
    border-color: var(--color-primary);
    color: var(--color-primary);
}

.submit-btn:disabled {
    opacity: 0.2;
    cursor: not-allowed;
}

/* ── live parse preview ─────────── */
.live-preview {
    text-align: center;
    font-size: 11px;
    color: var(--color-muted);
    margin-top: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
}
.live-tag {
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
}
.live-param {
    font-family: inherit;
}
.live-param code {
    font-family: inherit;
    font-size: 11px;
    padding: 0 3px;
    border: 1px solid var(--color-border);
    border-radius: 2px;
    background: var(--color-subtle-bg);
}
.live-err {
    color: var(--color-error);
}

.help-hint {
    text-align: center;
    font-size: 11px;
    color: var(--color-border-strong);
    margin-top: 6px;
    opacity: 0.5;
}

.help-hint code {
    font-family: inherit;
    font-size: 11px;
    padding: 0 3px;
    border: 1px solid var(--color-border);
    border-radius: 2px;
    background: var(--color-subtle-bg);
}
</style>
