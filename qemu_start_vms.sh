#!/bin/bash
# QEMU VM with CXL memory attached as default and only memory.
# assumes VMs were created on cxlvm{x}_disk.qcow2
# Assmues CXL memory on the host is on NUMA node 2

# VM1
sudo qemu-system-x86_64 \
-nographic \
-machine q35 \
-cpu host \
-smp 8 \
--enable-kvm \
-object memory-backend-ram,id=cxl-mem,size=16G,host-nodes=2,policy=bind,prealloc=on \
-m 16G,slots=1,maxmem=32G \
-drive if=virtio,file=cxlvm1_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:01 \
-net user,hostfwd=tcp::2222-:22 &

# VM2
sudo qemu-system-x86_64 \
-nographic \
-machine q35 \
-cpu host \
-smp 8 \
--enable-kvm \
-object memory-backend-ram,id=cxl-mem,size=16G,host-nodes=2,policy=bind,prealloc=on \
-m 16G,slots=1,maxmem=32G \
-drive if=virtio,file=cxlvm2_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:02 \
-net user,hostfwd=tcp::2223-:22 