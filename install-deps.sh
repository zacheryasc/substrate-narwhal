#!/usr/bin/bash

sudo apt update
sudo apt install --assume-yes build-essential
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# docker
sudo apt-get update
sudo apt-get install --assume-yes \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

sudo mkdir -m 0755 -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt-get update
sudo apt-get install --assume-yes docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

sudo groupadd docker
sudo usermod -aG docker $USER
newgrp docker
docker run hello-world

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $(pwd)/rustup.sh
sh $(pwd)/rustup.sh -y
rm $(pwd)/rustup.sh
source $HOME/.cargo/env

rustup default nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
