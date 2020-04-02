#/bin/sh
rm -rf ./www/dist
set -e
wasm-pack build --release
cd www/
npm run build-production
cd ..