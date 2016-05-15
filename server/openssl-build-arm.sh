#!/bin/bash

dir="$(dirname "${BASH_SOURCE[0]}")"
dir="$(cd "${dir}" && pwd)"

export SRCDIR="${dir}/openssl"
export INSTALLDIR="${dir}/openssl-arm-bin"
export TARGETMACH=arm-unknown-linux-gnueabihf
export CROSS=arm-linux-gnueabihf
export CC=${CROSS}-gcc
export LD=${CROSS}-ld
export AS=${CROSS}-as
export AR=${CROSS}-ar

cd ${SRCDIR}
./Configure --openssldir=${INSTALLDIR} shared os/compiler:${TARETMACH}-
#make depend
make
make install
