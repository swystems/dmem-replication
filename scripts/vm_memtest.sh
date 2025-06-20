#!/bin/bash
# VM with 3 numa nodes corresponding to the 3 memories in the servers
# used for memory latency tests

qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 65 \
--enable-kvm \
-object memory-backend-ram,size=16G,host-nodes=0,policy=bind,prealloc=on,id=local-mem \
-object memory-backend-ram,size=16G,host-nodes=1,policy=bind,prealloc=on,id=remote-mem \
-object memory-backend-ram,size=16G,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-m 48G,slots=3,maxmem=64G \
-numa node,nodeid=0,memdev=local-mem \
-numa node,nodeid=1,memdev=remote-mem \
-numa node,nodeid=2,memdev=cxl-mem \
-drive if=virtio,file=vms/ubuntu2204.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:03 \
-net user,hostfwd=tcp::2223-:22 \
-daemonize \
-display none
