#!/usr/bin/env bash

set -eux -o pipefail

(
    cd quick-protobuf/tests/rust_protobuf
    ./generate.sh
)

base_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

rm -r quick-protobuf/examples/pb_rs/*.rs quick-protobuf/examples/pb_rs/a
rm -r quick-protobuf/examples/pb_rs_v3/*.rs quick-protobuf/examples/pb_rs_v3/a

proto_sets=(
    perftest/src
    quick-protobuf/benches/perftest_data
    quick-protobuf/examples/pb_rs
    quick-protobuf/examples/pb_rs_v3
    quick-protobuf/tests/packed_primitives
    quick-protobuf/tests/rust_protobuf/common
)

nostd_proto_sets=(
    quick-protobuf/examples/pb_rs_nostd
)

for ps in "${proto_sets[@]}"; do
  cargo run -p pb-rs -- -I "$ps" -d "$ps" "$ps"/*.proto
done

for ps in "${nostd_proto_sets[@]}"; do
  cargo run -p pb-rs -- --nostd -I "$ps" -d "$ps" "$ps"/*.proto
done

rm -rf quick-protobuf/examples/pb_rs_v3/owned
mkdir -p quick-protobuf/examples/pb_rs_v3/owned
cargo run -p pb-rs quick-protobuf/examples/pb_rs_v3/*.proto \
  -I quick-protobuf/examples/pb_rs_v3 \
  --owned \
  --output_directory quick-protobuf/examples/pb_rs_v3/owned


# cd ../examples/codegen
# cargo run --example codegen_example
