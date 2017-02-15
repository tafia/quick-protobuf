cargo run ../examples/codegen/data_types_import.proto
cargo run ../examples/codegen/data_types.proto
cd ..
cargo run --example codegen_example

cd tests/rust_protobuf
sh generate.sh
