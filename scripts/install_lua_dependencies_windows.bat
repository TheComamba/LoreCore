choco install -y lua
choco install -y luarocks

luarocks install --server=https://luarocks.org/dev luaffi

echo Checking installation...
lua ffitest.lua
