#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

wasm-pack build ./webapp --dev --target web --out-name main
wasm-pack build ./plugin --dev --target web --out-name plugin

cp ./webapp/pkg/* ./app
cp ./plugin/pkg/* ./app

sed -i '/init.__wbindgen_wasm_module = module\;/a main(imports)\;' ./app/main.js
