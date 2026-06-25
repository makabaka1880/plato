<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const { t } = useI18n()
</script>

<template>
  <div class="root">
    <div class="card">
      <div class="code">{{ t('notFound.title') }}</div>
      <p class="sub">{{ t('notFound.subtitle') }}</p>
      <button class="home-btn" @click="router.push('/')">{{ t('notFound.back') }}</button>
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

.code {
  font-size: 72px;
  font-weight: 400;
  letter-spacing: -0.03em;
  color: var(--color-border-strong);
  margin-bottom: 8px;
}

.sub {
  font-size: 15px;
  color: var(--color-muted);
  margin: 0 0 24px;
}

.home-btn {
  font-family: inherit;
  font-size: 14px;
  padding: 8px 28px;
  cursor: pointer;
  background: var(--color-primary);
  color: var(--color-primary-fg);
  border: none;
  border-radius: 4px;
}

.home-btn:hover {
  background: var(--color-primary-hover);
}
</style>
