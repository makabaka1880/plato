<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePreferencesStore } from '@/stores/preferences'

const { t, locale } = useI18n()
const prefs = usePreferencesStore()

function setLocale(loc: string) {
    locale.value = loc
    prefs.setLocale(loc)
}

const props = defineProps<{ hasProgress: boolean }>()

const emit = defineEmits<{
    start: []
    continue: []
    goCustom: []
}>()

const continueBtn = ref<HTMLButtonElement | null>(null)
const startBtn = ref<HTMLButtonElement | null>(null)

onMounted(() => {
    if (props.hasProgress) continueBtn.value?.focus()
    else startBtn.value?.focus()
})
</script>

<template>
    <div class="hero">
        <div class="lang-switch">
            <button :class="{ active: locale === 'en' }" @click="setLocale('en')">EN</button>
            <button :class="{ active: locale === 'zh' }" @click="setLocale('zh')">中文</button>
        </div>
        <h1>{{ t('home.title') }}</h1>
        <div class="actions">
            <button v-if="props.hasProgress" ref="continueBtn" class="hero-btn" @click="emit('continue')">
                {{ t('home.continue') }}
            </button>
            <button ref="startBtn" class="hero-btn" :class="props.hasProgress ? 'secondary' : ''"
                @click="emit('start')">
                {{ t('home.startFresh') }}
            </button>
            <button class="hero-btn custom-btn" @click="emit('goCustom')">
                {{ t('home.customProblem') }}
            </button>
        </div>
    </div>
</template>

<style scoped>
.hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
}

.lang-switch {
    margin-bottom: 20px;
    display: flex;
    gap: 2px;
}

.lang-switch button {
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--color-muted);
    background: none;
    border: 1px solid transparent;
    border-radius: 3px;
    padding: 3px 8px;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
}

.lang-switch button:hover {
    color: var(--color-fg);
    border-color: var(--color-border);
}

.lang-switch button.active {
    color: var(--color-fg);
    border-color: var(--color-border);
    background: var(--color-subtle-bg);
}

.hero h1 {
    font-size: clamp(28px, 6vw, 56px);
    font-weight: 400;
    letter-spacing: -0.02em;
    margin-bottom: 32px;
}

.actions {
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: center;
}

.hero-btn {
    font-family: inherit;
    font-size: 15px;
    padding: 8px 0;
    cursor: pointer;
    width: 200px;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
}

.hero-btn:hover {
    background: var(--color-primary-hover);
}

.hero-btn.secondary {
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-muted);
}

.hero-btn.secondary:hover {
    background: var(--color-subtle-bg);
    border-color: var(--color-primary-hover);
    color: var(--color-primary-hover);
}

.hero-btn.custom-btn {
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-muted);
}

.hero-btn.custom-btn:hover {
    background: var(--color-subtle-bg);
    border-color: var(--color-primary-hover);
    color: var(--color-primary-hover);
}
</style>
