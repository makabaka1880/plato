<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTacticsStore } from '@/stores/tactics'
import { loadGlossary, type GlossaryEntry } from '@/data'
import Katex from './Katex.vue'
import katex from 'katex'

const props = defineProps<{
    glossaryTerm?: string
    allowedTactics?: string[]
}>()

const emit = defineEmits<{ close: [] }>()

const { t, locale } = useI18n()

const glossaryData = computed(() => loadGlossary(locale.value))

const store = useTacticsStore()
const allowedSet = computed(() => props.allowedTactics ? new Set(props.allowedTactics) : null)

// Filter groups: when section-scoped, hide unavailable groups
const visibleGroups = computed(() => {
    if (!allowedSet.value) return groups
    return groups.filter(g => {
        // Check if any entry in the group is in allowed tactics
        return g.entries.some(e => {
            // Entries without a specific tactic name (show, parse) are always shown
            if (e.syntax === '(show N)' || e.syntax === 'F') return true
            // Match by extracted tactic name from syntax
            const tacticName = e.syntax.match(/^\(([a-z][-a-z0-9]*)/)?.[1]
            if (!tacticName) return true
            return allowedSet.value!.has(tacticName)
        })
    })
})
const backdrop = ref<HTMLDivElement | null>(null)
const bodyRef = ref<HTMLDivElement | null>(null)
const activeTab = ref<'commands' | 'notations' | 'glossary'>('commands')
const targetedTerm = ref<string | null>(null)

function scrollToAlpha(letter: string) {
    const el = document.getElementById('gloss-key-' + letter)
    const body = bodyRef.value
    if (!el || !body) return
    const bodyRect = body.getBoundingClientRect()
    const elRect = el.getBoundingClientRect()
    const navHeight = (body.querySelector('.alpha-nav') as HTMLElement)?.offsetHeight ?? 0
    body.scrollTo({ top: body.scrollTop + elRect.top - bodyRect.top - navHeight - 8, behavior: 'smooth' })
}

function onDocKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
        e.preventDefault()
        emit('close')
    }
}

onUnmounted(() => {
    document.removeEventListener('keydown', onDocKeydown)
})

// ── Commands tab ───────────────────────────────────────────────────

interface CommandEntry {
    syntax: string
    i18nKey: string
    unlocked: boolean
    rule?: string
}

