<script setup lang="ts">
import { computed, ref, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Problem } from '@/types'
import { useProblemLatex } from '@/composables/useProblemLatex'
import { useVictory } from '@/composables/useVictory'
import { useProgressStore } from '@/stores/progress'
import Katex from '@/components/Katex.vue'
import InlineLatex from '@/components/InlineLatex.vue'
import ProofRepl from '@/components/ProofRepl.vue'
import PreferenceModal from '@/components/PreferenceModal.vue'
import TacticSidebar from '@/components/TacticSidebar.vue'

const { t } = useI18n()

const props = defineProps<{
    problemIdx: number
    problems: Problem[]
}>()

const emit = defineEmits<{
    next: []
    prev: []
    home: []
}>()

const progress = useProgressStore()

const problem = computed(() => props.problems[props.problemIdx] ?? null)
const hasNext = computed(() =>
    props.problemIdx < props.problems.length - 1 &&
    props.problemIdx < progress.highestSolved + 1
)
const hasPrev = computed(() => props.problemIdx > 0)

const problemRef = computed(() => problem.value)
const { goalLatex, premiseLatex, updateLatex } = useProblemLatex(problemRef)
const victory = useVictory()

const prefsOpen = ref(false)
const agreed = ref(false)
const showRepl = ref(false)
const proofLines = ref<string[]>([])

watch(() => props.problemIdx, () => {
    agreed.value = false
    showRepl.value = false
    proofLines.value = []
    victory.solved.value = false
    updateLatex()
})

async function onAgree() {
    agreed.value = true
    await nextTick()
    setTimeout(() => { showRepl.value = true }, 500)
}

function onSolved(lines: string[]) {
    proofLines.value = lines
    if (problem.value) {
        victory.fire(problem.value.unlocks)
        progress.markSolved(props.problemIdx)
    }
}
</script>

<template>
    <div v-if="!problem" class="not-found">
        {{ t('problem.notFound') }}
    </div>
    <div v-else class="root-row">
        <div class="root">
            <div class="header">
                <button class="logo" @click="emit('home')">{{ t('problem.logo') }}</button>
                <span class="sep">/</span>
                <span>{{ props.problemIdx + 1 }} / {{ props.problems.length }}</span>
                <span class="spacer"></span>
                <span class="goal-chip">{{ problem.goal }}</span>
                <button class="gear-btn" @click="prefsOpen = true" :title="t('problem.preferences')">⚙</button>
            </div>

            <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />

            <div class="body">
                <Transition name="fade-up">
                    <div v-if="!agreed" class="prompt">
                        <div class="prove-label">{{ t('problem.makeMeBelieve') }}</div>
                        <div class="prove-desc">
                            <InlineLatex :text="problem.description" />
                        </div>
                        <div class="goal-line">
                            <span v-if="problem.premise.length" class="premise-label">{{ t('problem.premise') }}</span>
                            <span v-if="problem.premise.length" class="premise-katex">
                                <Katex :expr="premiseLatex" />
                            </span>
                            <span class="goal-label">{{ t('problem.goal') }}</span>
                            <span class="goal-katex">
                                <Katex :expr="goalLatex" />
                            </span>
                        </div>
                        <button class="agree-btn" @click="onAgree">{{ t('problem.agree') }}</button>
                    </div>
                </Transition>

                <Transition name="fade-in">
                    <ProofRepl v-if="agreed && showRepl" :goal-latex="goalLatex" :premise-latex="premiseLatex"
                        :goal="problem.goal" :premise="problem.premise" :guides="problem.guides" :hints="problem.hints"
                        @solved="onSolved" style="flex:1;overflow:hidden" />
                </Transition>

                <Transition name="fade-in">
                    <div v-if="victory.solved.value" class="victory-overlay">
                        <div class="victory">
                            <div class="victory-text">{{ t('problem.victory') }}</div>
                            <div v-if="proofLines.length" class="proof-scroll">
                                <div v-for="(line, i) in proofLines" :key="i" class="proof-line">
                                    <InlineLatex :text="line" />
                                </div>
                            </div>
                            <button v-if="hasNext" class="victory-btn" @click="emit('next')">
                                {{ t('problem.nextProblem') }}
                            </button>
                            <button v-else class="victory-btn" @click="emit('home')">
                                {{ t('problem.backHome') }}
                            </button>
                        </div>
                    </div>
                </Transition>
            </div>

            <div class="footer">
                <button @click="emit('prev')" :disabled="!hasPrev" class="nav-btn">{{ t('problem.prev') }}</button>
                <span class="spacer"></span>
                <button @click="emit('next')" :disabled="!hasNext" class="nav-btn">{{ t('problem.next') }}</button>
            </div>
        </div>

        <TacticSidebar />
    </div>
