<script setup lang="ts">
import { ref, onMounted } from 'vue'

const props = defineProps<{ hasProgress: boolean }>()

const emit = defineEmits<{
  start: []
  continue: []
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
        <h1>? ⊢ Plato</h1>
        <div class="actions">
          <button v-if="props.hasProgress" ref="continueBtn" class="hero-btn" @click="emit('continue')">
            Continue
          </button>
          <button ref="startBtn" class="hero-btn" :class="props.hasProgress ? 'secondary' : ''" @click="emit('start')">
            &rarr; Start fresh
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

.hero h1 {
    font-size: clamp(28px, 6vw, 56px);
    font-weight: 400;
    letter-spacing: -0.02em;
    margin-bottom: 32px;
}

.actions {
    display: flex; flex-direction: column; gap: 8px; align-items: center;
}

.hero-btn {
    font-family: inherit; font-size: 15px;
    padding: 8px 0; cursor: pointer; width: 200px;
    background: var(--color-primary); color: var(--color-primary-fg); border: none;
    border-radius: 4px;
}
.hero-btn:hover { background: var(--color-primary-hover); }

.hero-btn.secondary {
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-muted);
}
.hero-btn.secondary:hover {
    border-color: var(--color-primary-hover);
    color: var(--color-primary-hover);
}
</style>
