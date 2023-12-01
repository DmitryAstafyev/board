#!/bin/bash
set -e

cd ../lib
sh ./build.sh
cd ../sandbox
rm -rf ./node_modules
rm ./build/*.wasm || true
yarn install
yarn run build-ts
yarn run build
yarn run serve