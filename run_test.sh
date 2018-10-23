#!/bin/bash

cargo run -p pb-rs quick-protobuf/examples/pb_rs/data_types_import.proto
cargo run -p pb-rs quick-protobuf/examples/pb_rs/data_types.proto
cargo run -p pb-rs quick-protobuf/examples/pb_rs_v3/data_types_import.proto
cargo run -p pb-rs quick-protobuf/examples/pb_rs_v3/data_types.proto
cargo run -p pb-rs quick-protobuf/tests/packed_primitives/person.proto
cargo run -p pb-rs quick-protobuf/benches/perftest_data/perftest_data.proto
cargo run -p quick-protobuf --example pb_rs_example
cargo run -p quick-protobuf --example pb_rs_example_v3

cd quick-protobuf/tests/rust_protobuf
./generate.sh
