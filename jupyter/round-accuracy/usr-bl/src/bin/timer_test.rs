use round_test::*;
// use std::thread;
use libc::{sched_setscheduler, sched_param, SCHED_FIFO, SCHED_DEADLINE, getpid};


// fn timer_loop(usize sleep_time: u64, threshold: u64, ratio: f32, attempts: usize) {}


fn main() {
    let matches = clap::Command::new("Performance test")
        .version("1.0")
        .about("")
        .arg(clap::arg!(-s --sleep <SLEEP_TIME> "Time to sleep for in usec.").required(true))
        .arg(clap::arg!(-t --threshold <THRESHOLD> "Threshold").default_value("0"))
        .arg(clap::arg!(-r --ratio <RATIO> "Sleep / busy poll ratio").default_value("1"))
        .arg(clap::arg!(-a --attempts <ATTEMPTS> "Number of tests").default_value("1000"))
        .get_matches();

    // Parse the command line arguments
    let sleep_time = matches.get_one::<String>("sleep").unwrap().replace('_', "");
    let sleep_time = sleep_time.parse::<u64>().expect("String not parsable");
    let threshold = matches
        .get_one::<String>("threshold")
        .unwrap()
        .replace('_', "");
    let mut threshold = threshold.parse::<u64>().expect("String not parsable");
    let ratio = matches.get_one::<String>("ratio").unwrap().replace('_', "");
    let ratio = ratio.parse::<f32>().expect("String not parsable");
    let mut durations = Vec::new();
    let attempts = matches
        .get_one::<String>("attempts")
        .unwrap()
        .replace('_', "");
    let attempts = attempts.parse::<usize>().expect("String not parsable");

    if sleep_time < threshold {
        panic!("Sleep time must be greater than threshold");
    }

    if ratio >= 0.0 && ratio < 1.0 {
        // If ratio is set, we use it to calculate the threshold
        // threshold = sleep_time / ratio
        threshold = (sleep_time as f32 * (1.0 - ratio)) as u64;
    }


    // Set the scheduler to SCHED_FIFO
    let mut param = sched_param {
        sched_priority: 99, // Set to a high priority for real-time scheduling
    };
    unsafe {
        if sched_setscheduler(getpid(), SCHED_FIFO, &mut param) != 0 {
            eprintln!("Failed to set SCHED_FIFO: {}", std::io::Error::last_os_error());
            return;
        }
    }

        // println!("ratio: {ratio}, threshold: {threshold}, sleep_time: {sleep_time}");
    for _ in 0..attempts {
        let start = std::time::Instant::now();

        busy_poll_sleep(sleep_time, threshold);
        // let _time = busy_poll_sleep_rdtsc(sleep_time, threshold);
        let end = start.elapsed();
        durations.push(end);
        // println!("Time to sleep {}, Elapsed {:?}", sleep_time, end);
    }

    let avg = std::time::Duration::from_secs_f64(mean(&durations));
    // let std = std::time::Duration::from_secs_f64(std(&durations));
    let min = durations.iter().min();
    let max = durations.iter().max();
    let durations_ns = durations.iter().map(|d| d.as_nanos() as u64).collect::<Vec<_>>();
    let p50 = percentile(&durations_ns, 0.5);
    let p99 = percentile(&durations_ns, 0.99);
    let p999 = percentile(&durations_ns, 0.999);
    let p9999 = percentile(&durations_ns, 0.9999);

    println!("Desired sleep duration of {sleep_time} nanoseconds: {attempts} runs");
    println!("Min: {:?}", min.unwrap());
    println!("Avg: {:?}", avg);
    println!("P50: {:?}us", p50 as f32 / 1000.0);
    println!("P99: {:?}us", p99 as f32 / 1000.0);
    println!("P999: {:?}us", p999 as f32 / 1000.0);
    println!("P9999: {:?}us", p9999 as f32 / 1000.0);
    // println!("Std: {:?}", std);
    println!("Max: {:?}", max.unwrap());
    println!(
        "jitter: {:?}us",
        (max.unwrap().as_nanos() as f32 - min.unwrap().as_nanos() as f32) / 1000.0
    );

}
