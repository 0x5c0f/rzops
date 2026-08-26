#!/bin/bash
set -e
NODE_VERSION=v22.17.0
cd /home/chenxiaodong
echo "--- downloading node $NODE_VERSION ---"
curl -fsSL "https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-linux-x64.tar.xz" -o /tmp/node.tar.xz
echo "--- extracting ---"
mkdir -p /home/chenxiaodong/node
tar -xJf /tmp/node.tar.xz -C /home/chenxiaodong/node --strip-components=1
echo "--- verify ---"
/home/chenxiaodong/node/bin/node --version
