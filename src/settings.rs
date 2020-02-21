#[derive(Clone)]
pub struct Settings {
    pub path: String,
    pub pattern: String,
    pub block_size: usize,
    pub jobs: usize,
}

impl Settings {
    pub fn new<T: Into<String>>(path: T, pattern: T, block_size: usize, jobs: usize) -> Self {
        // TODO: panic if pattern has characters other than [0-9a-f]
        Self {
            path: path.into(),
            pattern: pattern.into(),
            block_size,
            jobs,
        }
    }
}
