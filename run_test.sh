#!/usr/bin/env bash

set -e

./generate_modules.sh

cargo +stable run -p quick-protobuf --example pb_rs_example_v3_owned
cargo +stable run -p quick-protobuf --example pb_rs_example
cargo +stable run -p quick-protobuf --example pb_rs_example_v3

cargo +stable test -p pb-rs -p quick-protobuf
cargo +stable fmt
