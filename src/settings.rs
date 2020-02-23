use crate::command;
use num_cpus;
use regex::Regex;

#[derive(Clone)]
pub struct Settings {
    pub path: String,
    pub pattern: String,
    pub block_size: usize,
    pub jobs: usize,
}

impl Settings {
    ///
    /// Construct.
    ///
    /// # Panics
    /// pattern chars length should be 1..=40
    /// jobs should be more than 0
    /// block size should be less than 64.
    ///
    pub fn new<T: Into<String>>(path: T, pattern: T, block_size: usize, jobs: usize) -> Self {
        let pattern: String = pattern.into();
        let regx = Regex::new(r"^[0-9a-f]{1,40}$").unwrap();
        assert!(regx.is_match(&pattern));
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
        let path: String = command::current_dir_path();
        let num = num_cpus::get();
        Self::new(path, "0000000".to_owned(), 20, num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;

    #[test]
    fn settings_constructor() {
        Settings::new("./", "0000000", 20, 10);
    }

    #[test]
    #[should_panic]
    fn nonnominal_settings1() {
        Settings::new("./", "invalidpattern", 20, 10);
    }

    #[test]
    #[should_panic]
    fn nonnominal_settings2() {
        Settings::new("./", "0000000", 1000, 10);
    }

    #[test]
    #[should_panic]
    fn nonnominal_settings3() {
        Settings::new("./", "0000000", 1000, 0);
    }
}
