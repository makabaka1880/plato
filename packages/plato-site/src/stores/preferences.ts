import { defineStore } from 'pinia'

const LOCALE_KEY = 'plato-locale'

function loadLocale(): string {
  try {
    const raw = localStorage.getItem(LOCALE_KEY)
    if (raw === 'en' || raw === 'zh') return raw
  } catch { /* ignore */ }
  return 'en'
}

export const usePreferencesStore = defineStore('preferences', {
  state: () => ({
    viewMode: 'tex' as 'tex' | 'text',
    locale: loadLocale(),
  }),
  actions: {
    toggleViewMode() {
      this.viewMode = this.viewMode === 'tex' ? 'text' : 'tex'
    },
    setLocale(loc: string) {
      this.locale = loc
      localStorage.setItem(LOCALE_KEY, loc)
    },
  },
})
