use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    /*
    封装了一个Mutex类型的互斥锁和一个JobStatus类型的共享数据。Arc类型的智能指针可以在多个线程之间共享，而Mutex类型的互斥锁可以确保
    在同一时间只有一个线程可以访问共享数据，从而避免竞态条件的发生。
     */
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        let status = status.lock().unwrap();
        println!("jobs completed {}", status.jobs_completed);
    }
}
