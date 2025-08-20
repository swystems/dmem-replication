use repCXL::RepCXL;

fn main() {
    let mut rcxl = RepCXL::new(1024 * 1024, 64);
    rcxl.add_memory_node_from_file("/dev/shm/repCXL_test1");
    rcxl.add_memory_node_from_file("/dev/shm/repCXL_test2");
    rcxl.add_memory_node_from_file("/dev/shm/repCXL_test3");

    rcxl.dump_states();
}
