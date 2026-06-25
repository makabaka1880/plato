<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePreferencesStore } from '@/stores/preferences'

const { t, locale } = useI18n()
const prefs = usePreferencesStore()

function toggleLocale() {
  const next = locale.value === 'en' ? 'zh' : 'en'
  locale.value = next
  prefs.setLocale(next)
}

const emit = defineEmits<{ close: [] }>()

const doneBtn = ref<HTMLButtonElement | null>(null)

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

      <button ref="doneBtn" class="done-btn" @click="emit('close')">{{ t('common.done') }}</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.backdrop {
  position: fixed; inset: 0;
  background: rgba(0,0,0,0.15);
  display: flex; align-items: center; justify-content: center;
  z-index: 100;
}
.modal {
  background: var(--color-bg);
  border-radius: 12px;
  padding: 24px 28px;
  min-width: 280px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.12);
}
.title {
  font-size: 12px; font-weight: 600; letter-spacing: 0.08em;
  color: var(--color-muted); text-transform: uppercase;
  margin-bottom: 20px;
}
.row {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 14px; padding: 8px 0;
}
.toggle {
  font-family: inherit; font-size: 12px;
  padding: 4px 14px; cursor: pointer;
  background: var(--color-subtle-bg);
  border: 1px solid var(--color-border-light);
  border-radius: 4px; color: inherit;
}
.toggle:hover { background: var(--color-border-light); }
.done-btn {
  margin-top: 20px; width: 100%;
  font-family: inherit; font-size: 13px;
  padding: 6px 0; cursor: pointer;
  background: var(--color-primary);
  color: var(--color-primary-fg); border: none;
  border-radius: 6px;
}
.done-btn:hover { background: var(--color-primary-hover); }
</style>
