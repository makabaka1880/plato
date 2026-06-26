<script setup lang="ts">
import { ref } from 'vue'
import type { Tactic } from '@/types'
import Katex from './Katex.vue'
import InlineLatex from './InlineLatex.vue'

defineProps<{
    tactic: Tactic
    animate: boolean
}>()

const detailOpen = ref(false)
</script>

<template>
    <!-- single root for TransitionGroup compatibility -->
    <div>
        <div class="card" :class="{ pop: animate }" @click="detailOpen = true">
            <div class="card-body">
                <div class="name">{{ tactic.name }}</div>
                <div class="rule">
                    <Katex :expr="'\\displaystyle ' + tactic.rule" />
                </div>
                <div class="desc">
                    <InlineLatex :text="tactic.description" />
                </div>
            </div>
            <div class="card-hover-label">click for specs</div>
        </div>

        <!-- detail popover -->
        <Teleport to="body">
        <Transition name="tactic-fade">
            <div v-if="detailOpen" class="tactic-backdrop" @click.self="detailOpen = false">
                <div class="tactic-detail">
                    <div class="tactic-detail-head">
                        <span class="tactic-detail-name">{{ tactic.name }}</span>
                        <button class="tactic-detail-close" @click="detailOpen = false">&times;</button>
                    </div>
                    <div class="tactic-detail-body">
                        <div class="tactic-detail-section">
                            <div class="tactic-detail-label">Syntax</div>
                            <code class="tactic-detail-syntax">{{ tactic.syntax }}</code>
                        </div>
                        <div class="tactic-detail-section">
                            <div class="tactic-detail-label">Rule</div>
                            <div class="tactic-detail-rule">
                                <Katex :expr="'\\displaystyle ' + tactic.rule" />
                            </div>
                        </div>
                        <div class="tactic-detail-section">
                            <div class="tactic-detail-label">What it means</div>
                            <div class="tactic-detail-text">
                                <InlineLatex :text="tactic.description" />
                            </div>
                        </div>
                        <div v-if="tactic.example" class="tactic-detail-section">
                            <div class="tactic-detail-label">Example</div>
                            <div class="tactic-detail-text">
                                <InlineLatex :text="tactic.example" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>
    </div>
</template>

<style lang="scss" scoped>
.card {
    position: relative;
    padding: 12px 14px;
    background: var(--color-card-bg);
    border-radius: 8px;
    border: 1px solid var(--color-border);
    font-size: 13px;
    line-height: 1.5;
    cursor: pointer;
    transition: border-color 0.15s;
    overflow: hidden;

    &:hover {
        border-color: var(--color-muted);
    }
}

.card-body {
    transition: filter 0.25s ease, opacity 0.25s ease;

    .card:hover & {
        filter: blur(2px);
        opacity: 0.3;
    }
}

.card-hover-label {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-muted);
    opacity: 0;
    transition: opacity 0.25s ease;
    pointer-events: none;

    .card:hover & {
        opacity: 1;
    }
}

.name {
    font-weight: 600;
    font-size: 14px;
    margin-bottom: 8px;
}

.rule {
    display: flex;
    justify-content: center;
    padding: 12px 0;
    font-size: 15px;
}

.desc {
    font-size: 12px;
    color: var(--color-subtle);
    margin-top: 8px;
}

.pop {
    animation: cardPop 0.4s ease;
}

@keyframes cardPop {
    0% {
        transform: scale(0.95);
        opacity: 0.5;
    }

    60% {
        transform: scale(1.02);
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}
</style>

<style lang="scss">
.tactic-backdrop {
    position: fixed;
    inset: 0;
    z-index: 300;
    background: rgba(0, 0, 0, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
}

.tactic-detail {
    background: var(--color-bg);
    border-radius: 12px;
    max-width: 520px;
    width: 92vw;
    max-height: 80vh;
    overflow-y: auto;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.tactic-detail-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px 12px;
    border-bottom: 1px solid var(--color-border);
}

.tactic-detail-name {
    font-weight: 600;
    font-size: 16px;
}

.tactic-detail-close {
    background: none;
    border: none;
    font-size: 22px;
    cursor: pointer;
    color: var(--color-muted);
    line-height: 1;

    &:hover {
        color: var(--color-fg);
    }
}

.tactic-detail-body {
    padding: 16px 20px 20px;
}

.tactic-detail-section {
    margin-bottom: 16px;
}

.tactic-detail-label {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: var(--color-muted);
    text-transform: uppercase;
    margin-bottom: 6px;
}

.tactic-detail-syntax {
    font-family: var(--font-mono);
    font-size: 13px;
    color: var(--color-primary-hover);
    padding: 2px 6px;
    background: var(--color-subtle-bg);
    border-radius: 4px;
    border: 1px solid var(--color-border);
}

.tactic-detail-rule {
    display: flex;
    justify-content: center;
    padding: 12px 0;
    font-size: 16px;
}

.tactic-detail-text {
    font-size: 13px;
    color: var(--color-subtle);
    line-height: 1.8;
    white-space: pre-wrap;
    font-family: var(--font-mono);
}

.tactic-fade-enter-active {
    transition: all 0.2s ease;

    .tactic-detail {
        transform: scale(0.95);
    }
}

.tactic-fade-leave-active {
    transition: all 0.15s ease;
}

.tactic-fade-enter-from,
.tactic-fade-leave-to {
    opacity: 0;
}
</style>
