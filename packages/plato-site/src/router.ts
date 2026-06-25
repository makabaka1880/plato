import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import ProblemView from '@/views/ProblemView.vue'
import CustomProblemView from '@/views/CustomProblemView.vue'
import AboutView from '@/views/AboutView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import LockedView from '@/views/LockedView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', name: 'home', component: HomeView },
    {
      path: '/problem/:idx',
      name: 'problem',
      component: ProblemView,
      props: r => ({ problemIdx: Number(r.params.idx) }),
    },
    { path: '/custom', name: 'custom', component: CustomProblemView },
    { path: '/about', name: 'about', component: AboutView },
    { path: '/locked', name: 'locked', component: LockedView },
    { path: '/:pathMatch(.*)*', name: 'notFound', component: NotFoundView },
  ],
})

export default router
