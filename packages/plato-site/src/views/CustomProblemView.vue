<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Problem, Hint } from '@/types'
import { useProblemLatex } from '@/composables/useProblemLatex'
import { useVictory } from '@/composables/useVictory'
import Katex from '@/components/Katex.vue'
import InlineLatex from '@/components/InlineLatex.vue'
import ProofRepl from '@/components/ProofRepl.vue'
import PreferenceModal from '@/components/PreferenceModal.vue'
import TacticSidebar from '@/components/TacticSidebar.vue'

const { t } = useI18n()

const emit = defineEmits<{
  home: []
}>()

// ── Setup state ─────────────────────────────────────────────────────
const urlInput = ref('')
const fetching = ref(false)
const error = ref('')
const fileInput = ref<HTMLInputElement | null>(null)

// ── Problem state ───────────────────────────────────────────────────
const problems = ref<Problem[]>([])
const problemIdx = ref(0)
const problem = computed(() => problems.value[problemIdx.value] ?? null)
const hasPrev = computed(() => problemIdx.value > 0)
const hasNext = computed(() => problemIdx.value < problems.value.length - 1)
const isSet = computed(() => problems.value.length > 1)

const problemRef = computed(() => problem.value)
const { goalLatex, premiseLatex, updateLatex } = useProblemLatex(problemRef)
const victory = useVictory()

const agreed = ref(false)
const showRepl = ref(false)
const proofLines = ref<string[]>([])
const prefsOpen = ref(false)
const loadedMsg = ref('')

// ── Schema validation ───────────────────────────────────────────────
type ProblemData = Omit<Problem, 'id'>

function isValidProblemData(obj: unknown): obj is ProblemData {
  if (!obj || typeof obj !== 'object') return false
  const o = obj as Record<string, unknown>
  if (typeof o.description !== 'string') return false
  if (typeof o.goal !== 'string') return false
  if (!Array.isArray(o.premise)) return false
  if (!o.premise.every(p => typeof p === 'string')) return false
  if (o.guides !== undefined && !Array.isArray(o.guides)) return false
  if (o.hints !== undefined && !Array.isArray(o.hints)) return false
  if (o.unlocks !== undefined && !Array.isArray(o.unlocks)) return false
  return true
}

function dataToProblem(data: ProblemData, id: number): Problem {
  return {
    id,
    description: data.description,
    goal: data.goal,
    premise: data.premise ?? [],
    guides: (data.guides as Hint[]) ?? [],
    hints: (data.hints as Hint[]) ?? [],
    unlocks: data.unlocks ?? [],
  }
}

function parseProblems(json: string): Problem[] | null {
  try {
    const data = JSON.parse(json)
    if (Array.isArray(data)) {
      const list: Problem[] = []
      for (let i = 0; i < data.length; i++) {
        if (!isValidProblemData(data[i])) return null
        list.push(dataToProblem(data[i], i + 1))
      }
      return list
    }
    if (isValidProblemData(data)) {
      return [dataToProblem(data, -1)]
    }
    return null
  } catch {
    return null
  }
}

// ── Load actions ────────────────────────────────────────────────────
async function loadFromUrl() {
  const raw = urlInput.value.trim()
  if (!raw) return
  error.value = ''
  fetching.value = true
  try {
    const resp = await fetch(raw)
    if (!resp.ok) {
      error.value = `HTTP ${resp.status}: ${resp.statusText}`
      return
    }
    const text = await resp.text()
    const parsed = parseProblems(text)
    if (!parsed) {
      error.value = t('custom.invalidJson')
      return
    }
    setProblems(parsed)
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e)
    error.value = `${t('common.error')}: ${msg}`
  } finally {
    fetching.value = false
  }
}

function onFileChange() {
  error.value = ''
  const file = fileInput.value?.files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    const text = reader.result as string
    const parsed = parseProblems(text)
    if (!parsed) {
      error.value = t('custom.invalidJson')
      return
    }
    setProblems(parsed)
  }
  reader.onerror = () => {
    error.value = t('common.error')
  }
  reader.readAsText(file)
}

function setProblems(list: Problem[]) {
  problems.value = list
  problemIdx.value = 0
  loadedMsg.value = list.length === 1
    ? t('custom.singleLoaded')
    : t('custom.setLoaded', { n: list.length })
  agreed.value = false
  showRepl.value = false
  proofLines.value = []
  victory.solved.value = false
  updateLatex()
}

// ── Navigation ──────────────────────────────────────────────────────
function onNext() {
  if (hasNext.value) {
    problemIdx.value++
  }
}

