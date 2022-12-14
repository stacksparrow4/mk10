#!/bin/bash

wasm-pack build --release
cd www
npm run build
cd ..

rm -rf ghpages
mkdir ghpages

cp www/dist/* ghpages/

echo "Success!"
