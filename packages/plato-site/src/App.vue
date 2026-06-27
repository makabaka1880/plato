<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const showBanner = ref<boolean>(true)
</script>

<template>
    <div class="app">
        <div class="main">
            <div v-if="showBanner" class="dev-banner">
                <span class="banner-text">{{ t('home.dev_notice') }}</span>
                <button class="close-btn" @click="showBanner = false">&times;</button>
            </div>
            <router-view v-slot="{ Component, route }">
                <Transition name="page">
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
.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

.main {
    flex: 1;
    overflow: hidden;
    position: relative;
}

.page-enter-active,
.page-leave-active {
    transition: opacity 0.25s ease, transform 0.25s ease;
}

.page-enter-from {
    opacity: 0;
    transform: translateY(12px);
}

.page-leave-to {
    opacity: 0;
    transform: translateY(-8px);
}

.footer {
    flex-shrink: 0;
    text-align: center;
    padding: 6px 12px;
    font-size: 11px;
    color: var(--color-muted);
    border-top: 1px solid var(--color-border);

    a {
        color: inherit;
        font-weight: 500;
    }
}

.dev-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    font-size: 0.8rem;
    background-color: yellow;
    color: #000;

    .banner-text {
        flex: 1;
    }

    .close-btn {
        background: none;
        border: none;
        font-size: 1.2rem;
        cursor: pointer;
        padding: 0 4px;
        line-height: 1;
        opacity: 0.6;
        transition: opacity 0.2s;

        &:hover {
            opacity: 1;
        }
    }
}
</style>