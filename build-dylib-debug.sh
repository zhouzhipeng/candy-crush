#!/bin/bash

cd rust
cargo build

cp target/debug/*.dylib ../godot/extensions/librust_debug.dylib
cp target/debug/*.so ../godot/extensions/librust_debug.so
cp target/debug/*.dll ../godot/extensions/librust_debug.dll