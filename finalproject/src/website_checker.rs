use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use ureq::{self, Error};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: std::time::SystemTime,
}

pub fn check_website_status(url: &str, timeout: u64) -> WebsiteStatus {
    let start_time = Instant::now();
    let timestamp = std::time::SystemTime::now();

    let response = ureq::get(url)
        .timeout(Duration::from_millis(timeout))
        .call();

    let response_time = start_time.elapsed();

    let status_code = match response {
        Ok(response) => {
            if response.status() == 200 {
                Ok(response.status())
            } else {
                Err(format!("{}: status code {}", url, response.status()))
            }
        }
        Err(err) => {
            if let Error::Transport(_) = err {
                Err(format!("{}: timeout or transport error", url))
            } else {
                Err(format!("{}: {}", url, err))
            }
        }
    };

    WebsiteStatus {
        url: url.to_string(),
        status: status_code,
        response_time,
        timestamp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn test_website_success() {
        let url = "https://www.google.com";
        let timeout = 5000;
        let status = check_website_status(url, timeout);
        assert_eq!(status.url, url);
        assert!(status.status.is_ok());
    }

    #[test]
    fn test_website_error() {
        let url = "https://http://fakeurl-oiasjodij.com/";
        let timeout = 5000;
        let status = check_website_status(url, timeout);
        assert_eq!(status.url, url);
        assert!(status.status.is_err());
    }

    #[test]
    fn test_website_server() {
        let _m = mock("GET", "/").with_status(200).with_body("OK").create();

        let url = &mockito::server_url();
        let timeout = 5000;

        let status = check_website_status(url, timeout);
        assert_eq!(status.status, Ok(200));
    }

    #[test]
    fn test_website_server_failure() {
        let _m = mock("GET", "/").with_status(500).create();

        let url = &mockito::server_url();
        let timeout = 5000;

        let status = check_website_status(url, timeout);

        assert!(status.status.is_err());
    }

    #[test]
    fn test_concurrent_requests() {
        let websites = Arc::new(Mutex::new(vec![
            "https://www.discord.com",
            "https://www.google.com",
            "https://www.bing.com",
        ]));

        let mut handles = vec![];

        let start_time = Instant::now();
        for _ in 0..10 {
            let websites = Arc::clone(&websites);
            let handle = std::thread::spawn(move || {
                let websites = websites.lock().unwrap();
                for url in websites.iter() {
                    let _status = check_website_status(url, 5000);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let elapsed_time = start_time.elapsed();
        println!("Performance Test: {:?}", elapsed_time);
        assert!(elapsed_time.as_secs() < 5);
    }

    #[test]
    fn test_website_invalid_url() {
        let url = "https://invalid-url.com";
        let timeout = 5000;

        let status = check_website_status(url, timeout);
        assert!(status.status.is_err());
    }
}