</template>

<style scoped>
.root-row {
    display: flex;
    flex-direction: row;
    height: 100%;
}

.root {
    display: flex;
    flex-direction: column;
    height: 100%;
    flex: 1;
    overflow: hidden;
}

.header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-bottom: 1px solid var(--color-border);
    font-size: 14px;
}

.logo {
    text-decoration: none;
    font-weight: 600;
    color: inherit;
    border: none;
    background: none;
    cursor: pointer;
    font-family: inherit;
    font-size: inherit;
    padding: 0;
}

.sep {
    color: var(--color-border-strong);
}

.spacer {
    flex: 1;
}

.goal-chip {
    font-size: 13px;
    color: var(--color-muted);
}

.gear-btn {
    background: none;
    border: none;
    font-size: 16px;
    cursor: pointer;
    opacity: 0.35;
    padding: 0 4px;
}

.gear-btn:hover {
    opacity: 0.7;
}

.body {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
}

.footer {
    display: flex;
    gap: 8px;
    padding: 8px 12px;
    border-top: 1px solid var(--color-border);
}

.nav-btn {
    padding: 4px 12px;
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
}

.not-found {
    padding: 32px;
}

/* ── prompt ─────────────────────── */
.prompt {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 40px 20px;
}

.prove-label {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.1em;
    color: var(--color-muted);
    margin-bottom: 16px;
}

.prove-desc {
    font-size: clamp(20px, 4vw, 32px);
    max-width: 700px;
    line-height: 1.45;
    margin-bottom: 32px;
}

.goal-line {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 15px;
    color: var(--color-subtle);
    margin-bottom: 32px;
}

.premise-label {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: var(--color-border-strong);
}

.premise-katex {
    font-size: 16px;
    color: var(--color-muted);
    margin-right: 16px;
}

.goal-label {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: #aaa;
}

.goal-katex {
    font-size: 18px;
    color: var(--color-primary-hover);
}

.agree-btn {
    font-family: inherit;
    font-size: 14px;
    padding: 8px 32px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
}

.agree-btn:hover {
    background: var(--color-primary-hover);
}

/* ── victory ───────────────────── */
.victory-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.92);
    z-index: 10;
}

.victory {
    text-align: center;
}

.victory-text {
    font-size: clamp(28px, 5vw, 48px);
    font-weight: 400;
    margin-bottom: 16px;
}

.proof-scroll {
    max-height: 40vh;
    overflow-y: auto;
    text-align: left;
    font-size: 13px;
    line-height: 2;
    color: var(--color-subtle);
    padding: 16px 24px;
    margin-bottom: 24px;
    background: var(--color-subtle-bg);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    max-width: 560px;
    white-space: pre-wrap;
}

.victory-btn {
    font-family: inherit;
    font-size: 15px;
    padding: 8px 28px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
    display: inline-block;
}

.victory-btn:hover {
    background: var(--color-primary-hover);
}

/* ── transitions ────────────────── */
.fade-up-enter-active {
    transition: all 0.4s ease;
}

.fade-up-leave-active {
    transition: all 0.4s ease;
}

.fade-up-enter-from {
    opacity: 0;
    transform: translateY(12px);
}

.fade-up-leave-to {
    opacity: 0;
    transform: translateY(-12px);
}

.fade-in-enter-active {
    transition: opacity 0.4s ease 0.3s;
}

.fade-in-leave-active {
    transition: opacity 0.15s ease;
}

.fade-in-enter-from {
    opacity: 0;
}

.fade-in-leave-to {
    opacity: 0;
}
</style>
