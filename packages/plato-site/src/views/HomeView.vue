<script setup lang="ts">
const emit = defineEmits<{ start: [] }>()
</script>

<template>
  <button class="hero" @click="emit('start')">
    <h1>? ⊢ Plato</h1>
    <span class="start-link">&rarr; Start</span>
  </button>
</template>

<style scoped>
.hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-decoration: none;
  color: inherit;
  border: none;
  background: none;
  cursor: pointer;
  width: 100%;
  font-family: inherit;
}
.hero h1 {
  font-size: clamp(28px, 6vw, 56px);
  font-weight: 400;
  letter-spacing: -0.02em;
}
.start-link {
  font-size: 16px;
  margin-top: 12px;
  opacity: 0.5;
}
</style>
