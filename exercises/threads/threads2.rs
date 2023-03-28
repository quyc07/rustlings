// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed


use std::ops::AddAssign;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let mut handles = vec![];
    for i in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("{i}");
            // TODO: You must take an action before you update a shared value
            status_shared.jobs_completed.lock().and_then(|mut status| {
                *status += 1;
                Ok(Some(status))
            }).unwrap();
            i
        });
        handles.push(handle);
    }
    for handle in handles {
        let x = handle.join().unwrap();
        println!("x={}", x);
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
    }
    println!("jobs completed {}", status.jobs_completed.lock().unwrap());
}