function onPrev() {
  if (hasPrev.value) {
    problemIdx.value--
  }
}

// Reset solving state when problem changes
watch(problemIdx, () => {
  agreed.value = false
  showRepl.value = false
  proofLines.value = []
  victory.solved.value = false
  updateLatex()
})

// ── Solving flow ────────────────────────────────────────────────────
async function onAgree() {
  agreed.value = true
  await nextTick()
  setTimeout(() => { showRepl.value = true }, 500)
}

function onSolved(lines: string[]) {
  proofLines.value = lines
  if (problem.value) {
    victory.fire(problem.value.unlocks)
  }
}

function stop() {
  problems.value = []
  problemIdx.value = 0
  agreed.value = false
  showRepl.value = false
  proofLines.value = []
  victory.solved.value = false
  error.value = ''
  urlInput.value = ''
  loadedMsg.value = ''
}
</script>

<template>
  <div class="root-row">
    <div class="root">
      <!-- Header -->
      <div class="header">
        <button class="logo" @click="emit('home')">Plato</button>
        <span class="spacer"></span>
        <span v-if="problem" class="goal-chip">{{ problem.goal }}</span>
        <span v-else-if="loadedMsg" class="loaded-chip">{{ loadedMsg }}</span>
        <span class="spacer"></span>
        <button class="prefs-link" @click="prefsOpen = true">{{ t('problem.preferences') }}</button>
      </div>

      <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />

      <!-- ═══ Setup: URL / File input ═══ -->
      <div v-if="!problems.length" class="body setup-body">
        <div class="setup-card">
          <p class="setup-desc">{{ t('custom.desc') }}</p>

          <!-- URL input -->
          <div class="input-row">
            <input
              v-model="urlInput"
              type="url"
              class="url-input"
              :placeholder="t('custom.urlPlaceholder')"
              @keydown.enter="loadFromUrl"
            />
            <button class="load-btn" :disabled="fetching || !urlInput.trim()" @click="loadFromUrl">
              {{ fetching ? t('custom.fetching') : t('custom.loadUrl') }}
            </button>
          </div>

          <!-- File input -->
          <div class="or-divider">
            <span>or</span>
          </div>
          <div class="file-row">
            <input
              ref="fileInput"
              type="file"
              accept=".json,application/json"
              class="file-hidden"
              @change="onFileChange"
            />
            <button class="load-btn file-btn" @click="fileInput?.click()">
              {{ t('custom.loadFile') }}
            </button>
          </div>

          <!-- Error -->
          <div v-if="error" class="error-msg">{{ error }}</div>

          <!-- Spec & Back -->
          <div class="back-row">
            <a class="spec-link" href="https://github.com/makabaka1880/plato/blob/main/CREATING-PROBLEMS.md" target="_blank">{{ t('custom.viewSpec') }}</a>
            <button class="back-btn" @click="emit('home')">{{ t('custom.back') }}</button>
          </div>
        </div>
      </div>

      <!-- ═══ Solving ═══ -->
      <div v-else-if="!problem" class="not-found">
        {{ t('problem.notFound') }}
      </div>
      <div v-else class="body">
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
            <div class="agree-row">
              <button class="agree-btn" @click="onAgree">{{ t('custom.agreed') }}</button>
              <button class="stop-btn" @click="stop">{{ t('custom.back') }}</button>
            </div>
          </div>
        </Transition>

        <Transition name="fade-in">
          <ProofRepl
            v-if="agreed && showRepl"
            :goal-latex="goalLatex"
            :premise-latex="premiseLatex"
            :goal="problem.goal"
            :premise="problem.premise"
            :guides="problem.guides"
            :hints="problem.hints"
            @solved="onSolved"
            style="flex:1;overflow:hidden"
          />
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
              <div class="victory-actions">
                <button v-if="hasNext" class="victory-btn" @click="onNext">
                  {{ t('problem.nextProblem') }}
                </button>
                <button v-else class="victory-btn" @click="emit('home')">
                  {{ t('problem.backHome') }}
                </button>
              </div>
            </div>
          </div>
        </Transition>
      </div>

      <!-- ═══ Footer nav (only for problem set) ═══ -->
      <div v-if="isSet" class="footer">
        <button :disabled="!hasPrev" class="nav-btn" @click="onPrev">{{ t('problem.prev') }}</button>
        <div class="footer-info">
          {{ problemIdx + 1 }} / {{ problems.length }}
        </div>
        <button :disabled="!hasNext" class="nav-btn" @click="onNext">{{ t('problem.next') }}</button>
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
  padding: 10px 16px;
  border-bottom: 1px solid var(--color-border);
  font-size: 15px;
  flex-shrink: 0;
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

