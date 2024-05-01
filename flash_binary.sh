#!/bin/sh

# Echo the command to the console
set -x

BINFILE="firmware.bin"
#echo $BINFILE
st-flash --connect-under-reset write ${BINFILE} 0x8000000
