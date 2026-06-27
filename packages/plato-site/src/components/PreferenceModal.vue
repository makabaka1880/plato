<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePreferencesStore } from '@/stores/preferences'
import { exportProgress, importProgress } from '@/utils/progress-io'

const { t, locale } = useI18n()
const prefs = usePreferencesStore()

function toggleLocale() {
  const next = locale.value === 'en' ? 'zh' : 'en'
  locale.value = next
  prefs.setLocale(next)
}

const emit = defineEmits<{ close: [] }>()

const doneBtn = ref<HTMLButtonElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const importMsg = ref<string | null>(null)
const importOk = ref(false)

function onDocKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault()
    emit('close')
  }
}

onMounted(async () => {
  await nextTick()
  doneBtn.value?.focus()
  document.addEventListener('keydown', onDocKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', onDocKeydown)
})

function onExport() {
  const data = exportProgress()
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `plato-progress-${new Date().toISOString().slice(0, 10)}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

function onImportClick() {
  importMsg.value = null
  fileInput.value?.click()
}

function onImportFile(e: Event) {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = () => {
    try {
      const data = JSON.parse(reader.result as string)
      const err = importProgress(data)
      if (err) {
        importMsg.value = err
        importOk.value = false
      } else {
        importMsg.value = 'Progress imported. Reloading…'
        importOk.value = true
        setTimeout(() => window.location.reload(), 600)
      }
    } catch {
      importMsg.value = 'Invalid JSON file.'
      importOk.value = false
    }
  }
  reader.readAsText(file)
  // reset so re-selecting the same file re-triggers
  input.value = ''
}
</script>

<template>
  <div class="backdrop" @click.self="emit('close')">
    <div class="modal">
      <div class="title">{{ t('preferences.title') }}</div>

      <div class="row">
        <span>{{ t('preferences.proofOutput') }}</span>
        <button class="toggle" @click="prefs.toggleViewMode()">
          {{ prefs.viewMode === 'tex' ? t('preferences.tex') : t('preferences.text') }}
        </button>
      </div>

      <div class="row">
        <span>{{ t('preferences.language') }}</span>
        <button class="toggle" @click="toggleLocale()">
          {{ locale === 'en' ? 'English' : '中文' }}
        </button>
      </div>

      <div class="separator"></div>

      <div class="row">
        <span>{{ t('preferences.exportProgress') }}</span>
        <button class="toggle" @click="onExport">{{ t('preferences.export') }}</button>
      </div>

      <div class="row">
        <span>{{ t('preferences.importProgress') }}</span>
        <button class="toggle" @click="onImportClick">{{ t('preferences.import') }}</button>
        <input ref="fileInput" type="file" accept=".json" class="file-inp" @change="onImportFile" />
      </div>

      <div v-if="importMsg" class="import-msg" :class="{ ok: importOk }">
        {{ importMsg }}
      </div>

      <button ref="doneBtn" class="done-btn" @click="emit('close')">{{ t('common.done') }}</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.15); display: flex; align-items: center; justify-content: center; z-index: 100; }
.modal { background: var(--color-bg); border-radius: 12px; padding: 24px 28px; min-width: 260px; max-width: 92vw; width: 340px; box-shadow: 0 8px 32px rgba(0,0,0,0.12); }
.title { font-size: 12px; font-weight: 600; letter-spacing: 0.08em; color: var(--color-muted); text-transform: uppercase; margin-bottom: 20px; }
.row { display: flex; align-items: center; justify-content: space-between; font-size: 14px; padding: 8px 0; }
.toggle { font-family: inherit; font-size: 12px; padding: 4px 14px; cursor: pointer; background: var(--color-subtle-bg); border: 1px solid var(--color-border-light); border-radius: 4px; color: inherit;
  &:hover { background: var(--color-border-light); }
}
.separator { height: 1px; background: var(--color-border); margin: 10px 0; }
.file-inp { display: none; }
.import-msg { margin-top: 8px; font-size: 12px; color: #c0392b; text-align: center;
  &.ok { color: var(--color-fol-on); }
}
.done-btn { margin-top: 20px; width: 100%; font-family: inherit; font-size: 13px; padding: 6px 0; cursor: pointer; background: var(--color-primary); color: var(--color-primary-fg); border: none; border-radius: 6px;
  &:hover { background: var(--color-primary-hover); }
}
</style>
