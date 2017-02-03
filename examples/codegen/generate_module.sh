cd ../../codegen
cargo run ../examples/codegen/data_types_import.proto
cargo run ../examples/codegen/data_types.proto
cd ../examples/codegen
cargo run --example codegen_example
