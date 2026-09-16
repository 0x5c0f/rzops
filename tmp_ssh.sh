#!/bin/bash
cd /mnt/c/workspace/RzOps
echo '--- remote ---'
git remote -v
echo '--- ssh key ---'
ls ~/.ssh/ 2>/dev/null || echo 'no ~/.ssh'
echo "SSH_AUTH_SOCK=${SSH_AUTH_SOCK:-empty}"
echo '--- github auth ---'
ssh -T git@github.com 2>&1 | head -3
