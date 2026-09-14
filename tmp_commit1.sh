#!/bin/bash
cd /mnt/c/workspace/RzOps || exit 1
git add -A
git commit -q -m 'fix: 修复 8 个 UI 测试问题（编辑页枚举回显丢失/筛选失效/端口多选/徽章映射/详情补环境/审计资源列/权限控制/审计菜单）'
git log --oneline -2
