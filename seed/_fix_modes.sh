#!/bin/bash
# 临时脚本：修正 WSL 后遗症导致的 100755 权限污染（用完即删）
set -e
cd /mnt/c/workspace/RzOps

echo "=== 修改前模式分布 ==="
git ls-files -s | awk '{print $1}' | sort | uniq -c

# 对所有 100755 中非 .sh/.py 的文件去掉可执行位（core.quotepath=false 防止中文文件名转义）
git -c core.quotepath=false ls-files -s | grep '^100755' | sed 's/^100755 [0-9a-f]* 0\t//' | grep -vE '\.(sh|py)$' | while IFS= read -r f; do
  git update-index --chmod=-x "$f"
done

echo "=== 修改后模式分布 ==="
git ls-files -s | awk '{print $1}' | sort | uniq -c

echo "=== 变更文件数 ==="
git status --short | wc -l

echo "=== 100755 剩余（应为纯脚本） ==="
git ls-files -s | grep '^100755' | sed 's/^100755 [0-9a-f]* 0\t//'
