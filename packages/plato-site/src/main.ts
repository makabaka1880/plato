import 'katex/dist/katex.min.css'
import './style.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import { problems } from '@/data/problems'
import { useProgressStore } from '@/stores/progress'

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)
app.mount('#app')

/* ── debug cheats (browser console) ────────────────────────────────── */
;(window as any).__plato_unlockAll__ = () => {
  const store = useProgressStore(pinia)
  store.unlockAll(problems.length)
  console.log(`[plato] all ${problems.length} problems unlocked`)
}
