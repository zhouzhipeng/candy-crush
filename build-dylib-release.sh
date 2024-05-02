#!/bin/bash

cd rust
cargo build --release

cp target/release/*.dylib ../godot/extensions/librust_release.dylib
cp target/release/*.so ../godot/extensions/librust_release.so
cp target/release/*.dll ../godot/extensions/librust_release.dll