const groups: { i18nGroup: string; entries: CommandEntry[] }[] = [
    {
        i18nGroup: 'help.groups.basics',
        entries: [
            { syntax: '(assume F)', i18nKey: 'help.commands.assume', unlocked: store.collected.includes('assume'), rule: '\\frac{}{\\{p\\} \\vdash p}' },
            { syntax: '(fix x)', i18nKey: 'help.commands.fix', unlocked: true },
            { syntax: '(subst N (A F)...)', i18nKey: 'help.commands.subst', unlocked: store.collected.includes('subst'), rule: '\\frac{\\Gamma \\vdash p}{\\Gamma[\\vec{a} := \\vec{F}] \\vdash p[\\vec{a} := \\vec{F}]}' },
            { syntax: '(show N)', i18nKey: 'help.commands.show', unlocked: true },
            { syntax: 'F', i18nKey: 'help.commands.parse', unlocked: true },
        ],
    },
    {
        i18nGroup: 'help.groups.implication',
        entries: [
            { syntax: '(->-intro F N)', i18nKey: 'help.commands.->-intro', unlocked: store.collected.includes('->-intro'), rule: '\\frac{\\Gamma,\\; p \\vdash q}{\\Gamma \\vdash p \\to q}' },
            { syntax: '(->-elim N M)', i18nKey: 'help.commands.->-elim', unlocked: store.collected.includes('->-elim'), rule: '\\frac{\\Gamma \\vdash p \\to q \\quad \\Delta \\vdash p}{\\Gamma \\cup \\Delta \\vdash q}' },
        ],
    },
    {
        i18nGroup: 'help.groups.conjunction',
        entries: [
            { syntax: '(and-intro N M)', i18nKey: 'help.commands.and-intro', unlocked: store.collected.includes('and-intro'), rule: '\\frac{\\Gamma \\vdash p \\quad \\Delta \\vdash q}{\\Gamma \\cup \\Delta \\vdash p \\land q}' },
            { syntax: '(and-elim-l N)', i18nKey: 'help.commands.and-elim-l', unlocked: store.collected.includes('and-elim-l'), rule: '\\frac{\\Gamma \\vdash p \\land q}{\\Gamma \\vdash p}' },
            { syntax: '(and-elim-r N)', i18nKey: 'help.commands.and-elim-r', unlocked: store.collected.includes('and-elim-r'), rule: '\\frac{\\Gamma \\vdash p \\land q}{\\Gamma \\vdash q}' },
        ],
    },
    {
        i18nGroup: 'help.groups.disjunction',
        entries: [
            { syntax: '(or-intro-l N F)', i18nKey: 'help.commands.or-intro-l', unlocked: store.collected.includes('or-intro-l'), rule: '\\frac{\\Gamma \\vdash p}{\\Gamma \\vdash p \\lor q}' },
            { syntax: '(or-intro-r N F)', i18nKey: 'help.commands.or-intro-r', unlocked: store.collected.includes('or-intro-r'), rule: '\\frac{\\Gamma \\vdash q}{\\Gamma \\vdash p \\lor q}' },
            { syntax: '(or-elim N M K)', i18nKey: 'help.commands.or-elim', unlocked: store.collected.includes('or-elim'), rule: '\\frac{\\Gamma \\vdash p \\lor q \\quad \\Delta_1, p \\vdash r \\quad \\Delta_2, q \\vdash r}{\\Gamma \\cup \\Delta_1 \\cup \\Delta_2 \\vdash r}' },
        ],
    },
    {
        i18nGroup: 'help.groups.negation',
        entries: [
            { syntax: '(not-intro F N M)', i18nKey: 'help.commands.not-intro', unlocked: store.collected.includes('not-intro'), rule: '\\frac{\\Gamma, p \\vdash q \\quad \\Delta, p \\vdash \\lnot q}{\\Gamma \\cup \\Delta \\vdash \\lnot p}' },
            { syntax: '(not-elim N)', i18nKey: 'help.commands.not-elim', unlocked: store.collected.includes('not-elim') },
            { syntax: '(dne N)', i18nKey: 'help.commands.dne', unlocked: store.collected.includes('dne') },
            { syntax: '(ex-falso N F)', i18nKey: 'help.commands.ex-falso', unlocked: store.collected.includes('ex-falso') },
        ],
    },
    {
        i18nGroup: 'help.groups.quantifiers',
        entries: [
            { syntax: '(forall-intro x N)', i18nKey: 'help.commands.forall-intro', unlocked: store.collected.includes('forall-intro'), rule: '\\frac{\\Gamma, x \\vdash \\varphi \\quad x \\notin FV(\\Gamma)}{\\Gamma \\vdash \\forall x.\\; \\varphi}' },
            { syntax: '(forall-elim N t)', i18nKey: 'help.commands.forall-elim', unlocked: store.collected.includes('forall-elim'), rule: '\\frac{\\Gamma \\vdash \\forall x.\\; \\varphi}{\\Gamma \\vdash \\varphi[t/x]}' },
            { syntax: '(exists-intro N t x)', i18nKey: 'help.commands.exists-intro', unlocked: store.collected.includes('exists-intro'), rule: '\\frac{\\Gamma \\vdash \\varphi[t/x]}{\\Gamma \\vdash \\exists x.\\; \\varphi}' },
            { syntax: '(exists-elim N M x)', i18nKey: 'help.commands.exists-elim', unlocked: store.collected.includes('exists-elim'), rule: '\\frac{\\Gamma \\vdash \\exists x.\\; \\varphi \\quad \\Delta, x \\vdash \\psi \\quad x \\notin FV(\\Delta, \\psi)}{\\Gamma \\cup \\Delta \\vdash \\psi}' },
        ],
    },
    {
        i18nGroup: 'help.groups.modal',
        entries: [
            { syntax: '(top-intro)', i18nKey: 'help.commands.top-intro', unlocked: store.collected.includes('top-intro'), rule: '\\frac{}{\\Gamma \\vdash \\top}' },
            { syntax: '(box-intro N)', i18nKey: 'help.commands.box-intro', unlocked: store.collected.includes('box-intro'), rule: '\\frac{\\emptyset \\vdash A}{\\emptyset \\vdash \\Box A}' },
            { syntax: '(box-elim N M)', i18nKey: 'help.commands.box-elim', unlocked: store.collected.includes('box-elim'), rule: '\\frac{\\Gamma \\vdash \\Box (A \\to B) \\quad \\Delta \\vdash \\Box A}{\\Gamma \\cup \\Delta \\vdash \\Box B}' },
            { syntax: '(diamond-def N)', i18nKey: 'help.commands.diamond-def', unlocked: store.collected.includes('diamond-def'), rule: '\\diamond \\text{ definition}' },
        ],
    },
]

