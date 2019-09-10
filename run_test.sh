#!/usr/bin/env bash

set -e

./generate_modules.sh

cargo run -p quick-protobuf --example pb_rs_example_v3_owned
cargo run -p quick-protobuf --example pb_rs_example
cargo run -p quick-protobuf --example pb_rs_example_v3

# test that no_std can build
pushd quick-protobuf/no-std-example
  cargo build
popd

cargo test -p pb-rs -p quick-protobuf
