cd ../../codegen
for f in ../examples/codegen/*.proto; do
    cargo run $f
done
cd ../examples/codegen
cargo run --example codegen_example
