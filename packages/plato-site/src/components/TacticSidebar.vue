<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useTacticsStore } from '@/stores/tactics'
import TacticCard from './TacticCard.vue'

const store = useTacticsStore()
const items = computed(() => store.all)

const open = ref(false)

// Track freshly-added names for pop animation
const fresh = ref(new Set<string>())
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
  <!-- narrow screen: hamburger toggle -->
  <button class="toggle-btn" @click="open = !open" title="tactics">
    📋
    <span v-if="items.length" class="badge">{{ items.length }}</span>
  </button>

  <!-- narrow screen: overlay -->
  <Transition name="slide">
    <div v-if="open" class="backdrop" @click.self="open = false">
      <div class="overlay">
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
    </div>
  </Transition>

  <!-- wide screen: inline sidebar -->
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
/* ── narrow: toggle button ──────── */
.toggle-btn {
  display: none;
  position: fixed;
  bottom: 16px;
  right: 16px;
  z-index: 50;
  background: var(--color-bg);
  border: 1px solid var(--color-border);
  border-radius: 100%;
  width: 44px;
  height: 44px;
  font-size: 18px;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}
.toggle-btn:hover { border-color: var(--color-muted); }

.badge {
  position: absolute;
  top: -4px;
  right: -4px;
  background: var(--color-primary);
  color: var(--color-primary-fg);
  font-size: 10px;
  width: 18px;
  height: 18px;
  border-radius: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* ── narrow: overlay ────────────── */
.backdrop {
  display: none;
  position: fixed;
  inset: 0;
  z-index: 80;
  background: rgba(0,0,0,0.2);
  justify-content: flex-end;
}
.backdrop > .overlay {
  width: min(300px, 85vw);
  background: var(--color-bg);
  overflow-y: auto;
  padding: 20px 18px;
  box-shadow: -4px 0 16px rgba(0,0,0,0.1);
}

/* ── wide: inline sidebar ───────── */
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

.slide-enter-active { transition: all 0.2s ease; }
.slide-leave-active { transition: all 0.15s ease; }
.slide-enter-from,
.slide-leave-to { opacity: 0; }

/* ── responsive breakpoint ───────── */
@media (max-width: 820px) {
  .sidebar { display: none; }
  .toggle-btn { display: flex; align-items: center; justify-content: center; }
  .backdrop { display: flex; }
}
</style>
