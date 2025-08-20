use repCXL::RepCXL;
use std::fs::OpenOptions;

const MEMORY_SIZE: usize = 1024 * 1024; // 1 MiB
const CHUNK_SIZE: usize = 64; // 64 bytes
const NODES: usize = 3;
fn main() {
    // create memory nodes as files in tmpfs
    for i in 0..NODES {
        let path = format!("/dev/shm/repCXL_test{}", i);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .expect("Failed to create/open file in tmpfs");

        file.set_len(MEMORY_SIZE as u64)
            .expect("Failed to set file length");
    }

    let mut rcxl = RepCXL::new(MEMORY_SIZE, CHUNK_SIZE);

    println!("mem: {}", rcxl.size);
    for i in 0..NODES {
        rcxl.add_memory_node_from_file(&format!("/dev/shm/repCXL_test{}", i));
    }

    rcxl.init_state();

    rcxl.new_object::<[u16; 100]>(100)
        .expect("failed to create object");

    rcxl.new_object::<String>(100);

    rcxl.new_object::<String>(66);
    rcxl.new_object::<String>(67);
    // rcxl.remove_object::<String>(66);
    // rcxl.remove_object::<String>(100);

    rcxl.dump_states();
}
