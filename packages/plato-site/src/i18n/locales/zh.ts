export default {
    common: {
        ok: '确定',
        close: '×',
        dismiss: '关闭',
        done: '完成',
        loading: '加载 WASM 中…',
        submit: '提交',
        error: '错误',
    },

    home: {
        title: '? ⊢ 柏拉图',
        continue: '继续',
        startFresh: '→ 重新开始',
        customProblem: '→ 自定义题目',
    },

    custom: {
        title: '自定义题目',
        desc: '从 URL 或 JSON 文件加载题目或题目集。',
        urlPlaceholder: 'https://example.com/problem.json',
        loadUrl: '从 URL 加载',
        loadFile: '从文件加载',
        back: '← 返回',
        invalidJson: 'JSON 格式有误，请确认其符合题目格式。',
        fetching: '加载中…',
        agreed: '赞成',
        singleLoaded: '已加载 1 道题目。',
        setLoaded: '已加载 {n} 道题目。',
        viewSpec: '题目格式说明 ↗',
    },

    problem: {
        notFound: '未找到该问题。',
        logo: '柏拉图',
        preferences: '偏好设置',
        makeMeBelieve: '说服我',
        premise: '前提',
        goal: '目标',
        agree: '赞成',
        victory: '十分合理',
        nextProblem: '下一题 →',
        backHome: '返回首页',
        prev: '← 上一题',
        next: '下一题 →',
    },

    repl: {
        cmdPrefix: '> ',
        placeholder: '...',
        helpHint: '输入 {cmd} 查看命令和输入提示',
        tacticExpected: '错误：应输入 {tactic}',
        hideHint: '隐藏提示 {n}',
        showHint: '显示提示 {n}',
    },

    help: {
        tabs: {
            commands: '命令',
            notations: '符号',
            glossary: '术语表',
        },
        unlocked: '{n} 个已解锁',
        footer: '按 {key} 或点击外部关闭',

        groups: {
            basics: '基础',
            implication: '蕴含 (→)',
            conjunction: '合取 (∧)',
            disjunction: '析取 (∨)',
            negation: '否定 (¬)',
            quantifiers: '量词 (∀, ∃)',
        },

        commands: {
            assume: '假设一个公式，添加 {F} ⊢ F',
            fix: '引入一个项变量 {x} ⊢ x（类似 assume，但用于项）',
            subst: '在步骤 N 中统一替换原子公式',
            show: '重新显示步骤 N',
            parse: '解析一个公式 — 打印其结构',
            '->-intro': '从步骤 N 中消去公式 F，形成蕴含',
            '->-elim': '肯定前件 — 由 p → q 和 p 推出 q',
            'and-intro': '将步骤 N 和 M 合并为合取',
            'and-elim-l': '提取合取的左半部分',
            'and-elim-r': '提取合取的右半部分',
            'or-intro-l': '由步骤 N（证明 p），构造 p ∨ F',
            'or-intro-r': '由步骤 N（证明 q），构造 F ∨ q',
            'or-elim': '分情况证明 — 由 p ∨ q 和两个分别推出 r 的子证明，得出 r',
            'not-intro': '归谬法 — 假设 F 推出矛盾，从而得到 ¬F',
            'not-elim': '由 ¬p 推出 p → ⊥',
            dne: '双重否定消除 — 由 ¬¬p 推出 p',
            'ex-falso': '由 ⊥ 推出任意公式（爆炸原理）',
            'forall-intro': '全称引入 — 消去步骤 N 中的 x（x 不在其他假设中自由出现）',
            'forall-elim': '全称实例化 — 由 ∀x.φ 得到 φ[t/x]',
            'exists-intro': '存在引入 — 由 φ(t) 构造 ∃x.φ（将项 t 泛化为变量 x）',
            'exists-elim': '见证消除 — 步骤 N 证明 ∃x.φ，步骤 M 在见证 x 下证明 ψ，x 不在结论中自由出现',
        },

        notations: {
            typingSymbols: '符号输入',
            formulaSyntax: '公式语法',
            variablesTerms: '变量与项',
            syntaxIntro: '公式采用 <b>S-表达式</b>（前缀）记法。每个连接词用括号包裹其子公式。',
            negation: '否定',
            implication: '蕴含',
            conjunction: '合取',
            disjunction: '析取',
            universal: '全称',
            existential: '存在',
            varDesc: '<b>大写</b>字母（<code>A</code>、<code>P</code>、<code>Q</code>）是命题变量或谓词。<b>小写</b>字母（<code>x</code>、<code>y</code>、<code>t</code>）是项变量。',
            predApp: '谓词应用：<code>(App P x)</code> — 谓词 <code>P</code> 作用于项 <code>x</code>。嵌套应用：<code>(App (App R x) y)</code> 表示二元关系。',
        },
    },

    tactics: {
        title: '已收集的战术',
        toggle: '战术',
        empty: '解题可收集战术',
    },

    preferences: {
        title: '偏好设置',
        proofOutput: '证明输出',
        language: '语言',
        tex: 'TeX',
        text: '文本',
    },

    roadmap: {
        title: '你的旅程',
        empty: '解决第一个问题来开启旅程。',
        node: '问题 {n}',
        proof: '证明',
    },

    footer: {
        rights: '版权所有',
        author: 'Makabaka1880',
    },
}