const unlockedCount = computed(() => visibleGroups.value.reduce(
    (n, g) => n + g.entries.filter(e => e.unlocked).length,
    0,
))

// ── Notations tab ──────────────────────────────────────────────────

interface SymbolEntry {
    symbol: string
    input: string
}

const symbols: SymbolEntry[] = [
    { symbol: '\\to', input: '->' },
    { symbol: '\\lnot', input: 'not' },
    { symbol: '\\forall', input: 'forall' },
    { symbol: '\\exists', input: 'exists' },
    { symbol: '\\bot', input: '_|_' },
    { symbol: '\\land', input: 'and' },
    { symbol: '\\lor', input: 'or' },
]

// ── Glossary tab ───────────────────────────────────────────────────

// Group entries by their leading letter key
const glossaryGroups = computed(() => {
    const map = new Map<string, GlossaryEntry[]>()
    for (const entry of glossaryData.value) {
        const k = entry.key.toUpperCase()
        if (!map.has(k)) map.set(k, [])
        map.get(k)!.push(entry)
    }
    return Array.from(map.entries()).sort(([a], [b]) => a.localeCompare(b))
})

// Unique sorted letter keys for the quick-nav bar
const glossaryKeys = computed(() => glossaryGroups.value.map(([k]) => k))

// Scroll to a specific glossary term after mount
async function scrollToGlossaryTerm(termId: string) {
    await nextTick()
    const el = document.querySelector(`[data-glossary-id="${CSS.escape(termId)}"]`) as HTMLElement | null
    const body = bodyRef.value
    if (!el || !body) return
    const bodyRect = body.getBoundingClientRect()
    const elRect = el.getBoundingClientRect()
    const navHeight = (body.querySelector('.alpha-nav') as HTMLElement)?.offsetHeight ?? 0
    body.scrollTo({ top: body.scrollTop + elRect.top - bodyRect.top - navHeight - 8, behavior: 'smooth' })
    // Flash animation
    targetedTerm.value = termId
    setTimeout(() => { targetedTerm.value = null }, 2000)
}

// On mount: handle deep-link to a glossary term
onMounted(async () => {
    await nextTick()
    document.addEventListener('keydown', onDocKeydown)
    if (props.glossaryTerm) {
        activeTab.value = 'glossary'
        await scrollToGlossaryTerm(props.glossaryTerm)
    }
})

function renderTex(text: string): string {
    return text.replace(/\$([^$]+)\$/g, (_: string, expr: string) => {
        try {
            return katex.renderToString(expr, { throwOnError: false, displayMode: false })
        } catch {
            return expr
        }
    })
}
</script>

