#!/usr/bin/env python3
import re, os

pages = [
    'attachments', 'backup-plans', 'certificates', 'contracts', 'credentials',
    'database-instances', 'datacenters', 'domains', 'monitor-targets', 'ops-sites',
    'providers', 'server-ips', 'server-ports', 'servers',
]
base = '/mnt/c/workspace/RzOps/rzops-web/src'

# ---------- types ----------
types = [
    'attachment', 'backup_plan', 'certificate', 'contract', 'credential',
    'database_instance', 'datacenter', 'domain', 'provider', 'server',
    'server_ip', 'server_port',
]
for t in types:
    p = f'{base}/lib/types/{t}.ts'
    with open(p, encoding='utf-8') as f:
        c = f.read()
    o = c
    c = re.sub(r'(\n\s*)limit\?: number;', r'\1page?: number;', c)
    c = re.sub(r'(\n\s*)offset\?: number;', r'\1per_page?: number;', c)
    if c != o:
        with open(p, 'w', encoding='utf-8', newline='') as f:
            f.write(c)
        print('TYPES OK ' + t)
    else:
        print('TYPES NOCHANGE ' + t)

# ---------- pages ----------
for r in pages:
    p = f'{base}/routes/{r}/+page.svelte'
    if not os.path.exists(p):
        print('MISSING ' + r)
        continue
    with open(p, encoding='utf-8') as f:
        c = f.read()
    o = c

    c = c.replace('{ limit: 20, offset: 0 }', '{ page: 1, per_page: 20 }')
    c = c.replace('{ limit: 200, offset: 0 }', '{ page: 1, per_page: 200 }')
    c = re.sub(r'let offset = \$derived\(query\.offset \?\? 0\);', 'let page = $derived(query.page ?? 1);', c)
    c = re.sub(r'let limit = \$derived\(query\.limit \?\? 20\);', 'let perPage = $derived(query.per_page ?? 20);', c)
    # handleSearch: offset: 0 -> page: 1
    c = re.sub(r'offset: 0 \};', 'page: 1 };', c)
    # handlePageChange
    c = re.sub(
        r'function handlePageChange\(newOffset: number\) \{\n(\s+)query = \{ \.\.\.query, offset: newOffset \};\n(\s+)loadData\(\);\n(\s+)\}',
        r'function handlePageChange(newPage: number) {\n\1query = { ...query, page: newPage };\n\2loadData();\n\3}',
        c,
    )
    # HTML display
    c = c.replace(
        'Math.min(offset + 1, total)}-{Math.min(offset + limit, total)}',
        'Math.min((page - 1) * perPage + 1, total)}-{Math.min(page * perPage, total)}',
    )
    c = c.replace('disabled={offset === 0}', 'disabled={page <= 1}')
    c = c.replace('handlePageChange(Math.max(0, offset - limit))', 'handlePageChange(page - 1)')
    c = c.replace('disabled={offset + limit >= total}', 'disabled={page * perPage >= total}')
    c = c.replace('handlePageChange(offset + limit)', 'handlePageChange(page + 1)')

    if c != o:
        with open(p, 'w', encoding='utf-8', newline='') as f:
            f.write(c)
        print('PAGE OK ' + r)
    else:
        print('PAGE NOCHANGE ' + r)
