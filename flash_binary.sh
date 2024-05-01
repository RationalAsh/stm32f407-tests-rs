#!/bin/sh

# Get the first argument and if it does not exist set it to the default.
# This is the name of the binary to compile.
BINARY_NAME=${1:-blinky}

# Echo the command to the console
set -x

#echo $BINFILE
#st-flash --connect-under-reset write firmware/${BINARY_NAME}.bin 0x8000000
cp firmware/${BINARY_NAME}.bin /Volumes/DIS_F407VG
