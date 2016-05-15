#!/bin/bash

cd "$(dirname "${BASH_SOURCE[0]}")"
dir="$(pwd)"

export OPENSSL_PATH="${dir}/openssl-arm-bin"
export OPENSSL_LIB_DIR=${OPENSSL_PATH}/lib
export OPENSSL_INCLUDE_DIR=${OPENSSL_PATH}/include

cargo build --target=armv7-unknown-linux-gnueabihf
