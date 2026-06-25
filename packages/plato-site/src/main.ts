import 'katex/dist/katex.min.css'
import './style.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import i18n from './i18n'
import router from './router'
import { loadSections } from '@/data'
import { useProgressStore } from '@/stores/progress'

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)
app.use(router)
app.use(i18n)
app.mount('#app')

/* ── debug cheats (browser console) ────────────────────────────────── */
;(window as any).__plato_unlockAll__ = () => {
  const store = useProgressStore(pinia)
  const sections = loadSections(i18n.global.locale.value as string)
  let total = 0
  for (const sec of sections) {
    store.unlockAll(sec.id, sec.problems.length)
    total += sec.problems.length
  }
  console.log(`[plato] all ${total} problems across ${sections.length} sections unlocked`)
}
