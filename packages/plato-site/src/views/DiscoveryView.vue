<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getSection } from '@/data'
import { useDiscoveryStore } from '@/stores/discovery'
import DiscoveryDialog from '@/components/DiscoveryDialog.vue'

const router = useRouter()
const { locale } = useI18n()
const discoveryStore = useDiscoveryStore()

const props = defineProps<{
    sectionId: string
}>()

const section = computed(() => getSection(props.sectionId, locale.value))

function onComplete() {
    discoveryStore.markViewed(props.sectionId)
    router.push(`/section/${props.sectionId}/problem/0`)
}

function onSkip() {
    router.push(`/section/${props.sectionId}/problem/0`)
}
</script>

<template>
    <div v-if="!section" class="not-found">Section not found.</div>
    <DiscoveryDialog
        v-else
        :discovery="section.discovery"
        :section-id="section.id"
        @complete="onComplete"
        @skip="onSkip"
    />
</template>

<style lang="scss" scoped>
.not-found {
    padding: 32px;
    text-align: center;
    color: var(--color-muted);
}
</style>
