#[derive(Clone)]
pub struct Config {
    pub num_threads: usize,
    pub timeout: u64,
}

impl Config {
    pub fn load() -> Self {
        Config {
            num_threads: 4,
            timeout: 5000,
        }
    }
}
