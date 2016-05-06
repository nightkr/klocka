#!/bin/bash
export OPENSSL_PATH=/home/teo/opt/openssl-arm-bin
export OPENSSL_LIB_DIR=$OPENSSL_PATH/lib
export OPENSSL_INCLUDE_DIR=$OPENSSL_PATH/include

cargo build --target=armv7-unknown-linux-gnueabihf
