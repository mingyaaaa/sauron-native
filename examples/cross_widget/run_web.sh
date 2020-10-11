#!/bin/bash

set -v
if wasm-pack build --target no-modules -- --features "with-web sauron-native/with-debug"; then
    basic-http-server ./ -a 0.0.0.0:7000
fi
