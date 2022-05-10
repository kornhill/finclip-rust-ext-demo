#!/usr/bin/env bash
# building
cbindgen src/lib.rs -l c > myplugin.h
cargo lipo --release 

