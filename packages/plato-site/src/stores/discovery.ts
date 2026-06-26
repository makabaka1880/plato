import { defineStore } from 'pinia'

const KEY = 'plato-discoveries'

interface DiscoveryState {
    viewed: Record<string, boolean>
    position: Record<string, number>
}

function load(): DiscoveryState {
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (parsed && typeof parsed.viewed === 'object') {
                return {
                    viewed: parsed.viewed as Record<string, boolean>,
                    position: (parsed.position as Record<string, number>) ?? {},
                }
            }
        }
    } catch { /* ignore */ }
    return { viewed: {}, position: {} }
}

function save(state: DiscoveryState) {
    localStorage.setItem(KEY, JSON.stringify(state))
}

export const useDiscoveryStore = defineStore('discovery', {
    state: (): DiscoveryState => load(),

    actions: {
        markViewed(sectionId: string) {
            this.viewed[sectionId] = true
            save(this.$state)
        },

        isViewed(sectionId: string): boolean {
            return this.viewed[sectionId] ?? false
        },

        getPosition(sectionId: string): number {
            return this.position[sectionId] ?? 0
        },

        setPosition(sectionId: string, idx: number) {
            this.position[sectionId] = idx
            save(this.$state)
        },

        reset() {
            this.viewed = {}
            this.position = {}
            save(this.$state)
        },
    },
})
