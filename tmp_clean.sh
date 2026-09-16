#!/bin/bash
cd /mnt/c/workspace/RzOps || exit 1
git rm --cached tmp_commit.sh tmp_doc3.py 2>&1 | tail -2
rm -f tmp_commit.sh tmp_doc3.py
# 检查是否还有其他 tmp 残留
ls tmp_* 2>/dev/null && echo "WARN: tmp files remain" || echo "no tmp files remain"
git add -A
git status --short | head -6
git commit -m 'chore: 移除误入库的临时脚本（tmp_commit.sh / tmp_doc3.py）' 2>&1 | tail -1
git push origin feature/frontend-ui-refactor 2>&1 | tail -1
