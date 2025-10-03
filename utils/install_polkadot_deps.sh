#!/bin/bash
set -e

# Atualiza os pacotes e instala dependências essenciais
sudo apt update
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler



# Instala utilitários do Rust necessários para compilar o Polkadot
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src


# Clona o repositório do Polkadot
# if [ ! -d "polkadot" ]; then
#     # git clone https://github.com/paritytech/polkadot.git
# fi
rustup show

echo "Dependências do Polkadot instaladas com sucesso."