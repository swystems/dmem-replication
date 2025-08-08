use round_test::*;
use libc::{mmap,munmap,PROT_READ,PROT_WRITE,MAP_SHARED};
use std::fs::{OpenOptions};
use std::os::unix::io::AsRawFd;
use clap::{Arg,value_parser};

fn main() {
     let matches = clap::Command::new("round tester").version("1.0")
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
            Arg::new("ratio")
            .short('r')
            .long("ratio")
            .help("Sleep / busy poll ratio")
            .default_value("1")
            .default_value("1000")
            .value_parser(value_parser!(f32))
        )
        .arg(
            Arg::new("attempts")
            .short('a')
            .long("attempts")
            .help("Number of tests")
            .default_value("1000")
            .value_parser(value_parser!(u32))
        )
        .arg(
            Arg::new("local")
            .short('l')
            .long("local")
            .help("read from local memory instead of shared memory")
            .default_value("false")
            .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Parse the command line arguments
    let round_time = matches.get_one::<u64>("duration").unwrap().clone();
    let ratio = matches.get_one::<f32>("ratio").unwrap().clone() as f32;
    // let ratio = ratio.parse::<f32>().expect("String not parsable");
    let attempts = matches.get_one::<u32>("attempts").unwrap().clone();
    let local = matches.get_one::<bool>("local").unwrap().clone();

    let mut mem_latencies = Vec::new();
    let mut timer_latencies = Vec::new();
    if ratio < 0.0 || ratio > 1.0 {
        panic!("Ratio must be between 0.0 and 1.0");
    }

    let path = "/sys/bus/pci/devices/0000:00:03.0/resource2";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Failed to open shared memory");


    let size: usize = 64 ; 

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
        panic!("mmap failed: {}", std::io::Error::last_os_error());
    }

    let ptr = ptr as *mut u8;

    unsafe {
        // Initialize the shared memory region to zero
        std::ptr::write_bytes(ptr, 0, size);
    }
    
    unsafe {
        std::ptr::read_volatile(ptr);
    }

    let local_mem = vec![0u8; size];
    for _ in 0..attempts{
        let start = std::time::Instant::now();    
        // read from local memory
        if local {
          let _ = local_mem[0];
        }          
        // read from shared memory
        else {
            unsafe {
               std::ptr::read_volatile(ptr);
            }
        }
        let mem_elapsed = start.elapsed().as_nanos() as u64;

        // sleep or busy poll until next round
        if round_time > mem_elapsed  {
            let time_left = round_time - mem_elapsed;
            let threshold = (time_left as f32 * (1.0 - ratio)) as u64;
            busy_poll_sleep(time_left, threshold);
        }

        let end = start.elapsed().as_nanos() as u64;
        mem_latencies.push(mem_elapsed);
        // println!("mem lat {:?}ns, total elapsed {:?}ns", mem_elapsed, end);
        timer_latencies.push(end - mem_elapsed);
    }
    
    unsafe {
        munmap(ptr as *mut libc::c_void, size);
    }

    let mem_avg = mem_latencies.iter().sum::<u64>() as f64 / attempts as f64;
    let mem_max = mem_latencies.iter().max().unwrap();
    let mem_min = mem_latencies.iter().min().unwrap();
    let timer_avg = timer_latencies.iter().sum::<u64>() as f64 / attempts as f64; 
    let timer_max = timer_latencies.iter().max().unwrap();
    let timer_min = timer_latencies.iter().min().unwrap();
    println!("Desired sleep duration of {round_time} nanoseconds: {attempts} runs");
    println!("Memory Avg: {:?}", mem_avg);
    println!("Memory Max: {:?}", mem_max);
    println!("Memory Min: {:?}", mem_min);
    println!("Memory 50th percentile: {:?}", percentile(&mem_latencies, 0.5));
    println!("Memory 95th percentile: {:?}", percentile(&mem_latencies, 0.95));
    println!("Memory 99th percentile: {:?}", percentile(&mem_latencies, 0.99)); 
    println!("Memory 99.99th percentile: {:?}", percentile(&mem_latencies, 0.9999));
    println!("jitter: {:?}us", (mem_max - mem_min) as f64/1000.0);
    println!("timer avg: {:?}", timer_avg);
    println!("timer max: {:?}", timer_max);
    println!("timer min: {:?}", timer_min);
    println!("timer 50th percentile: {:?}", percentile(&timer_latencies, 0.5));
    println!("timer 95th percentile: {:?}", percentile(&timer_latencies, 0.95));
    println!("timer 99th percentile: {:?}", percentile(&timer_latencies, 0.99));
    println!("timer 99.99th percentile: {:?}", percentile(&timer_latencies, 0.9999));
    println!("jitter: {:?}us", (timer_max - timer_min) as f64/1000.0);
}
