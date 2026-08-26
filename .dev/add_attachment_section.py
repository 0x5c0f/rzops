#!/usr/bin/env python3
import re

# route -> (var_name, target_type)
pages = {
    'datacenters': ('dcData', 'data_center'),
    'providers': ('provData', 'provider'),
    'domains': ('d', 'domain'),
    'certificates': ('cert', 'certificate'),
    'database-instances': ('inst', 'database'),
    'ops-sites': ('site', 'site'),
    'credentials': ('credential', 'credential'),
    'backup-plans': ('plan', 'backup_plan'),
    'monitor-targets': ('target', 'monitor_target'),
}

for route, (var, ttype) in pages.items():
    path = f'/mnt/c/workspace/RzOps/rzops-web/src/routes/{route}/[id]/+page.svelte'
    with open(path, encoding='utf-8') as f:
        content = f.read()
    orig = content
    # 1) insert import right after <script lang="ts">
    imp = "  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';\n"
    content = content.replace('<script lang="ts">\n', '<script lang="ts">\n' + imp, 1)
    # 2) insert AttachmentSection before the last closing block
    marker = '  {/if}\n</div>\n'
    if marker in content:
        content = content.replace(marker, '    <AttachmentSection targetType="' + ttype + '" targetId={' + var + '.id} />\n  {/if}\n</div>\n', 1)
    else:
        print('MARKER NOT FOUND: ' + route)
        continue
    if content != orig:
        with open(path, 'w', encoding='utf-8', newline='') as f:
            f.write(content)
        print('OK ' + route)
    else:
        print('NOCHANGE ' + route)
