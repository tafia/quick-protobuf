cd ../../codegen

for f in ../tests/rust_protobuf/v2/*.proto; do
    cargo run $f
done
