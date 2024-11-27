use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

fn sending_data_across_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let ten_millis = time::Duration::from_millis(10);

    for i in 1..=5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                {
                    let mut num = counter.lock().unwrap();
                    println!("Thread: {}, count: {}", i, *num);
                    *num += 1;
                }
                thread::sleep(ten_millis);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *counter.lock().unwrap());
}

fn main() {
    sending_data_across_threads();
}
