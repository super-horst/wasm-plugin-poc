#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

wasm-pack build ./webapp --dev --target web --out-name wasm --out-dir "$(pwd)/app"
