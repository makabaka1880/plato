<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()

const props = withDefaults(defineProps<{
    /** Which container to render into: inline column or slide-in overlay. */
    mode?: 'inline' | 'overlay'
    /** Show/hide for overlay mode. Defaults to visible for inline. */
    modelValue?: boolean
}>(), {
    mode: 'overlay',
    modelValue: false,
})

const emit = defineEmits<{
    'update:modelValue': [value: boolean]
}>()

const visible = ref(props.modelValue ?? props.mode === 'inline')

watch(() => props.modelValue, (v) => {
    visible.value = v ?? props.mode === 'inline'
})

const giscusEl = ref<HTMLElement | null>(null)
let loaded = false

function loadGiscus() {
    const el = giscusEl.value
    if (!el || loaded) return
    el.innerHTML = ''
    const script = document.createElement('script')
    script.src = 'https://giscus.app/client.js'
    script.setAttribute('data-repo', 'makabaka1880/plato')
    script.setAttribute('data-repo-id', 'R_kgDOTEIufQ')
    script.setAttribute('data-mapping', 'number')
    script.setAttribute('data-term', '1')
    script.setAttribute('data-reactions-enabled', '1')
    script.setAttribute('data-emit-metadata', '0')
    script.setAttribute('data-input-position', 'bottom')
    script.setAttribute('data-theme', 'light')
    script.setAttribute('data-lang', locale.value === 'zh' ? 'zh-CN' : 'en')
    script.setAttribute('crossorigin', 'anonymous')
    script.async = true
    el.appendChild(script)
    loaded = true
}

function reload() {
    loaded = false
    nextTick(() => loadGiscus())
}

if (props.mode === 'inline') {
    onMounted(() => { nextTick(() => loadGiscus()) })
} else {
    // Overlay mode: load only when panel becomes visible and DOM is ready
    watch(visible, (v) => {
        if (v) nextTick(() => loadGiscus())
    })
}

watch(locale, () => reload())

onUnmounted(() => {
    loaded = false
})
</script>

<template>
    <!-- Inline mode: a simple container -->
    <div v-if="props.mode === 'inline'" ref="giscusEl" class="giscus-wrap"></div>

    <!-- Overlay mode: teleported slide-in panel -->
    <Teleport v-else to="body">
        <Transition name="comments-slide">
            <div v-if="visible" class="comments-panel">
                <div class="comments-panel-head">
                    <span class="comments-panel-title">Comments</span>
                    <button class="comments-panel-close" @click="emit('update:modelValue', false)">&times;</button>
                </div>
                <div ref="giscusEl" class="giscus-wrap"></div>
            </div>
        </Transition>
    </Teleport>
</template>

<style lang="scss">
.comments-panel {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: 380px;
    max-width: 92vw;
    z-index: 100;
    background: var(--color-bg);
    border-left: 1px solid var(--color-border);
    box-shadow: -4px 0 16px rgba(0, 0, 0, 0.08);
    display: flex;
    flex-direction: column;
}

.giscus-wrap {}

.comments-panel-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 18px 12px;
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
}

.comments-panel-title {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
}

.comments-panel-close {
    background: none;
    border: none;
    font-size: 22px;
    cursor: pointer;
    color: var(--color-muted);
    line-height: 1;

    &:hover {
        color: var(--color-fg);
    }
}

.giscus-wrap {
    flex: 1;
    overflow-y: auto;
    min-height: 100%;
    margin: 24px;
}

.comments-slide-enter-active {
    transition: opacity 0.25s ease;

    .comments-panel {
        transition: transform 0.25s ease;
    }
}

.comments-slide-leave-active {
    transition: opacity 0.2s ease;

    .comments-panel {
        transition: transform 0.2s ease;
    }
}

.comments-slide-enter-from,
.comments-slide-leave-to {
    opacity: 0;

    .comments-panel {
        transform: translateX(100%);
    }
}
</style>
