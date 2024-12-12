use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use ureq;
use crate::config::Config;

pub fn check_website_status(websites: Arc<Mutex<Vec<&str>>>, config: Config) {
    let mut websites = websites.lock().unwrap();

    for url in websites.drain(..) {
        let start_time = Instant::now();
        let response = ureq::get(url).timeout(Duration::from_millis(config.timeout)).call();

        let response_time = start_time.elapsed();
        let status = match response {
            Ok(res) => Ok(res.status()),
            Err(err) => Err(format!("Error: {}", err)),
        };

        println!("{:?} - {:?} - {:?}", url, status, response_time);
    }
}
