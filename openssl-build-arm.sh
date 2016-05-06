#!/bin/bash

export INSTALLDIR=/home/teo/opt/openssl-arm-bin
export TARGETMACH=arm-unknown-linux-gnueabihf
export CROSS=arm-linux-gnueabihf
export CC=${CROSS}-gcc
export LD=${CROSS}-ld
export AS=${CROSS}-as
export AR=${CROSS}-ar

./Configure --openssldir=${INSTALLDIR} shared os/compiler:arm-unknown-linux-gnueabihf-
#make depend
make
make install
