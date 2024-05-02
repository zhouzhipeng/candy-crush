#!/bin/bash

cd rust
cargo build --release

cp target/debug/*.dylib ../godot/extensions/librust_release.dylib
cp target/debug/*.so ../godot/extensions/librust_release.so
cp target/debug/*.dll ../godot/extensions/librust_release.dll