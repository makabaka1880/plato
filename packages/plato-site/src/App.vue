<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
    <div class="app">
        <div class="main">
            <router-view v-slot="{ Component, route }">
                <Transition name="page" mode="out-in">
                    <component :is="Component" :key="route.fullPath" />
                </Transition>
            </router-view>
        </div>
        <footer class="footer">
            {{ t('footer.rights') }}
            <a href="https://blog.makabaka1880.xyz" target="_blank">{{ t('footer.author') }}</a>
            &copy; 2026
        </footer>
    </div>
</template>

<style lang="scss" scoped>
.app { display: flex; flex-direction: column; height: 100vh; }
.main { flex: 1; overflow: hidden; position: relative; }

.page-enter-active, .page-leave-active { transition: opacity 0.25s ease, transform 0.25s ease; }
.page-enter-from { opacity: 0; transform: translateY(12px); }
.page-leave-to { opacity: 0; transform: translateY(-8px); }

.footer {
  flex-shrink: 0; text-align: center; padding: 6px 12px;
  font-size: 11px; color: var(--color-muted); border-top: 1px solid var(--color-border);
  a { color: inherit; font-weight: 500; }
}
</style>
