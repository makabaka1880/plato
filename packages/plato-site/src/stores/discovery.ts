import { defineStore } from 'pinia'

const KEY = 'plato-discoveries'

interface DiscoveryState {
    viewed: Record<string, boolean>
}

function load(): DiscoveryState {
    try {
        const raw = localStorage.getItem(KEY)
        if (raw !== null) {
            const parsed = JSON.parse(raw)
            if (parsed && typeof parsed.viewed === 'object') {
                return { viewed: parsed.viewed as Record<string, boolean> }
            }
        }
    } catch { /* ignore */ }
    return { viewed: {} }
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

        reset() {
            this.viewed = {}
            save(this.$state)
        },
    },
})
