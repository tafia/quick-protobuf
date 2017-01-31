rustup install nightly
cd ../../codegen
cargo run ../benches/unit_bench/perftest_data.proto
cd ../benches/unit_bench
rustup run nightly cargo test
