use crate::git::git_command;
use num_cpus;
use regex::Regex;

#[derive(Clone)]
pub struct Settings {
    pub path: String,
    pub pattern: String,
    pub block_size: u64,
    pub jobs: usize,
}

impl Settings {
    pub fn new<T: Into<String>>(path: T, pattern: T, block_size: u64, jobs: usize) -> Self {
        let pattern: String = pattern.into();
        let regx = Regex::new(r"^[0-9a-f]$").unwrap();
        assert!(!regx.is_match(&pattern));
        assert!(jobs > 0);
        assert!(block_size < 64);
        Self {
            path: path.into(),
            pattern,
            block_size,
            jobs,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let path: String = git_command::current_dir_path();
        let num = num_cpus::get();
        Self::new(path, "0000000".to_owned(), 20, num - 1)
    }
}
