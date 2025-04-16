#[derive(Debug)]
pub enum Mode {
    CommitMessage,
    FileAnalysis,
    ContributorAnalysis,
}

impl Mode {
    pub fn description(&self) -> &'static str {
        match self {
            Mode::CommitMessage => "Generate commit message",
            Mode::FileAnalysis => "Analyze file changes",
            Mode::ContributorAnalysis => "Analyze contributors",
        }
    }
}