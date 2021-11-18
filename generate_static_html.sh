#!/bin/sh


. ./build.sh &&\

## This is needed in order to have a consistent result on the generated css from the page.rs module
cargo run --bin generate_html --features "open-ports" > client/index.html


