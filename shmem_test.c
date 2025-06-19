/**
* @brief Inter-VM communication test using shared memory
* @note This code is intended to be run in two different VMs (VM1 and VM2)
*/
#include <stdio.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <string.h>

int main() {
    // ivshmem host memory device found with lspci. Resource2 = BAR2, the shared 
    // memory region
    int fd = open("/sys/bus/pci/devices/0000:00:03.0/resource2", O_RDWR);
    // mmap to 16 MB
    char *shmem = mmap(NULL, 16*1024*1024, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
    if (shmem == MAP_FAILED) {
        perror("mmap failed");
        return 1;
    }

    // Write data from VM1
    const char *msg = "Hello from VM1!";
    memcpy(shmem, msg, strlen(msg) + 1);
  
    // Read data in VM2
    char buffer[32];
    strncpy(buffer, shmem, sizeof(buffer)); 
    printf("Received message: %s\n", shmem);

    munmap(shmem, 16*1024*1024);
    close(fd);
    return 0;
}
