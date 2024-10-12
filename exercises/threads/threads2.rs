// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));  
    let mut handles = vec![];  
  
    for _ in 0..10 {  
        let status_shared = Arc::clone(&status);  
        let handle = thread::spawn(move || {  
            thread::sleep(Duration::from_millis(250));  
            // 锁定 Mutex 并获取 JobStatus 的可变引用  
            let mut status_inner = status_shared.lock().unwrap();  
            // 更新 jobs_completed  
            status_inner.jobs_completed += 1;  
        });  
        handles.push(handle);  
    }  
  
    // 等待所有线程完成  
    for handle in handles {  
        handle.join().unwrap();  
    }  
  
    // 在所有线程都完成后，打印最终的 jobs_completed 值  
    let status_final = status.lock().unwrap();  
    println!("jobs completed {}", status_final.jobs_completed);
}