.spacer {
  flex: 1;
}

.goal-chip {
  font-size: 13px;
  color: var(--color-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.loaded-chip {
  font-size: 13px;
  color: var(--color-primary-hover);
}

.prefs-link {
  font-family: inherit;
  font-size: 13px;
  cursor: pointer;
  background: none;
  border: none;
  color: var(--color-muted);
  padding: 2px 6px;
  border-radius: 4px;
}

.prefs-link:hover {
  color: var(--color-fg);
  background: var(--color-subtle-bg);
}

/* ── Setup ────────────────────────── */
.setup-body {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.setup-card {
  width: min(480px, 90vw);
  background: var(--color-subtle-bg);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 32px 28px;
  text-align: center;
}

.setup-desc {
  font-size: 14px;
  color: var(--color-muted);
  margin: 0 0 24px;
  line-height: 1.6;
}

.input-row {
  display: flex;
  gap: 8px;
}

.url-input {
  flex: 1;
  font-family: inherit;
  font-size: 13px;
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-bg);
  color: var(--color-fg);
  outline: none;
}

.url-input:focus {
  border-color: var(--color-primary-hover);
}

.url-input::placeholder {
  color: var(--color-border-strong);
}

.load-btn {
  font-family: inherit;
  font-size: 13px;
  padding: 8px 16px;
  cursor: pointer;
  background: var(--color-primary);
  color: var(--color-primary-fg);
  border: none;
  border-radius: 6px;
  white-space: nowrap;
}

.load-btn:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.load-btn:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.or-divider {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 20px 0;
  font-size: 12px;
  color: var(--color-border-strong);
}

.or-divider::before,
.or-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--color-border);
}

.file-row {
  display: flex;
  justify-content: center;
}

.file-hidden {
  display: none;
}

.file-btn {
  width: 100%;
}

.error-msg {
  margin-top: 16px;
  font-size: 13px;
  color: #c44;
}

.back-row {
  display: flex;
  gap: 12px;
  align-items: center;
  justify-content: center;
  margin-top: 20px;
}

.spec-link {
  font-family: inherit;
  font-size: 12px;
  color: var(--color-muted);
  text-decoration: none;
  border: 1px solid var(--color-border);
  padding: 6px 14px;
  border-radius: 6px;
  transition: color 0.15s, border-color 0.15s;
}

.spec-link:hover {
  color: var(--color-primary-hover);
  border-color: var(--color-primary-hover);
}

.back-btn {
  font-family: inherit;
  font-size: 13px;
  cursor: pointer;
  background: none;
  border: 1px solid var(--color-border);
  color: var(--color-muted);
  padding: 6px 20px;
  border-radius: 6px;
}

.back-btn:hover {
  border-color: var(--color-primary-hover);
  color: var(--color-primary-hover);
}

.stop-btn {
  font-family: inherit;
  font-size: 13px;
  cursor: pointer;
  background: none;
  border: 1px solid var(--color-border);
  color: var(--color-muted);
  padding: 6px 20px;
  border-radius: 6px;
}

.stop-btn:hover {
  border-color: var(--color-primary-hover);
  color: var(--color-primary-hover);
}

/* ── Footer nav ──────────────────── */
.footer {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-top: 1px solid var(--color-border);
  flex-shrink: 0;
}

.footer-info {
  flex: 1;
  text-align: center;
  font-size: 13px;
  color: var(--color-muted);
}

.nav-btn {
  padding: 4px 12px;
  font-family: inherit;
  font-size: 13px;
  cursor: pointer;
}

.nav-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

/* ── Prompt ───────────────────────── */
.body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  position: relative;
}

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

.agree-row {
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;
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

.not-found {
  padding: 32px;
}

/* ── Victory ──────────────────────── */
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
  max-height: 90vh;
  overflow-y: auto;
  padding: 20px 16px;
}

.victory-text {
  font-size: clamp(28px, 5vw, 48px);
  font-weight: 400;
  margin-bottom: 16px;
}

.proof-scroll {
  max-height: 35vh;
  overflow-y: auto;
  text-align: left;
  font-size: 13px;
  line-height: 2;
  color: var(--color-subtle);
  padding: 16px 18px;
  margin: 0 auto 24px;
  background: var(--color-subtle-bg);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  width: min(560px, 85vw);
  white-space: pre-wrap;
}

.victory-actions {
  display: flex;
  justify-content: center;
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

/* ── Transitions ──────────────────── */
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
