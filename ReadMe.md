# A simple app demo how to use capnp in rust

## Prepare
```bash
sudo apt update && sudo apt install capnproto -y
capnpc --version
capnp --version
```

## How to build
```bash
export OUT_DIR=~/Desktop/Tmp/rust_test_capnp
cargo build
cargo run server localhost:2000 
cargo run client localhost:2000 hello
```

## Reference:
1. https://medium.com/@learnwithshobhit/comparing-capn-proto-and-grpc-in-rust-a-performance-and-feature-analysis-61d2da815d18