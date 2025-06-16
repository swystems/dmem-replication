pub fn get_time_ns() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!");
    duration.as_nanos() as u64
}

fn mean(durations: &[std::time::Duration]) -> f64{
    let dur = durations.iter().sum::<std::time::Duration>()/ durations.len() as u32;    
    dur.as_secs_f64()
}

fn std(durations: &[std::time::Duration]) -> f64{
    let mean = mean(durations);    
    let variance = durations.iter().map(|el| {
            let diff = el.as_secs_f64()- mean;
            diff*diff
        }
     ).sum::<f64>()/durations.len() as f64;
     variance.sqrt()
}

// busy poll sleep: alternate between sleeping and busy polling
// sleep for ns - threshold nanoseconds, busy poll for the rest
#[no_mangle]
pub fn busy_poll_sleep(mut ns: u64, threshold: u64){
    if ns==0{
        return;
    }
    let start = std::time::Instant::now();

    if ns > threshold{
        let sleep_duration = std::time::Duration::from_nanos(ns - threshold);
        // uses nanosleep() syscall on linux
        std::thread::sleep(sleep_duration);
        // unsafe { libc::nanosleep(&libc::timespec{tv_sec: sleep_duration.as_secs() as i64, tv_nsec: sleep_duration.subsec_nanos() as i64}, &mut libc::timespec{tv_sec: 0, tv_nsec: 0}) };
        // One might expect the diff to always be 1_000_000 nanoseconds,
        // but given that thread::sleep might sleep for more than the requested time
        // it might not always be the case.
        let diff =  start.elapsed();
       // println!("diff {:?}", diff);
        if diff.as_nanos() as u64 > ns{
            return;
        }
        ns -= diff.as_nanos() as u64;
        // println!("NS is {ns} {:?}", diff);
    }

    let new_start = get_time_ns();
    while get_time_ns() - new_start< ns {
        std::hint::spin_loop();
        //std::thread::yield_now();
    }
        
}

fn main() {
     let matches = clap::Command::new("Performance test").version("1.0").about("")
        .arg(clap::arg!(-s --sleep <SLEEP_TIME> "Time to sleep for in usec.").required(true))
        .arg(clap::arg!(-t --threshold <THRESHOLD> "Threshold").default_value("0"))
        .arg(clap::arg!(-r --ratio <RATIO> "Sleep / busy poll ratio").default_value("1"))
        .arg(clap::arg!(-a --attempts <ATTEMPTS> "Number of tests").default_value("1000"))
        .get_matches();

    // Parse the command line arguments
    let sleep_time = matches.get_one::<String>("sleep").unwrap().replace('_', "");
    let sleep_time =  sleep_time.parse::<u64>().expect("String not parsable");
    let threshold= matches.get_one::<String>("threshold").unwrap().replace('_', "");
    let mut threshold = threshold.parse::<u64>().expect("String not parsable");
    let  ratio = matches.get_one::<String>("ratio").unwrap().replace('_',"");
    let ratio = ratio.parse::<f32>().expect("String not parsable");
    let mut durations = Vec::new();
    let attempts = matches.get_one::<String>("attempts").unwrap().replace('_', "");
    let attempts = attempts.parse::<usize>().expect("String not parsable");
    
    if sleep_time < threshold {
        panic!("Sleep time must be greater than threshold");
    }

    if ratio >= 0.0 && ratio < 1.0 {
        // If ratio is set, we use it to calculate the threshold
        // threshold = sleep_time / ratio
        threshold = (sleep_time as f32 * (1.0-ratio)) as u64;
    }

    // println!("ratio: {ratio}, threshold: {threshold}, sleep_time: {sleep_time}");
    for _ in 0..attempts{
        let start = std::time::Instant::now();    
    
        busy_poll_sleep(sleep_time, threshold);
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
    println!("jitter: {:?}us", (max.unwrap().as_nanos() as f32- min.unwrap().as_nanos() as f32)/1000.0);
}
