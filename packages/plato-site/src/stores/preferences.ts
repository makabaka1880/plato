import { defineStore } from 'pinia'

export const usePreferencesStore = defineStore('preferences', {
  state: () => ({
    /** TeX or text rendering for proof output */
    viewMode: 'tex' as 'tex' | 'text',
  }),
  actions: {
    toggleViewMode() {
      this.viewMode = this.viewMode === 'tex' ? 'text' : 'tex'
    },
  },
})
