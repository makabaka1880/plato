<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const { t } = useI18n()

const emit = defineEmits<{
    'open-prefs': []
}>()
</script>

<template>
    <div class="header">
        <button class="logo" @click="router.push('/')">{{ t('problem.logo') }}</button>
        <span class="spacer"></span>
        <slot />
        <span class="spacer"></span>
        <a class="gh-link" href="https://github.com/makabaka1880/plato" target="_blank" title="GitHub">GitHub</a>
        <button class="prefs-link" @click="emit('open-prefs')">{{ t('problem.preferences') }}</button>
    </div>
</template>

<style scoped>
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

.gh-link {
    font-family: inherit;
    font-size: 13px;
    color: var(--color-muted);
    text-decoration: none;
    padding: 2px 6px;
    border-radius: 4px;
}

.gh-link:hover {
    color: var(--color-fg);
    background: var(--color-subtle-bg);
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
</style>