<template>
    <div ref="backdrop" class="backdrop" tabindex="-1" @keydown.esc="emit('close')" @click.self="emit('close')">
        <div class="modal">
            <div class="head">
                <div class="tabs">
                    <button class="tab" :class="{ active: activeTab === 'commands' }" @click="activeTab = 'commands'">{{
                        t('help.tabs.commands') }}</button>
                    <button class="tab" :class="{ active: activeTab === 'notations' }"
                        @click="activeTab = 'notations'">{{ t('help.tabs.notations') }}</button>
                    <button class="tab" :class="{ active: activeTab === 'glossary' }" @click="activeTab = 'glossary'">{{
                        t('help.tabs.glossary') }}</button>
                </div>
                <span class="count">
                    <template v-if="activeTab === 'commands'">{{ t('help.unlocked', { n: unlockedCount }) }}</template>
                </span>
                <button class="close-btn" @click="emit('close')">&times;</button>
            </div>

            <!-- ═══ Commands tab ═══ -->
            <div v-if="activeTab === 'commands'" class="body">
                <div v-for="group in visibleGroups" :key="group.i18nGroup" class="group">
                    <div class="group-label">{{ t(group.i18nGroup) }}</div>
                    <div v-for="cmd in group.entries" :key="cmd.syntax" class="cmd" :class="{ locked: !cmd.unlocked }">
                        <div class="cmd-head">
                            <code class="syntax">{{ cmd.syntax }}</code>
                            <span v-if="!cmd.unlocked" class="lock-icon">🔒</span>
                        </div>
                        <div class="desc">{{ t(cmd.i18nKey) }}</div>
                        <div v-if="cmd.rule && cmd.unlocked" class="rule">
                            <Katex :expr="cmd.rule" />
                        </div>
                    </div>
                </div>
            </div>

            <!-- ═══ Notations tab ═══ -->
            <div v-if="activeTab === 'notations'" class="body">
                <div class="group">
                    <div class="group-label">{{ t('help.notations.typingSymbols') }}</div>
                    <div class="sym-table">
                        <div v-for="s in symbols" :key="s.symbol" class="sym-row">
                            <span class="sym-render">
                                <Katex :expr="s.symbol" />
                            </span>
                            <code class="sym-code">{{ s.input }}</code>
                        </div>
                    </div>
                </div>

                <div class="group">
                    <div class="group-label">{{ t('help.notations.formulaSyntax') }}</div>
                    <div class="gloss-entry">
                        <!-- eslint-disable-next-line vue/no-v-html -->
                        <p v-html="t('help.notations.syntaxIntro')"></p>
                    </div>
                    <div class="sym-table">
                        <div class="sym-row">
                            <span class="sym-name">{{ t('help.notations.negation') }}</span>
                            <code class="sym-code">(not p)</code>
                        </div>
                        <div class="sym-row">
                            <span class="sym-name">{{ t('help.notations.implication') }}</span>
                            <code class="sym-code">(-> p q)</code>
                        </div>
                        <div class="sym-row">
                            <span class="sym-name">{{ t('help.notations.conjunction') }}</span>
                            <code class="sym-code">(and p q)</code>
                        </div>
                        <div class="sym-row">
                            <span class="sym-name">{{ t('help.notations.disjunction') }}</span>
                            <code class="sym-code">(or p q)</code>
                        </div>
                        <div class="sym-row">
                            <span class="sym-name">{{ t('help.notations.universal') }}</span>
                            <code class="sym-code">(forall x body)</code>
                        </div>
                        <div class="sym-row">
                            <span class="sym-name">{{ t('help.notations.existential') }}</span>
                            <code class="sym-code">(exists x body)</code>
                        </div>
                    </div>
                </div>

                <div class="group">
                    <div class="group-label">{{ t('help.notations.variablesTerms') }}</div>
                    <div class="gloss-entry">
                        <!-- eslint-disable-next-line vue/no-v-html -->
                        <p v-html="t('help.notations.varDesc')"></p>
                        <!-- eslint-disable-next-line vue/no-v-html -->
                        <p v-html="t('help.notations.predApp')" style="margin-top: 6px;"></p>
                    </div>
                </div>

                <div class="group">
                    <div class="group-label">{{ t('help.notations.axiomSetTitle') }}</div>
                    <div class="gloss-entry axiom-pl">
                        <!-- eslint-disable-next-line vue/no-v-html -->
                        <p v-html="t('help.notations.axiomSetPL')"></p>
                    </div>
                    <div class="gloss-entry axiom-fol" style="margin-top: 4px;">
                        <!-- eslint-disable-next-line vue/no-v-html -->
                        <p v-html="t('help.notations.axiomSetFOL')"></p>
                    </div>
                </div>
            </div>

            <!-- ═══ Glossary tab ═══ -->
            <div v-if="activeTab === 'glossary'" ref="bodyRef" class="body glossary-body">
                <div v-if="glossaryKeys.length > 1" class="alpha-nav">
                    <button v-for="letter in glossaryKeys" :key="letter" class="alpha-btn"
                        @click="scrollToAlpha(letter)">{{ letter }}</button>
                </div>
                <div v-for="[letter, entries] in glossaryGroups" :key="letter">
                    <div :id="'gloss-key-' + letter" class="group-label alpha-anchor">{{ letter }}</div>
                    <div v-for="entry in entries" :key="entry.id" :data-glossary-id="entry.id"
                        :class="['gloss-entry', { 'gloss-flash': targetedTerm === entry.id }]">
                        <div class="gloss-term">{{ entry.term }}</div>
                        <div class="gloss-intuitive" v-html="renderTex(entry.intuitive)"></div>
                        <div class="gloss-def" v-html="renderTex(entry.definition)"></div>
                    </div>
                </div>
            </div>

            <div class="foot">
                <i18n-t keypath="help.footer" tag="span">
                    <template #key><kbd>Esc</kbd></template>
                </i18n-t>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.backdrop {
    position: fixed;
    inset: 0;
    z-index: 200;
    background: rgba(0, 0, 0, 0.3);
    outline: none;
    display: flex;
    align-items: center;
    justify-content: center;
}

