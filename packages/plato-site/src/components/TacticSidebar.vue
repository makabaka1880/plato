<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useTacticsStore } from '@/stores/tactics'
import TacticCard from './TacticCard.vue'

const store = useTacticsStore()
const items = computed(() => store.all)

// Track freshly-added names for pop animation
const fresh = ref(new Set<string>())

// When items change, mark new names
let prev = new Set<string>()
watch(items, (next) => {
  const cur = new Set(next.map(t => t.name))
  for (const name of cur) {
    if (!prev.has(name)) fresh.value.add(name)
  }
  prev = cur
}, { deep: true })
</script>

<template>
  <div class="sidebar">
    <div class="title">tactics collected</div>
    <TransitionGroup name="list" tag="div">
      <TacticCard
        v-for="t in items" :key="t.name"
        :tactic="t"
        :animate="fresh.has(t.name)"
      />
    </TransitionGroup>
    <div v-if="items.length === 0" class="empty">
      solve problems to collect tactics
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  width: 280px; flex-shrink: 0; overflow-y: auto;
  padding: 20px 18px; border-left: 1px solid var(--color-border);
  height: 100%; box-sizing: border-box;
}
.title {
  font-size: 11px; font-weight: 600; letter-spacing: 0.08em;
  color: #bbb; text-transform: uppercase; margin-bottom: 12px;
}

.empty {
  font-size: 12px; color: #bbb; line-height: 1.6;
}
.list-enter-active { transition: all 0.35s ease; }
.list-leave-active { transition: all 0.2s ease; }
.list-enter-from { opacity: 0; transform: translateX(12px); }
</style>
