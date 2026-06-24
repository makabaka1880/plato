<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { useTacticsStore } from '@/stores/tactics'
import Katex from './Katex.vue'

const emit = defineEmits<{ close: [] }>()

const store = useTacticsStore()
const backdrop = ref<HTMLDivElement | null>(null)

function onDocKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault()
    emit('close')
  }
}

onMounted(async () => {
  await nextTick()
  backdrop.value?.focus()
  document.addEventListener('keydown', onDocKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', onDocKeydown)
})

interface CommandEntry {
  syntax: string
  description: string
  unlocked: boolean
  rule?: string
}

const groups: { label: string; entries: CommandEntry[] }[] = [
  {
    label: 'Basics',
    entries: [
      {
        syntax: '(assume F)',
        description: 'Assume a formula, adds {F} ⊢ F',
        unlocked: store.collected.has('assume'),
        rule: '\\frac{}{\\{p\\} \\vdash p}',
      },
      {
        syntax: '(show N)',
        description: 'Re-print step N',
        unlocked: true,
      },
      {
        syntax: 'F',
        description: 'Parse a formula (prints its structure)',
        unlocked: true,
      },
    ],
  },
  {
    label: 'Implication (→)',
    entries: [
      {
        syntax: '(->-intro F N)',
        description: 'Discharge formula F from step N to form an implication',
        unlocked: store.collected.has('→-intro'),
        rule: '\\frac{\\Gamma,\\; p \\vdash q}{\\Gamma \\vdash p \\to q}',
      },
      {
        syntax: '(->-elim N M)',
        description: 'Modus ponens — from p → q and p, derive q',
        unlocked: store.collected.has('→-elim'),
        rule: '\\frac{\\Gamma \\vdash p \\to q \\quad \\Delta \\vdash p}{\\Gamma \\cup \\Delta \\vdash q}',
      },
    ],
  },
  {
    label: 'Conjunction (∧)',
    entries: [
      {
        syntax: '(and-intro N M)',
        description: 'Combine steps N and M into a conjunction',
        unlocked: store.collected.has('∧-intro'),
        rule: '\\frac{\\Gamma \\vdash p \\quad \\Delta \\vdash q}{\\Gamma \\cup \\Delta \\vdash p \\land q}',
      },
      {
        syntax: '(and-elim-l N)',
        description: 'Extract the left half of a conjunction',
        unlocked: store.collected.has('∧-elim-l'),
        rule: '\\frac{\\Gamma \\vdash p \\land q}{\\Gamma \\vdash p}',
      },
      {
        syntax: '(and-elim-r N)',
        description: 'Extract the right half of a conjunction',
        unlocked: store.collected.has('∧-elim-r'),
        rule: '\\frac{\\Gamma \\vdash p \\land q}{\\Gamma \\vdash q}',
      },
    ],
  },
  {
    label: 'Disjunction (∨)',
    entries: [
      {
        syntax: '(or-intro-l N F)',
        description: 'From step N (proves p), form p ∨ F',
        unlocked: store.collected.has('∨-intro-l'),
        rule: '\\frac{\\Gamma \\vdash p}{\\Gamma \\vdash p \\lor q}',
      },
      {
        syntax: '(or-intro-r N F)',
        description: 'From step N (proves q), form F ∨ q',
        unlocked: store.collected.has('∨-intro-r'),
        rule: '\\frac{\\Gamma \\vdash q}{\\Gamma \\vdash p \\lor q}',
      },
      {
        syntax: '(or-elim N M K)',
        description: 'Proof by cases — from p ∨ q and two subproofs reaching r, conclude r',
        unlocked: store.collected.has('∨-elim'),
        rule: '\\frac{\\Gamma \\vdash p \\lor q \\quad \\Delta_1, p \\vdash r \\quad \\Delta_2, q \\vdash r}{\\Gamma \\cup \\Delta_1 \\cup \\Delta_2 \\vdash r}',
      },
    ],
  },
  {
    label: 'Negation (¬)',
    entries: [
      {
        syntax: '(not-intro F N M)',
        description: 'Reductio ad absurdum — from assuming F you reach a contradiction, so ¬F',
        unlocked: store.collected.has('¬-intro'),
        rule: '\\frac{\\Gamma, p \\vdash q \\quad \\Delta, p \\vdash \\lnot q}{\\Gamma \\cup \\Delta \\vdash \\lnot p}',
      },
      {
        syntax: '(not-elim N)',
        description: 'From ¬p derive p → ⊥',
        unlocked: store.collected.has('¬-elim'),
      },
      {
        syntax: '(dne N)',
        description: 'Double negation elimination — from ¬¬p derive p',
        unlocked: store.collected.has('¬¬-elim'),
      },
      {
        syntax: '(ex-falso N F)',
        description: 'From ⊥ derive anything (principle of explosion)',
        unlocked: store.collected.has('ex-falso'),
      },
    ],
  },
]

const unlockedCount = groups.reduce(
  (n, g) => n + g.entries.filter(e => e.unlocked).length,
  0,
)
</script>

<template>
  <div ref="backdrop" class="backdrop" tabindex="-1" @keydown.esc="emit('close')" @click.self="emit('close')">
    <div class="modal">
      <div class="head">
        <span class="title">commands</span>
        <span class="count">{{ unlockedCount }} unlocked</span>
        <button class="close-btn" @click="emit('close')">&times;</button>
      </div>

      <div class="body">
        <div v-for="group in groups" :key="group.label" class="group">
          <div class="group-label">{{ group.label }}</div>
          <div
            v-for="cmd in group.entries"
            :key="cmd.syntax"
            class="cmd"
            :class="{ locked: !cmd.unlocked }"
          >
            <div class="cmd-head">
              <code class="syntax">{{ cmd.syntax }}</code>
              <span v-if="!cmd.unlocked" class="lock-icon">🔒</span>
            </div>
            <div class="desc">{{ cmd.description }}</div>
            <div v-if="cmd.rule && cmd.unlocked" class="rule">
              <Katex :expr="cmd.rule" />
            </div>
          </div>
        </div>
      </div>

      <div class="foot">
        <span>press <kbd>Esc</kbd> or click outside to close</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.backdrop {
  position: fixed; inset: 0; z-index: 200;
  background: rgba(0, 0, 0, 0.3);
  display: flex; align-items: center; justify-content: center;
  outline: none;
}
.modal {
  background: var(--color-bg);
  border-radius: 12px;
  box-shadow: 0 12px 48px rgba(0, 0, 0, 0.18);
  max-height: 85vh;
  max-width: 620px;
  width: 92vw;
  display: flex; flex-direction: column;
}
.head {
  display: flex; align-items: center; gap: 10px;
  padding: 16px 20px 12px;
  border-bottom: 1px solid var(--color-border);
}
.title {
  font-size: 13px; font-weight: 600;
  letter-spacing: 0.06em; text-transform: uppercase;
}
.count { font-size: 11px; color: var(--color-muted); }
.close-btn {
  margin-left: auto; background: none; border: none;
  font-size: 20px; cursor: pointer; color: var(--color-muted);
}
.close-btn:hover { color: var(--color-primary-hover); }

.body {
  overflow-y: auto; padding: 10px 20px;
}
.group { margin-bottom: 14px; }
.group-label {
  font-size: 11px; font-weight: 600;
  letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--color-muted); margin-bottom: 6px;
  padding-top: 4px;
}
.cmd {
  padding: 6px 10px; margin-bottom: 4px;
  border-radius: 6px;
  border: 1px solid var(--color-border);
  background: var(--color-subtle-bg);
}
.cmd.locked {
  opacity: 0.35;
  background: transparent;
  border-style: dashed;
}
.cmd-head { display: flex; align-items: center; gap: 6px; }
.syntax {
  font-size: 12px; color: var(--color-primary-hover);
}
.locked .syntax { color: var(--color-muted); }
.lock-icon { font-size: 10px; }
.desc {
  font-size: 11px; color: var(--color-muted); margin-top: 2px;
}
.rule {
  margin-top: 4px; font-size: 14px;
  padding: 4px 0;
}

.foot {
  padding: 10px 20px 14px;
  font-size: 11px; color: var(--color-border-strong);
  border-top: 1px solid var(--color-border);
  text-align: center;
}
kbd {
  font-family: inherit; font-size: 10px;
  padding: 1px 5px; border: 1px solid var(--color-border-strong);
  border-radius: 3px; background: var(--color-subtle-bg);
}
</style>
