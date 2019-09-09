#!/usr/bin/env bash

set -eu -o pipefail

(
    cd quick-protobuf/tests/rust_protobuf
    ./generate.sh
)

base_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

proto_sets=(
    perftest/src/*.proto
    quick-protobuf/benches/perftest_data/*.proto
    quick-protobuf/examples/pb_rs/*.proto
    quick-protobuf/examples/pb_rs_v3/*.proto
    quick-protobuf/tests/packed_primitives/*.proto
    quick-protobuf/tests/rust_protobuf/common/*.proto
)

for ps in "${proto_sets[@]}"; do
    for proto in $ps; do
        cargo +stable run -p pb-rs "${base_dir}"/"${proto}"
    done
done


rm -rf quick-protobuf/examples/pb_rs_v3/owned
mkdir -p quick-protobuf/examples/pb_rs_v3/owned
for proto in quick-protobuf/examples/pb_rs_v3/*.proto; do
    (
        cargo +stable run -p pb-rs "${base_dir}"/"${proto}" \
              --owned \
              --output_directory "${base_dir}"/quick-protobuf/examples/pb_rs_v3/owned
    )
done


# cd ../examples/codegen
# cargo run --example codegen_example
