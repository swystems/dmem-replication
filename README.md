# dmem-replication
Disaggregated memory experiments. swystems &lt;> IBM research 


## QEMU VM management

### create Ubuntu 24.04 VM using CXL memory

Create a disk (at least 20G, will leave 7G available to use with Ubuntu 24.04):

```sh
qemu-img create -f qcow2 cxlvm1_disk.qcow2 20G
```

Get Ubuntu 24.04 server

```sh
wget https://releases.ubuntu.com/noble/ubuntu-24.04.2-live-server-amd64.iso
```

Create VM. The following settings map the CXL host memory (`host-nodes=2` ->
NUMA node 2 where CXL device is attached to) as normal memory device to the VM.

```sh
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
-net user,hostfwd=tcp::2222-:22 \
-cdrom ubuntu-24.04.2-live-server-amd64.iso
```

In order to see the serial output during Ubuntu installation the `console=ttyS0`
kernel boot option might be needed. Type "e" on the "Try to install Ubuntu" option
in GRUB -> add `console=ttyS0 after "..vmlinuz" word.

### Start a VM

QEMU does is stateless: all emulation option (CPU, mem, NICs) are set at startup.
Only the disk info is retained (OS, files etc.). Start a VM with the same command
as create but without `-cdrom ...`

### Clone / multiple VMs

Copy-paste disk with OS, start VMs with different network interfaces and SSH 
host fowarding ports.


