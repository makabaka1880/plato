<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

const sectionId = computed(() => {
    const raw = route.query.section
    return typeof raw === 'string' ? raw : ''
})

const n = computed(() => (Number(route.query.n) || 0) + 1)
const closest = computed(() => (Number(route.query.closest) || 0) + 1)

function goContinue() {
    if (sectionId.value) {
        router.push(`/section/${sectionId.value}/problem/${closest.value - 1}`)
    } else {
        router.push('/')
    }
}
</script>

<template>
    <div class="root">
        <div class="card">
            <div class="title">{{ t('locked.title') }}</div>
            <p class="sub">{{ t('locked.subtitle', { n }) }}</p>
            <p v-if="closest > 1" class="hint">{{ t('locked.hint') }}</p>
            <div class="actions">
                <button class="home-btn" @click="router.push('/')">{{ t('locked.back') }}</button>
                <button v-if="closest > 1" class="next-btn" @click="goContinue">
                    {{ t('home.continue') }}
                </button>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.root {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 20px;
}

.card {
    text-align: center;
}

.title {
    font-size: 36px;
    font-weight: 400;
    letter-spacing: -0.02em;
    color: var(--color-muted);
    margin-bottom: 8px;
}

.sub {
    font-size: 15px;
    color: var(--color-muted);
    margin: 0 0 8px;
}

.hint {
    font-size: 13px;
    color: var(--color-border-strong);
    margin: 0 0 24px;
}

.actions {
    display: flex;
    gap: 8px;
    justify-content: center;
    margin-top: 24px;
}

.home-btn {
    font-family: inherit;
    font-size: 14px;
    padding: 8px 24px;
    cursor: pointer;
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-muted);
    border-radius: 4px;
}

.home-btn:hover {
    border-color: var(--color-primary-hover);
    color: var(--color-primary-hover);
}

.next-btn {
    font-family: inherit;
    font-size: 14px;
    padding: 8px 24px;
    cursor: pointer;
    background: var(--color-primary);
    color: var(--color-primary-fg);
    border: none;
    border-radius: 4px;
}

.next-btn:hover {
    background: var(--color-primary-hover);
}
</style>
