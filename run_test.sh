#!/bin/bash

cargo run -p pb-rs quick-protobuf/examples/pb_rs/data_types_import.proto
cargo run -p pb-rs quick-protobuf/examples/pb_rs/data_types.proto
cargo run -p quick-protobuf --example pb_rs_example

cd quick-protobuf/tests/rust_protobuf
./generate.sh
