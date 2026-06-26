<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getSection } from '@/data'
import { useDiscoveryStore } from '@/stores/discovery'
import DiscoveryDialog from '@/components/DiscoveryDialog.vue'
import NavBar from '@/components/NavBar.vue'
import PreferenceModal from '@/components/PreferenceModal.vue'

const router = useRouter()
const { t, locale } = useI18n()
const discoveryStore = useDiscoveryStore()

const props = defineProps<{
    sectionId: string
}>()

const section = computed(() => getSection(props.sectionId, locale.value))
const prefsOpen = ref(false)

const progress = ref(0)
const totalLines = computed(() => section.value?.discovery.lines.length ?? 1)

function onProgress(idx: number) {
    progress.value = totalLines.value > 0 ? (idx + 1) / totalLines.value : 0
}

function onComplete() {
    discoveryStore.markViewed(props.sectionId)
    router.push(`/section/${props.sectionId}/problem/0`)
}

function onSkip() {
    discoveryStore.markViewed(props.sectionId)
    router.push(`/section/${props.sectionId}/problem/0`)
}
</script>

<template>
    <div v-if="!section" class="not-found">Section not found.</div>
    <div v-else class="root">
        <NavBar @open-prefs="prefsOpen = true">
            <span class="section-chip">{{ t(section.meta.nameI18nKey) }}</span>
        </NavBar>

        <PreferenceModal v-if="prefsOpen" @close="prefsOpen = false" />

        <div class="progress-bar">
            <div class="progress-fill" :style="{ width: (progress * 100) + '%' }"></div>
        </div>

        <div class="body">
            <DiscoveryDialog
                :discovery="section.discovery"
                :section-id="section.id"
                @progress="onProgress"
                @complete="onComplete"
                @skip="onSkip"
            />
        </div>
    </div>
</template>

<style lang="scss" scoped>
.not-found {
    padding: 32px;
    text-align: center;
    color: var(--color-muted);
}

.root {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
}

.section-chip {
    font-size: 13px;
    color: var(--color-muted);
}

.progress-bar {
    height: 2px; background: var(--color-border); flex-shrink: 0;
}
.progress-fill {
    height: 100%; width: 0; background: var(--color-primary-hover);
    transition: width 0.3s ease;
}

.body {
    flex: 1;
    overflow: hidden;
}

@media (max-width: 600px) {
    .section-chip { display: none; }
}
</style>