.modal {
    background: var(--color-bg);
    border-radius: 12px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.18);
    max-height: 85vh;
    max-width: 620px;
    width: 92vw;
    display: flex;
    flex-direction: column;
}

.head {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 20px;
    border-bottom: 1px solid var(--color-border);
}

.tabs {
    display: flex;
    gap: 2px;
}

.tab {
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--color-muted);
    background: none;
    border: none;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: color 0.15s, background 0.15s;

    &:hover {
        color: var(--color-fg);
    }

    &.active {
        color: var(--color-fg);
        background: var(--color-subtle-bg);
    }
}

.count {
    font-size: 11px;
    color: var(--color-muted);
    margin-left: auto;
}

.close-btn {
    background: none;
    border: none;
    font-size: 20px;
    cursor: pointer;
    color: var(--color-muted);
    margin-left: 0;

    &:hover {
        color: var(--color-primary-hover);
    }
}

.body {
    overflow-y: auto;
    padding: 10px 20px;
}

.group {
    margin-bottom: 14px;
}

.group-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-muted);
    margin-bottom: 6px;
    padding-top: 4px;
}

// ── Commands ─────────────────────────────────────────────────
.cmd {
    padding: 6px 10px;
    margin-bottom: 4px;
    border-radius: 6px;
    border: 1px solid var(--color-border);
    background: var(--color-subtle-bg);

    &.locked {
        opacity: 0.35;
        background: transparent;
        border-style: dashed;

        .syntax {
            color: var(--color-muted);
        }
    }
}

.cmd-head {
    display: flex;
    align-items: center;
    gap: 6px;
}

.syntax {
    font-size: 12px;
    color: var(--color-primary-hover);
}

