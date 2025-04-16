use std::error::Error;
use std::fmt::Debug;
use async_trait::async_trait;

use crate::providers::Provider;

/// Trait for git-specific model behavior
#[async_trait]
pub trait GitAnalyzer: Debug {
    fn name(&self) -> &str;
    async fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn Error>>;
    async fn analyze_file_changes(&self, diff: &str) -> Result<String, Box<dyn Error>>;
    async fn analyze_contributor(&self, stats: &str) -> Result<String, Box<dyn Error>>;
}

/// Implementation of GitAnalyzer that uses any Provider
#[derive(Debug)]
pub struct GitAnalyzerImpl {
    provider: Box<dyn Provider>,
}

impl GitAnalyzerImpl {
    pub fn new(provider: Box<dyn Provider>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl GitAnalyzer for GitAnalyzerImpl {
    fn name(&self) -> &str {
        self.provider.name()
    }

    async fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn Error>> {
        self.provider.generate_text(SYSTEM_MESSAGE, diff, 0.7).await
    }

    async fn analyze_file_changes(&self, diff: &str) -> Result<String, Box<dyn Error>> {
        self.provider.generate_text(FILE_ANALYSIS_PROMPT, diff, 0.7).await
    }

    async fn analyze_contributor(&self, stats: &str) -> Result<String, Box<dyn Error>> {
        self.provider.generate_text(CONTRIBUTOR_ANALYSIS_PROMPT, stats, 0.7).await
    }
}

pub fn wrap_provider(provider: Box<dyn Provider>) -> Box<dyn GitAnalyzer> {
    Box::new(GitAnalyzerImpl::new(provider))
}

const SYSTEM_MESSAGE: &str = r#"You are an expert software developer tasked with writing clear, concise, and informative git commit messages following the Conventional Commits specification. Given a git diff, you will:

1. Analyze the changes to understand what was modified
2. Create a commit message following these rules:
   - Use the conventional commit format: <type>: <description>
   - Common types are: feat (new feature), fix (bug fix), docs (documentation), style (formatting), refactor, test, chore
   - The description should use imperative mood ("Add feature" not "Added feature")
   - The entire message should be 50 chars or less
   - Focus on the "why" and "what" rather than the "how"

Please provide only the commit message without any additional commentary or markdown formatting."#;

const FILE_ANALYSIS_PROMPT: &str = r#"You are an expert software developer tasked with analyzing changes to a file. Given a git diff or file content, you will:

1. Analyze the changes to understand what was modified
2. The following could be relevant to the changes:
   - What functionality was added, modified, or removed
   - Any potential impact on the codebase
   - Notable implementation details or design decisions
   - Any potential concerns or suggestions for improvement

Format your response in markdown with appropriate headers, lists, and code blocks where relevant.
Do not include ``` tags in your response unless you are explicitly using them to format code. Do not include ```markdown!
Try to be as concise as possible while still providing meaningful insights.
Please focus on providing meaningful insights rather than just describing the changes line by line."#;

const CONTRIBUTOR_ANALYSIS_PROMPT: &str = r#"You are an expert software developer tasked with analyzing a contributor's work in a repository. Given information about their commits, files changed, and overall impact, you will:

1. Analyze their contributions to understand their role and impact:
   - Primary areas of focus and expertise
   - Types of changes they typically make
   - Impact on the codebase architecture and quality
   - Notable patterns in their work

2. Provide a concise but comprehensive summary that covers:
   - Their main areas of contribution
   - The significance of their changes
   - Their apparent role in the project
   - Any notable patterns or specialties in their work

Format your response in markdown with appropriate headers, lists, and emphasis where relevant.
Please provide a clear, professional summary that helps understand the contributor's role and impact on the project."#; 