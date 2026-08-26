#!/bin/bash
echo "1" | sudo -S ln -sf /mnt/c/tools/nodejs/node.exe /usr/local/bin/node 2>&1
echo "1" | sudo -S ln -sf /mnt/c/tools/nodejs/npm /usr/local/bin/npm 2>&1
ls -la /usr/local/bin/node /usr/local/bin/npm
node --version 2>&1
npm --version 2>&1 | head -3
