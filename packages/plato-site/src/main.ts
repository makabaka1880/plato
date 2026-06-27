import 'katex/dist/katex.min.css'
import './style.scss'
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
    store.unlockAll(sec.id, sec.levels.length)
    total += sec.levels.length
  }
  console.log(`[plato] all ${total} levels across ${sections.length} sections unlocked`)
}

;(window as any).__plato_goto__ = (section: string | null, levelIdx: number) => {
  if (!section) {
    const sections = loadSections(i18n.global.locale.value as string)
    let offset = 0
    for (const sec of sections) {
      if (levelIdx < offset + sec.levels.length) {
        router.push({ name: 'level', params: { sectionId: sec.id, idx: String(levelIdx - offset) } })
        return
      }
      offset += sec.levels.length
    }
    console.log(`[plato] level ${levelIdx} not found`)
    return
  }
  router.push({ name: 'level', params: { sectionId: section, idx: String(levelIdx) } })
}

;(window as any).__plato_help__ = () => {
  console.log('[plato] debug cheats:')
  console.log('  __plato_unlockAll()           — unlock all levels')
  console.log('  __plato_goto(section, idx)    — jump to level index (e.g. __plato_goto("propositional", 5))')
  console.log('  __plato_goto(null, globalIdx) — jump by global level index')
}
