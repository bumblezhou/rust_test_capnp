# A simple app demo how to use capnp in rust

## Prepare
```bash
sudo apt update && sudo apt install capnproto -y
capnpc --version
capnp --version
```

## How to build
```bash
export OUT_DIR=$(pwd)
cargo build
cargo run server localhost:2000 
cargo run client localhost:2000 hello
```

## Reference:
1. https://medium.com/@learnwithshobhit/comparing-capn-proto-and-grpc-in-rust-a-performance-and-feature-analysis-61d2da815d18
2. https://forge.rust-lang.org/infra/other-installation-methods.html#rustup
3. https://capnproto.org/install.html