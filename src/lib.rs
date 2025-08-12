use libc::{c_int, c_void, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

#[link(name = "numa")]
extern "C" {
    pub fn numa_alloc_onnode(size: usize, node: c_int) -> *mut c_void;
    pub fn numa_free(mem: *mut c_void, size: usize);
}

#[derive(PartialEq, Debug)]
enum MemoryType {
    Numa,
    File,
}

struct MemoryNode {
    id: u16,
    type_: MemoryType,
    addr: *mut u8,
    size: usize,
}

impl MemoryNode {
    fn from_file(id: u16, path: &str, size: usize) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .expect("Failed to open shared memory");

        let ptr = unsafe {
            mmap(
                std::ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            panic!(
                "Failed to mmap {}. Error: {}",
                path,
                std::io::Error::last_os_error()
            );
        }

        let ptr = ptr as *mut u8;

        unsafe {
            // Initialize the shared memory region to zero
            std::ptr::write_bytes(ptr, 0, size);
        }

        MemoryNode {
            id,
            type_: MemoryType::File,
            addr: ptr,
            size,
        }
    }

    pub fn from_numa(id: u16, size: usize, numa_node: i32) -> Self {
        let ptr = unsafe { numa_alloc_onnode(size, numa_node) };
        if ptr.is_null() {
            panic!("numa_alloc_onnode failed");
        }
        let ptr = ptr as *mut u8;

        MemoryNode {
            id,
            type_: MemoryType::Numa,
            addr: ptr,
            size,
        }
    }
}

impl Drop for MemoryNode {
    fn drop(&mut self) {
        if self.type_ == MemoryType::Numa {
            unsafe {
                numa_free(self.addr as *mut c_void, self.size);
            }
        } else if self.type_ == MemoryType::File {
            unsafe {
                munmap(self.addr as *mut libc::c_void, self.size);
            }
            // File is automatically closed when it goes out of scope
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_memory_node_from_file() {
        let mnid = 1;
        let path = "/dev/shm/repCXL_test";
        let size = 1024; // 1 KiB

        // Create and open the file with read/write permissions
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to create/open file in tmpfs");

        // Resize the file to 4096 bytes (one page)
        file.set_len(4096).expect("Failed to set file length");

        let node = MemoryNode::from_file(mnid, path, size);
        assert_eq!(node.id, mnid);
        assert_eq!(node.type_, MemoryType::File);
        assert!(!node.addr.is_null());
        assert_eq!(node.size, size); // 1 KiB

        // Clean up: remove the tmpfs file
        remove_file(path).expect("Failed to remove tmpfs file");
    }

    #[test]
    fn test_memory_node_from_numa() {
        let mnid = 0;
        let size = 1024; // 1 KiB
        let numa_node = 0; // Node 0 should exist on most systems

        let node = MemoryNode::from_numa(mnid, size, numa_node);

        unsafe {
            *node.addr = 31;
            // Initialize the shared memory region to zero
            std::ptr::write_bytes(node.addr, 4, size);
        }

        assert_eq!(node.id, mnid);
        assert_eq!(node.type_, MemoryType::Numa);
        assert!(!node.addr.is_null());

        assert_eq!(node.size, size); // 1 KiB
    }
}

pub struct RepCXL {
    memory_nodes: Vec<MemoryNode>,
}

impl RepCXL {
    pub fn new() -> Self {
        RepCXL {
            memory_nodes: Vec::new(),
        }
    }

    // pub fn add_memory_node(&self, )
}
