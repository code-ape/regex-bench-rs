extern crate core_affinity;
use std::time::{Instant, Duration, SystemTime};
use memchr::memchr_iter;


fn main() {
    // program args
    let mut args = std::env::args();
    let _ = args.next();
    let arg1 = args.next();
    if arg1.is_none() {
        panic!("missing first argument for comma-delimited CPU cores to use");
    }
    let arg2 = args.next();
    if arg2.is_none() {
        panic!("missing second argument for number of seconds");
    }
    let core_id_str = arg1.unwrap().parse::<String>().unwrap();
    let core_ids = core_id_str.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let num_seconds: u64 = arg2.unwrap().parse().unwrap();
    
    // Print available core IDs for reference
    let available_core_ids = core_affinity::get_core_ids().unwrap().iter().map(|x| x.id).collect::<Vec<usize>>();
    println!("cpu cores available = {:?}", available_core_ids);

    let work_duration = Instant::now();

    let mut handles = Vec::with_capacity(core_ids.len());
    for core_id in core_ids {
        let t_num_seconds = num_seconds;
        let t_core_id = core_id;
        handles.push(std::thread::spawn(move || {
            let res = core_affinity::set_for_current(core_affinity::CoreId { id: t_core_id });
            if !res {
                println!("WARNING! failed to pin thread to core #{}", t_core_id);
            }

            let mut subject = "#".to_string();
            let mut num_cycles = 0;
            let mut current_time = SystemTime::now();
            let start_time = current_time;
            let end_time = current_time + Duration::new(t_num_seconds, 0);
            while end_time > current_time {
                let bulk_cycle_count = 1_000;
                for _ in 0..bulk_cycle_count {
                    subject = replace_hash(&subject);
                }
                num_cycles += bulk_cycle_count;
                current_time = SystemTime::now();
            }
            let final_time_secs = current_time.duration_since(start_time).unwrap().as_secs_f32();
            let cycles_per_sec = num_cycles as f32 / final_time_secs;
            println!("core #{}, cycles = {}, secs = {}, cycles/sec = {}", t_core_id, num_cycles, final_time_secs, cycles_per_sec);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("total {}", work_duration.elapsed().as_secs_f32());
}

#[inline(never)]
fn replace_hash(haystack: &str) -> String {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for offset in memchr_iter(b'#', haystack.as_bytes()) {
        new.push_str(&haystack[last_match..offset]);
        new.push_str("benchmark#");
        last_match = offset + 1;
    }
    new.push_str(&haystack[last_match..]);
    new
}
