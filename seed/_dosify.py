import pathlib, subprocess

out = subprocess.run(['git', 'diff', '--name-only', 'a06e720', 'HEAD'], capture_output=True, text=True).stdout
files = [f for f in out.splitlines() if f.strip()]

def blob(rev, f):
    r = subprocess.run(['git', 'show', f'{rev}:{f}'], capture_output=True)
    return r.stdout

def lf_lines(b):
    return b.replace(b'\r\n', b'\n').split(b'\n')  # 保留末尾空串

converted = 0
for f in files:
    p = pathlib.Path(f)
    if not p.exists() or not p.is_file():
        continue
    ob = blob('a06e720', f)
    wb = p.read_bytes()
    if ob == wb:
        continue
    old_lines = lf_lines(ob)
    # 简化：记录 a06e720 每行是否有 CR 结尾
    old_cr = [old_lines[i].endswith(b'\r') if False else False for i in range(len(old_lines))]
    # 更直接：按 \n 拆分保留 \r 标记
    ob_split = ob.split(b'\n')
    old_cr_set = set()
    for i, seg in enumerate(ob_split):
        if seg.endswith(b'\r'):
            old_cr_set.add(i)
    wb_split = wb.replace(b'\r\n', b'\n').split(b'\n')
    out_parts = []
    for i, seg in enumerate(wb_split):
        if i in old_cr_set:
            out_parts.append(seg + b'\r')
        else:
            out_parts.append(seg)
    target = b'\n'.join(out_parts)
    if wb != target:
        p.write_bytes(target)
        converted += 1
print(f'逐行对齐 {converted} 个文件')
