#!/usr/bin/env bash

set -e

./generate_modules.sh

pushd quick-protobuf
cargo run --example pb_rs_example_v3_owned
cargo run --example pb_rs_example
cargo run --example pb_rs_example_v3
cargo run --example pb_rs_example_nostd --no-default-features
popd

cargo test -p pb-rs -p quick-protobuf
