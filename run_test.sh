#!/usr/bin/env bash

set -e

rm -rf quick-protobuf/examples/pb_rs_v3/owned
mkdir -p quick-protobuf/examples/pb_rs_v3/owned 
cargo +stable run -p pb-rs -- --owned -d quick-protobuf/examples/pb_rs_v3/owned quick-protobuf/examples/pb_rs_v3/*.proto
cargo +stable run -p quick-protobuf --example pb_rs_example_v3_owned

cargo +stable run -p pb-rs quick-protobuf/examples/pb_rs/data_types_import.proto
cargo +stable run -p pb-rs quick-protobuf/examples/pb_rs/data_types.proto
cargo +stable run -p pb-rs quick-protobuf/examples/pb_rs_v3/data_types_import.proto
cargo +stable run -p pb-rs quick-protobuf/examples/pb_rs_v3/data_types.proto
cargo +stable run -p pb-rs quick-protobuf/tests/packed_primitives/person.proto
cargo +stable run -p pb-rs quick-protobuf/benches/perftest_data/perftest_data.proto
cargo +stable run -p quick-protobuf --example pb_rs_example
cargo +stable run -p quick-protobuf --example pb_rs_example_v3

pushd quick-protobuf/tests/rust_protobuf
  ./generate.sh
popd
cargo +stable test -p pb-rs -p quick-protobuf
cargo +stable fmt
