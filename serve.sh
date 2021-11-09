#!/bin/sh

. ./build.sh &&\


cargo run --bin server --release --no-default-features --features="open-ports"

