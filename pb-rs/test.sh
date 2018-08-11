cd ../tests/rust_protobuf
./generate.sh

cd ../issue69
./generate.sh

cargo test
