#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReactRuntime {
    Classic,
    Automatic,
}

#[derive(Debug, Clone)]
pub struct ReactConfig {
    pub runtime: ReactRuntime,
}

impl Default for ReactConfig {
    fn default() -> Self {
        Self { runtime: ReactRuntime::Automatic }
    }
}
