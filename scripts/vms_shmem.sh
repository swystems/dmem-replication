#!/bin/bash
# QEMU VM with CXL memory attached as default and only memory.
# assumes VMs were created on cxlvm{x}_disk.qcow2
# Assmues CXL memory on the host is on NUMA node 2
# start in background

truncate -s 16G /dev/shm/ivshmem
chmod 666 /dev/shm/ivshmem

COMMON_SETTINGS="-machine q35 \
-cpu host \
-smp 8 \
--enable-kvm \
-object memory-backend-ram,size=16G,host-nodes=0,policy=bind,prealloc=on,id=local-mem \
-object memory-backend-file,size=16G,share=on,mem-path=/dev/shm/ivshmem,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-device ivshmem-plain,memdev=cxl-mem \
-m 32G,slots=2,maxmem=64G \
-daemonize \
-display none"

# VM1
sudo qemu-system-x86_64 \
-drive if=virtio,file=cxlvm1_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:01 \
-net user,hostfwd=tcp::2222-:22 \
${COMMON_SETTINGS} &

# VM2
sudo qemu-system-x86_64 \
-drive if=virtio,file=cxlvm2_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:02 \
-net user,hostfwd=tcp::2223-:22 \
${COMMON_SETTINGS}
