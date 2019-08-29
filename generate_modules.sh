#!/bin/env bash

set -e

(
    cd quick-protobuf/tests/rust_protobuf
    ./generate.sh
)

base_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

proto_sets=(
    quick-protobuf/tests/rust_protobuf/common/*.proto
    quick-protobuf/tests/packed_primitives/*.proto
    quick-protobuf/examples/pb_rs_v3/*.proto
    quick-protobuf/examples/pb_rs/*.proto
    perftest/src/*.proto
)

for ps in "${proto_sets[@]}"; do
    for proto in $ps; do
        (
            cd pb-rs
            cargo run "${base_dir}"/$proto
        )
    done
done


for proto in quick-protobuf/examples/pb_rs_v3/*.proto; do
    (
        cd pb-rs
        cargo run "${base_dir}"/"${proto}" \
              --owned \
              --output_directory "${base_dir}"/quick-protobuf/examples/pb_rs_v3/owned
    )
done


# cd ../examples/codegen
# cargo run --example codegen_example
