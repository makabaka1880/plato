<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useTacticsStore } from '@/stores/tactics'
import TacticCard from './TacticCard.vue'

const store = useTacticsStore()
const items = computed(() => store.all)

const open = ref(false)

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
  <!-- narrow: floating button -->
  <button class="toggle-btn" @click="open = true" title="tactics">
    📋
    <span v-if="items.length" class="badge">{{ items.length }}</span>
  </button>

  <!-- narrow: slide-in overlay -->
  <Teleport to="body">
    <Transition name="slide">
      <div v-if="open" class="backdrop" @click.self="open = false">
        <div class="overlay">
          <div class="overlay-header">
            <span class="title">Tactics Collected</span>
            <button class="close-btn" @click="open = false">&times;</button>
          </div>
          <TransitionGroup name="list" tag="div">
            <TacticCard
              v-for="t in items" :key="t.name"
              :tactic="t"
              :animate="fresh.has(t.name)"
            />
          </TransitionGroup>
          <div v-if="items.length === 0" class="empty">
            Solve problems to collect tactics
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>

  <!-- wide: inline sidebar -->
  <div class="sidebar">
    <div class="title">Tactics Collected</div>
    <TransitionGroup name="list" tag="div">
      <TacticCard
        v-for="t in items" :key="t.name"
        :tactic="t"
        :animate="fresh.has(t.name)"
      />
    </TransitionGroup>
    <div v-if="items.length === 0" class="empty">
      Solve problems to collect tactics
    </div>
  </div>
</template>

<style>
/* un-scoped so Teleported backdrop matches */
.backdrop {
  position: fixed; inset: 0; z-index: 200;
  background: rgba(0,0,0,0.2);
  display: flex; justify-content: flex-end;
}
.backdrop > .overlay {
  width: min(300px, 85vw);
  background: #fff;
  overflow-y: auto;
  padding: 20px 18px;
  box-shadow: -4px 0 16px rgba(0,0,0,0.1);
}
@media (min-width: 821px) {
  .backdrop { display: none; }
}

.slide-enter-active { transition: all 0.2s ease; }
.slide-leave-active { transition: all 0.15s ease; }
.slide-enter-from { opacity: 0; }
.slide-enter-from .overlay { transform: translateX(100%); }
.slide-enter-active .overlay { transition: transform 0.2s ease; }
.slide-leave-to   { opacity: 0; }
.slide-leave-to   .overlay { transform: translateX(100%); }
.slide-leave-active .overlay { transition: transform 0.15s ease; }
</style>

<style scoped>
.toggle-btn {
  display: none;
  position: fixed; bottom: 16px; right: 16px; z-index: 50;
  background: #fff; border: 1px solid var(--color-border);
  border-radius: 100%; width: 44px; height: 44px; font-size: 18px;
  cursor: pointer; box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  align-items: center; justify-content: center;
}
.toggle-btn:hover { border-color: var(--color-muted); }

.badge {
  position: absolute; top: -4px; right: -4px;
  background: var(--color-primary); color: var(--color-primary-fg);
  font-size: 10px; width: 18px; height: 18px;
  border-radius: 100%;
  display: flex; align-items: center; justify-content: center;
}

.overlay-header {
  display: flex; align-items: center; justify-content: space-between;
  margin-bottom: 12px;
}
.close-btn {
  background: none; border: none; font-size: 20px; cursor: pointer;
  color: var(--color-muted); line-height: 1;
}
.close-btn:hover { color: var(--color-fg); }

.sidebar {
  width: 280px; flex-shrink: 0; overflow-y: auto;
  padding: 20px 18px; border-left: 1px solid var(--color-border);
  height: 100%; box-sizing: border-box;
}
.title {
  font-size: 11px; font-weight: 600; letter-spacing: 0.08em;
  color: #bbb; text-transform: uppercase;
}
.empty {
  font-size: 12px; color: #bbb; line-height: 1.6;
}

.list-enter-active { transition: all 0.35s ease; }
.list-leave-active { transition: all 0.2s ease; }
.list-enter-from { opacity: 0; transform: translateX(12px); }

@media (max-width: 820px) {
  .sidebar   { display: none; }
  .toggle-btn { display: flex; }
}
</style>
