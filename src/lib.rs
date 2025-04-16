use std::error::Error;
use git2::Repository;

pub mod providers;
pub mod git_analysis;
pub mod git;
pub mod ui;
pub mod modes;

#[derive(Debug)]
pub struct Config {
    model: Box<dyn git_analysis::GitAnalyzer>,
}

#[derive(Debug)]
pub struct FileAnalysis {
    pub path: String,
    pub explanation: String,
}

impl Config {
    pub fn new(model: Box<dyn git_analysis::GitAnalyzer>) -> Self {
        Self { model }
    }

    pub async fn generate_commit_message(&self, diff: &str) -> Result<String, Box<dyn Error>> {
        self.model.generate_commit_message(diff).await
    }

    pub async fn analyze_changes(&self, repo: &Repository) -> Result<Vec<FileAnalysis>, Box<dyn Error>> {
        let file_diffs = git::get_file_diffs(repo)?;
        
        let analysis_futures: Vec<_> = file_diffs.into_iter().map(|(path, diff)| {
            let model = &self.model;
            async move {
                let explanation = model.analyze_file_changes(&diff).await?;
                Ok::<FileAnalysis, Box<dyn Error>>(FileAnalysis {
                    path,
                    explanation,
                })
            }
        }).collect();

        futures::future::join_all(analysis_futures)
            .await
            .into_iter()
            .collect()
    }

    pub async fn analyze_contributor(&self, stats: &str) -> Result<String, Box<dyn Error>> {
        self.model.analyze_contributor(stats).await
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".")?;
    
    let providers = providers::get_available_providers();

    let selected_idx = providers::select_provider(&providers)?;
    let config = Config::new(git_analysis::wrap_provider(providers.into_iter().nth(selected_idx).unwrap()));

    loop {
        let mode = ui::select_mode().await?;
        mode.execute(&config, &repo).await?;

        let options = ["✨ Do something else", "❌ Exit"];
        if ui::show_selection_menu("What would you like to do next?", &options, 0)? == 1 {
            break;
        }
        println!("\x1B[2J\x1B[1;1H"); // Clear screen
    }
    
    Ok(())
} 