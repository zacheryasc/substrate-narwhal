#!/usr/bin/bash

sudo apt update
sudo apt install --assume-yes build-essential
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $(pwd)/rustup.sh
sh $(pwd)/rustup.sh -y
rm $(pwd)/rustup.sh
source $HOME/.cargo/env

rustup default nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
