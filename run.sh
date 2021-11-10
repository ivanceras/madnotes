#!/bin/sh

. ./build-for-desktop.sh &&\


cargo run --release --no-default-features --features="desktop-app open-ports"

