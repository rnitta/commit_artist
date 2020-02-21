#[derive(Clone)]
pub enum MatchMode {
    Repeated(usize),
    Pattern(String),
}

#[derive(Clone)]
pub struct Settings {
    pub path: String,
    pub mode: MatchMode,
    pub jobs: usize,
}

impl Settings {
    pub fn new<T: Into<String>>(path: T, mode: MatchMode, jobs: usize) -> Self {
        Self {
            path: path.into(),
            mode,
            jobs,
        }
    }
}
