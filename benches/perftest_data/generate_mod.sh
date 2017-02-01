cd ../../codegen
cargo run ../benches/perftest_data/perftest_data.proto
cd ../benches/perftest_data
mv perftest_data.rs mod.rs
