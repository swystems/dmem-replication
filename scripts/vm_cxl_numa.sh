#!/bin/bash

qemu-system-x86_64 \
    -machine q35 \ 
    -cpu host \
    -smp 8 \ 
    -accel kvm \
    -enable-kvm \
    -m 16G,slots=1,maxmem=32G \
    -object memory-backend-ram,id=local-mem0,size=16G,host-nodes=0,policy=bind,prealloc=on \
    -object memory-backend-ram,id=cxl-mem,size=16G,host-nodes=2,policy=bind,prealloc=on \
    -numa node,nodeid=0,memdev=local-mem0 \
    -numa node,nodeid=1,memdev=cxl-mem \