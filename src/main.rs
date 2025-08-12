use clap::{value_parser, Arg};
use repCXL::

fn main() {
    let matches = clap::Command::new("round tester")
        .version("1.0")
        .about("Test the round time (interval) accuracy of the system")
        .arg(
            Arg::new("duration")
                .short('d')
                .long("duration")
                .help("Duration of the round")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("attempts")
                .short('a')
                .long("attempts")
                .help("Number of tests")
                .default_value("1000")
                .value_parser(value_parser!(u32)),
        )
        .get_matches();

    // Parse the command line arguments
    let round_time = matches.get_one::<u64>("duration").unwrap().clone();
    // let ratio = ratio.parse::<f32>().expect("String not parsable");
    let attempts = matches.get_one::<u32>("attempts").unwrap().clone();

    let mut mem_latencies = Vec<u64::new();
    let mut timer_latencies = Vec<u64>::new();

    // let mn1 = MemoryNode::from_file(
    //     1,
    //     "/dev/shm/repCXL_test",
    //     1024 * 1024, // 1 MiB
    // );

    // for _ in 0..attempts {
    //     let start = std::time::Instant::now();

    //     // read from shared memory
    //     unsafe {
    //         std::ptr::read_volatile(ptr);
    //     }
    //     let mem_elapsed = start.elapsed().as_nanos() as u64;

    //     // sleep or busy poll until next round
    //     // if round_time > mem_elapsed {
    //     //     let time_left = round_time - mem_elapsed;
    //     //     let threshold = (time_left as f32 * (1.0 - ratio)) as u64;
    //     //     busy_poll_sleep(time_left, threshold);
    //     // }

    //     let end = start.elapsed().as_nanos() as u64;
    //     mem_latencies.push(mem_elapsed);
    //     // println!("mem lat {:?}ns, total elapsed {:?}ns", mem_elapsed, end);
    //     timer_latencies.push(end - mem_elapsed);
    // }
}