.lock-icon {
    font-size: 10px;
}

.desc {
    font-size: 11px;
    color: var(--color-muted);
    margin-top: 2px;
}

.rule {
    margin-top: 4px;
    font-size: 14px;
    padding: 4px 0;
}

// ── Notations ────────────────────────────────────────────────
.sym-table {
    border: 1px solid var(--color-border);
    border-radius: 6px;
    overflow: hidden;
}

.sym-row {
    display: flex;
    align-items: center;
    padding: 6px 10px;
    border-bottom: 1px solid var(--color-border);

    &:last-child {
        border-bottom: none;
    }
}

.sym-render {
    width: 48px;
    flex-shrink: 0;
    font-size: 16px;
    text-align: center;
    color: var(--color-fg);
}

.sym-name {
    width: 110px;
    flex-shrink: 0;
    font-size: 11px;
    color: var(--color-muted);
}

.sym-code {
    font-family: inherit;
    font-size: 12px;
    color: var(--color-primary-hover);
}

// ── Glossary ─────────────────────────────────────────────────
.glossary-body {
    padding-top: 0;
}

.alpha-nav {
    display: flex;
    flex-wrap: wrap;
    gap: 2px;
    margin: 0 0 12px;
    padding: 0 0 10px;
    border-bottom: 1px solid var(--color-border);
    position: sticky;
    top: 0;
    background: var(--color-bg);
    z-index: 1;
}

.alpha-btn {
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    line-height: 1;
    color: var(--color-muted);
    background: none;
    border: 1px solid transparent;
    border-radius: 3px;
    width: 24px;
    height: 24px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.15s, border-color 0.15s;

    &:hover {
        color: var(--color-fg);
        border-color: var(--color-border);
    }
}

.alpha-anchor {
    scroll-margin-top: 46px;
}

.gloss-flash {
    animation: glossHighlight 1.8s ease-out;
}

@keyframes glossHighlight {
    0% {
        background: var(--color-glossary-flash);
        border-color: #b0d0f0;
    }

    100% {
        background: var(--color-subtle-bg);
        border-color: var(--color-border);
    }
}

.gloss-entry {
    padding: 8px 10px;
    margin-bottom: 4px;
    border-radius: 6px;
    border: 1px solid var(--color-border);
    background: var(--color-subtle-bg);
    scroll-margin-top: 46px;
    font-size: 11px;


    code {
        font-family: inherit;
        padding: 0 3px;
        border: 1px solid var(--color-border);
        border-radius: 2px;
        background: var(--color-bg);
    }
}

.axiom-pl :deep(b) {
    color: var(--color-fol-off);
}

.axiom-fol :deep(b) {
    color: var(--color-fol-on);
}

.gloss-term {
    font-size: 12px;
    font-weight: 600;
    color: var(--color-fg);
    margin-bottom: 3px;
}

.gloss-intuitive {
    font-size: 11px;
    color: var(--color-fg);
    line-height: 1.65;
    padding: 6px 8px;
    margin-bottom: 6px;
    background: var(--color-hint-bg);
    border-left: 3px solid var(--color-hint-border);
    border-radius: 0 4px 4px 0;
}

.gloss-def {
    font-size: 11px;
    color: var(--color-muted);
    line-height: 1.65;

    code {
        font-family: inherit;
        font-size: 11px;
        padding: 0 3px;
        border: 1px solid var(--color-border);
        border-radius: 2px;
        background: var(--color-bg);
    }
}

.foot {
    padding: 10px 20px 14px;
    font-size: 11px;
    color: var(--color-border-strong);
    border-top: 1px solid var(--color-border);
    text-align: center;
    flex-shrink: 0;
}

kbd {
    font-family: inherit;
    font-size: 10px;
    padding: 1px 5px;
    border: 1px solid var(--color-border-strong);
    border-radius: 3px;
    background: var(--color-subtle-bg);
}
</style>
