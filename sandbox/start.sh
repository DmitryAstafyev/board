cd ../lib
sh ./build.sh
cd ../sandbox
rm -rf ./node_modules
yarn install
yarn run build
yarn run serve