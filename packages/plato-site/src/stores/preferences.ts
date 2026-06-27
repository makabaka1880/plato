import { defineStore } from 'pinia'

export const LOCALE_KEY = 'plato-locale'
const VIEW_MODE_KEY = 'plato-view-mode'

function loadLocale(): string {
  try {
    const raw = localStorage.getItem(LOCALE_KEY)
    if (raw === 'en' || raw === 'zh') return raw
  } catch { /* ignore */ }
  return 'en'
}

function loadViewMode(): 'tex' | 'text' {
  try {
    const raw = localStorage.getItem(VIEW_MODE_KEY)
    if (raw === 'tex' || raw === 'text') return raw
  } catch { /* ignore */ }
  return 'tex'
}

export const usePreferencesStore = defineStore('preferences', {
  state: () => ({
    viewMode: loadViewMode() as 'tex' | 'text',
    locale: loadLocale(),
  }),
  actions: {
    toggleViewMode() {
      this.viewMode = this.viewMode === 'tex' ? 'text' : 'tex'
      localStorage.setItem(VIEW_MODE_KEY, this.viewMode)
    },
    setLocale(loc: string) {
      this.locale = loc
      localStorage.setItem(LOCALE_KEY, loc)
    },
  },
})
