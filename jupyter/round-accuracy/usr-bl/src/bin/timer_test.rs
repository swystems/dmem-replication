use round_test::*;

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

    // println!("ratio: {ratio}, threshold: {threshold}, sleep_time: {sleep_time}");
    for _ in 0..attempts {
        let start = std::time::Instant::now();

        // busy_poll_sleep(sleep_time, threshold);
        let _time = busy_poll_sleep_rdtsc(sleep_time, threshold);
        let end = start.elapsed();
        durations.push(end);
        //println!("Time to sleep {}, Elapsed {:?}", sleep_time, end);
    }

    let avg = std::time::Duration::from_secs_f64(mean(&durations));
    let std = std::time::Duration::from_secs_f64(std(&durations));
    let max = durations.iter().max();
    let min = durations.iter().min();
    println!("Desired sleep duration of {sleep_time} nanoseconds: {attempts} runs");
    println!("Avg: {:?}", avg);
    println!("Std: {:?}", std);
    println!("Max: {:?}", max.unwrap());
    println!("Min: {:?}", min.unwrap());
    println!(
        "jitter: {:?}us",
        (max.unwrap().as_nanos() as f32 - min.unwrap().as_nanos() as f32) / 1000.0
    );
}
