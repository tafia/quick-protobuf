#!/usr/bin/env bash

set -e

./generate_modules.sh

cargo run -p quick-protobuf --example pb_rs_example_v3_owned
cargo run -p quick-protobuf --example pb_rs_example
cargo run -p quick-protobuf --example pb_rs_example_v3
cargo run --no-default-features -p quick-protobuf --example pb_rs_example_nostd

cargo test -p pb-rs -p quick-protobuf
