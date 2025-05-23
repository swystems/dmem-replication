# VM configurations

List of some QEMU VM configurations to copy-paste.

```sh
# Host CXL memory and local RAM
# CXL memory shared via ivshmem as file (no NUMA)
sudo qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 8 \
--enable-kvm \
-object memory-backend-file,size=16G,share=on,mem-path=/dev/shm/ivshmem,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-device ivshmem-plain,memdev=cxl-mem \
-object memory-backend-ram,size=16G,host-nodes=0,policy=bind,prealloc=on,id=local-mem \
-numa node,nodeid=0,memdev=local-mem \
-m 16G \
-drive if=virtio,file=cxlvm1_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:01 \
-net user,hostfwd=tcp::2222-:22 \
-nographic


# Host CXL memory and local memory on 2 NUMA guest nodes, 16GB + 16GB
sudo qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 8 \
--enable-kvm \
-object memory-backend-ram,size=16G,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-object memory-backend-ram,size=16G,host-nodes=0,policy=bind,prealloc=on,id=local-mem \
-numa node,nodeid=0,cpus=0-3,memdev=cxl-mem \
-numa node,nodeid=1,cpus=4-7,memdev=local-mem \
-m 32G,slots=2,maxmem=64G \
-drive if=virtio,file=cxlvm1_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:01 \
-net user,hostfwd=tcp::2222-:22 \
-nographic

# Host CXL memory and local memory on 2 NUMA guest nodes, 8GB + 8GB
sudo qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 8 \
--enable-kvm \
-object memory-backend-ram,size=8G,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-object memory-backend-ram,size=8G,host-nodes=0,policy=bind,prealloc=on,id=local-mem \
-numa node,nodeid=0,cpus=4-7,memdev=cxl-mem \
-numa node,nodeid=1,cpus=0-3,memdev=local-mem \
-m 16G,slots=2,maxmem=32G \
-drive if=virtio,file=cxlvm1_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:01 \
-net user,hostfwd=tcp::2222-:22 \
-nographic

# Host CXL memory on guest local node
sudo qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 50 \
--enable-kvm \
-object memory-backend-ram,size=16G,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-m 16G,slots=1,maxmem=32G \
-numa node,nodeid=0,memdev=cxl-mem \
-drive if=virtio,file=cxlvm2_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:02 \
-net user,hostfwd=tcp::2223-:22 \
-nographic

# Local, remote and CXL memory as NUMA (same config as host)
sudo qemu-system-x86_64 \
-machine q35 \
-cpu host \
-smp 33 \
--enable-kvm \
-object memory-backend-ram,size=16G,host-nodes=0,policy=bind,prealloc=on,id=local-mem \
-object memory-backend-ram,size=16G,host-nodes=1,policy=bind,prealloc=on,id=remote-mem \
-object memory-backend-ram,size=16G,host-nodes=2,policy=bind,prealloc=on,id=cxl-mem \
-m 48G,slots=3,maxmem=64G \
-numa node,nodeid=0,memdev=local-mem \
-numa node,nodeid=1,memdev=remote-mem \
-numa node,nodeid=2,memdev=cxl-mem \
-drive if=virtio,file=cxlvm1_disk.qcow2,cache=none \
-net nic,macaddr=52:54:00:12:34:02 \
-net user,hostfwd=tcp::2222-:22 \
-nographic
```