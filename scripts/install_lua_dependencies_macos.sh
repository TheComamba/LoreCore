#!/bin/bash
set -e

version=5.1
brew update
brew install lua@$version luarocks

luarocks install --lua-dir=/usr/local/opt/lua@5.1 --server=https://luarocks.org/dev luaffi

echo Checking installation...
lua ffitest.lua
