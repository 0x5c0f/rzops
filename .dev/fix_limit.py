#!/usr/bin/env python3
import os, re

base = '/mnt/c/workspace/RzOps/rzops-web/src'
files = []
for root, dirs, fnames in os.walk(base):
    for fn in fnames:
        if fn.endswith('.svelte') or fn.endswith('.ts'):
            files.append(os.path.join(root, fn))

for f in files:
    with open(f, encoding='utf-8') as fh:
        c = fh.read()
    o = c
    # limit: N -> per_page: N  (只针对 api 查询参数字面量)
    c = re.sub(r'\blimit:\s*(\d+)', r'per_page: \1', c)
    if c != o:
        with open(f, 'w', encoding='utf-8', newline='') as fh:
            fh.write(c)
        print('OK ' + f.replace(base, ''))
