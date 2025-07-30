#!/bin/bash

sh scripts/build.sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-mirx/debug/bootimage-mirx.bin 