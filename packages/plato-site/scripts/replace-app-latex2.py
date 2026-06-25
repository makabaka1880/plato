#!/usr/bin/env python3
"""Replace $(App X Y)$ with $X(Y)$ in all problem guide/hint text. Both locales."""
import json, os, re

def replace_app_in_text(text: str) -> str:
    """Replace $(App X Y)$ patterns with $X(Y)$ in a text string."""

    def replacer(m):
        inner = m.group(0)
        # Replace (App pred arg) with pred(arg)
        # Use [^\s)]+ to match predicate/term without capturing closing parens
        pattern = re.compile(r'\(App\s+([^\s)]+)\s+([^\s)]+)\)')
        while True:
            new_inner, count = pattern.subn(r'\1(\2)', inner)
            if count == 0:
                break
            inner = new_inner
        return inner

    return re.sub(r'\$[^$]*\(App\s+[^\s)]+\s+[^\s)]+\)[^$]*\$', replacer, text)

# Apply to all problems
for loc in ['en', 'zh']:
    problems_dir = f'packages/plato-site/src/data/{loc}/problems'
    for f in sorted(os.listdir(problems_dir)):
        path = os.path.join(problems_dir, f)
        data = json.load(open(path))
        changed = False
        for sec in ['guides', 'hints']:
            for item in data.get(sec, []):
                if item.get('text'):
                    new_text = replace_app_in_text(item['text'])
                    if new_text != item['text']:
                        item['text'] = new_text
                        changed = True
        for u in data.get('unlocks', []):
            for key in ['description', 'example']:
                if key in u and u[key]:
                    new_text = replace_app_in_text(u[key])
                    if new_text != u[key]:
                        u[key] = new_text
                        changed = True
        if changed:
            json.dump(data, open(path, 'w'), ensure_ascii=False, indent=2)
            open(path, 'a').write('\n')
            print(f'{loc}/{f} OK')

print('Done.')
