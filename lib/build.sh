cd ../core
wasm-pack build --target bundler
cd ../lib
rm -rf ./build
rm -rf ./node_modules
yarn install
yarn run build-ts
