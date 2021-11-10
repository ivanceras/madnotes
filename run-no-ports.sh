#!/bin/sh

. ./build-for-desktop.sh &&\


cargo run --release --features="desktop-app"

