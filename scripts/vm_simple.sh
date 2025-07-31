#!/bin/bash
# VM with 3 numa nodes corresponding to the 3 memories in the servers
# used for memory latency tests

qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 24 \
--enable-kvm \
-m 48G \
-drive if=virtio,file=vms/ubuntu2404rt.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:01 \
-net user,hostfwd=tcp::2222-:22 \
-daemonize \
-display none
