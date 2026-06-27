export default {
  assume: '假设 {F} 成立，那么 {conclusion}。',
  extend: '在第 {n} 行的上下文里多放一个 {F}，{conclusion} 照样成立。',
  fix: '取一个任意的 {x}，就有了 {conclusion}。',
  subst: '第 {n} 行证明的结果不依赖具体符号——把里面的名字换掉，照样得到 {conclusion}。',
  show: '回看第 {n} 步。',
  parse: '解析一下这个公式：{F}。',

  '->-intro': '从 {F} 出发，走到了第 {n} 行的结论，于是可以宣布 {conclusion}。',
  '->-elim': '第 {n} 行说"如果前提成立，结论就跟着成立"，而第 {m} 行恰好满足了那个前提——于是有了 {conclusion}。',

  'and-intro': '第 {n} 行和第 {m} 行各给了一半；拼在一起，就是 {conclusion}。',
  'and-elim-l': '第 {n} 行断言了两件事；取左边那件，就够得到 {conclusion}。',
  'and-elim-r': '第 {n} 行断言了两件事；取右边那件，就够得到 {conclusion}。',

  'or-intro-l': '第 {n} 行已经成立了，在旁边再加一个 {F} 也不影响——于是有了 {conclusion}。',
  'or-intro-r': '第 {n} 行已经成立了，在前面放一个 {F} 也没关系——于是有了 {conclusion}。',
  'or-elim': '碰到一个分岔。第 {n} 行给出了两种可能；第 {m} 和第 {k} 行各自证明无论哪边都通向同一个结果，所以 {conclusion}。',

  'not-intro': '第 {n} 行和第 {m} 行互相矛盾。追根溯源，问题出在 {F} 这个假设上——所以否定它，得到 {conclusion}。',
  'not-elim': '第 {n} 行暗示顺着某条路走下去会出问题。沿着它走到底，得到 {conclusion}。',
  dne: '否定了两次，东西还在——第 {n} 行证实了这一点——那就只能承认它是真的：{conclusion}。',
  'ex-falso': '第 {n} 行给出了一个不可能的情况。从不可能出发，什么都推得出来——不如就拿 {conclusion} 吧。',

  'forall-intro': '第 {n} 行讨论 {x} 的时候没有附加任何特殊条件，所以对每一个 {x} 都成立：{conclusion}。',
  'forall-elim': '如果对一切都成立——第 {n} 行说确实如此——那对眼下的 {t} 自然也成立：{conclusion}。',
  'exists-intro': '第 {n} 行给了我们一个具体的例子 {t}，所以确实有东西满足 {x}：{conclusion}。',
  'exists-elim': '第 {n} 行告诉我们有东西满足 {x}。借着第 {m} 行提供的见证，我们最终得到 {conclusion}。',

  'box-intro': '第 {n} 行的证明不依赖任何世俗假设，因此它必然成立：{conclusion}。',
  'box-elim': '第 {n} 行说必然性尊重蕴含关系，而第 {m} 行给出了必然的前提——因此 {conclusion}。',
  'top-intro': '真随时可证，无需任何前提：{conclusion}。',
  'diamond-def-rev': '将第 {n} 行的双重否定通过定义折叠回来，得到 {conclusion}。',
  'diamond-def': '展开第 {n} 行中可能性的定义，得到 {conclusion}。',
}
