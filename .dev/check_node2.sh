#!/bin/bash
echo "which node: $(which node 2>&1)"
node --version 2>&1 | head -2
echo "--- WSL native node ---"
ls -la /usr/bin/node* /usr/local/bin/node* /usr/bin/npm* 2>/dev/null
echo "--- PATH ---"
echo "$PATH"
