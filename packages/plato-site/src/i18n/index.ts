import { createI18n } from 'vue-i18n'
import en from './locales/en'
import zh from './locales/zh'

const LOCALE_KEY = 'plato-locale'

function loadLocale(): string {
  try {
    const raw = localStorage.getItem(LOCALE_KEY)
    if (raw === 'en' || raw === 'zh') return raw
  } catch { /* ignore */ }
  return 'en'
}

const i18n = createI18n({
  legacy: false,
  locale: loadLocale(),
  fallbackLocale: 'en',
  messages: { en, zh },
})

export default i18n
export { LOCALE_KEY }
