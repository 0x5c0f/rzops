#!/bin/bash
echo "--- test node.exe from WSL ---"
/mnt/c/tools/nodejs/node.exe --version 2>&1 | head -2
echo "--- test npm script ---"
head -1 /mnt/c/tools/nodejs/npm 2>&1
