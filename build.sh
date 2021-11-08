#!/bin/sh

RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build client --target web --release
