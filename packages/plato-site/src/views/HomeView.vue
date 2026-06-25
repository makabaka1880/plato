<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { loadProblems } from '@/data'
import { useProgressStore } from '@/stores/progress'
import { useTacticsStore } from '@/stores/tactics'
import { useRoadmapStore } from '@/stores/roadmap'
import { usePreferencesStore } from '@/stores/preferences'

const router = useRouter()
const { t, locale } = useI18n()
const prefs = usePreferencesStore()

function setLocale(loc: string) {
    locale.value = loc
    prefs.setLocale(loc)
}

const props = defineProps<{ hasProgress: boolean }>()

function onStart() {
    useProgressStore().reset()
    useTacticsStore().reset()
    useRoadmapStore().reset()
    router.push('/problem/0')
}

function onContinue() {
    const problems = loadProblems(locale.value)
    const idx = Math.min(useProgressStore().highestSolved + 1, problems.length - 1)
    router.push(`/problem/${idx}`)
}

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
            <button v-if="props.hasProgress" ref="continueBtn" class="hero-btn" @click="onContinue">
                {{ t('home.continue') }}
            </button>
            <button ref="startBtn" class="hero-btn" :class="props.hasProgress ? 'secondary' : ''"
                @click="onStart">
                {{ t('home.startFresh') }}
            </button>
            <button class="hero-btn custom-btn" @click="router.push('/custom')">
                {{ t('home.customProblem') }}
            </button>
        </div>
        <div class="sub-actions">
            <a href="https://github.com/makabaka1880/plato">{{ t('home.contribute') }}</a>
            <span class="sub-sep">·</span>
            <router-link to="/about">{{ t('home.about') }}</router-link>
            <span class="sub-sep">·</span>
            <a href="https://blog.makabaka1880.xyz/articles/260625-building-plato">{{ t('home.story') }}</a>
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
    padding-top: 8vh;
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

.sub-actions {
    display: flex;
    gap: 0;
    align-items: center;
    justify-content: center;
    margin-top: 72px;
    font-size: 12px;
    color: var(--color-border-strong);
}

.sub-actions a {
    color: inherit;
    text-decoration: none;
    padding: 2px 6px;
    cursor: pointer;
    transition: color 0.15s;
}

.sub-actions a:hover {
    color: var(--color-muted);
}

.sub-actions .sub-sep {
    color: var(--color-border);
    user-select: none;
}
</style>
