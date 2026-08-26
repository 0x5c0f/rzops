#!/usr/bin/env python3
import re

edits = {
    'servers': ('ServerResponse', 'name'),
    'certificates': ('CertificateResponse', 'name'),
    'database-instances': ('DatabaseInstanceResponse', 'name'),
    'ops-sites': ('OpsSiteResponse', 'name'),
    'credentials': ('CredentialResponse', 'name'),
    'backup-plans': ('BackupPlanResponse', 'name'),
    'monitor-targets': ('MonitorTargetResponse', 'name'),
    'server-ips': ('ServerIpResponse', 'ip_address'),
    'server-ports': ('ServerPortResponse', 'service_name'),
    'attachments': ('AttachmentResponse', 'filename'),
    'datacenters': ('DataCenterResponse', 'name'),
    'providers': ('ProviderResponse', 'name'),
    'domains': ('DomainResponse', 'domain_name'),
}

for route, (typ, firstcol) in edits.items():
    path = f'/mnt/c/workspace/RzOps/rzops-web/src/routes/{route}/+page.svelte'
    with open(path, encoding='utf-8') as f:
        content = f.read()
    orig = content
    # 1) handleEdit -> 编辑页 (route that currently goes to detail without /edit)
    content = content.replace(
        "goto(`/" + route + "/${item.id}`);",
        "goto(`/" + route + "/${item.id}/edit`);"
    )
    # 2) first column add link to detail
    pat = re.compile(r"\{\s*key: '" + firstcol + r"',\s*label: '[^']*'\s*\}")
    def repl(m):
        return m.group(0)[:-1] + ", link: (item: " + typ + ") => `/" + route + "/${item.id}` }"
    content = pat.sub(repl, content, count=1)
    if content != orig:
        with open(path, 'w', encoding='utf-8', newline='') as f:
            f.write(content)
        print('OK ' + route)
    else:
        print('NOCHANGE ' + route)
