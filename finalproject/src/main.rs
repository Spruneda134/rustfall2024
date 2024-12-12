use std::sync::{Arc, Mutex};
use std::thread;
mod config;
mod website_checker;

use config::Config;
use website_checker::check_website_status;

fn main() {
    let config = Config::load();
    let websites = vec![
        "https://www.discord.com",
        "https://www.google.com",
        "https://www.github.com",
        "https://www.stackoverflow.com",
        "https://www.nasa.gov",
    ];

    let websites = Arc::new(Mutex::new(websites));

    let mut handles = vec![];

    for _ in 0..config.num_threads {
        let websites = Arc::clone(&websites);
        let config = config.clone();
        let handle = thread::spawn(move || {
            let mut websites = websites.lock().unwrap();

            for url in websites.drain(..) {
                let website_status = check_website_status(url, config.timeout);
                println!("{:?}", website_status);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
