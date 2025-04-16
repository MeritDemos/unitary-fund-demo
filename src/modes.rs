use std::error::Error;
use git2::Repository;

use crate::git;
use crate::ui;
use crate::Config;

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

    pub async fn execute(&self, config: &Config, repo: &Repository) -> Result<(), Box<dyn Error>> {
        match self {
            Mode::CommitMessage => handle_commit_message(config, repo).await,
            Mode::FileAnalysis => handle_file_analysis(config, repo).await,
            Mode::ContributorAnalysis => handle_contributor_analysis(config, repo).await,
        }
    }
}

async fn handle_commit_message(config: &Config, repo: &Repository) -> Result<(), Box<dyn Error>> {
    let diff = git::get_diff(repo)?;
    
    loop {
        let commit_message = generate_with_spinner(config, &diff).await?;
        
        let options = [
            "✨ Regenerate message",
            "📝 Edit commit type",
            "✅ Stage and commit",
            "❌ Cancel"
        ];
        
        match ui::show_selection_menu("What would you like to do?", &options, 2)? {
            0 => continue, // Regenerate
            1 => {
                let types = [
                    "feat: New feature",
                    "fix: Bug fix",
                    "docs: Documentation",
                    "style: Formatting",
                    "refactor: Code restructure",
                    "test: Testing",
                    "chore: Maintenance",
                ];
                
                let type_idx = ui::show_selection_menu("Select commit type", &types, 0)?;
                let selected_type = types[type_idx].split(':').next().unwrap();
                let description = commit_message.split(':').nth(1).unwrap_or(&commit_message).trim();
                let new_message = format!("{}: {}", selected_type, description);
                
                println!("\n📝 New Commit Message");
                println!("══════════════════════");
                println!("{}\n", new_message);

                let confirm_options = ["✅ Confirm and commit", "🔄 Start over", "❌ Cancel"];
                match ui::show_selection_menu("Would you like to proceed with this commit message?", &confirm_options, 0)? {
                    0 => {
                        git::stage_and_commit(repo, &new_message)?;
                        println!("Changes committed successfully!");
                        break;
                    }
                    1 => continue,
                    _ => break,
                }
            }
            2 => {
                git::stage_and_commit(repo, &commit_message)?;
                println!("Changes committed successfully!");
                break;
            }
            _ => break,
        }
    }
    Ok(())
}

async fn handle_file_analysis(config: &Config, repo: &Repository) -> Result<(), Box<dyn Error>> {
    let spinner = ui::create_spinner("Analyzing changes")?;
    let analyses = config.analyze_changes(repo).await?;
    spinner.finish_and_clear();
    
    println!("\n📊 File Analysis Results");
    println!("══════════════════════\n");
    
    for analysis in analyses {
        println!("📁 {}", analysis.path);
        println!("───────────────────");
        println!("{}\n", analysis.explanation);
    }
    
    Ok(())
}

async fn handle_contributor_analysis(config: &Config, repo: &Repository) -> Result<(), Box<dyn Error>> {
    let contributors = git::get_contributors(repo)?;
    
    println!("\n👥 Repository Contributors");
    println!("═════════════════════════\n");
    
    let mut contributor_items: Vec<String> = contributors.iter().map(|c| {
        format!("{} <{}> ({} commits)", c.name, c.email, c.commit_count)
    }).collect();
    contributor_items.push("❌ Exit".to_string());

    loop {
        let selection = ui::show_selection_menu("Select a contributor to view details", &contributor_items, 0)?;

        if selection == contributor_items.len() - 1 {
            break;
        }

        let contributor = &contributors[selection];
        display_contributor_info(contributor);
        
        let spinner = ui::create_spinner("Analyzing contributor's work")?;
        let stats = format_contributor_stats(contributor, repo)?;
        let summary = config.analyze_contributor(&stats).await?;
        spinner.finish_and_clear();

        println!("\n🤖 AI Analysis");
        println!("═════════════");
        println!("{}\n", summary);

        println!("\nPress Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        println!("\x1B[2J\x1B[1;1H"); // Clear screen
    }

    Ok(())
}

async fn generate_with_spinner(config: &Config, diff: &str) -> Result<String, Box<dyn Error>> {
    let spinner = ui::create_spinner("Generating commit message")?;
    let commit_message = config.generate_commit_message(diff).await?;
    spinner.finish_and_clear();

    println!("\n📝 Generated Commit Message");
    println!("══════════════════════════");
    println!("{}\n", commit_message);
    
    Ok(commit_message)
}

fn display_contributor_info(contributor: &git::ContributorStats) {
    println!("\n👤 Contributor Details: {}", contributor.name);
    println!("══════════════════════════════");
    println!("📧 Email: {}", contributor.email);
    
    println!("\n📊 Statistics");
    println!("───────────");
    println!("  • Commits: {}", contributor.commit_count);
    println!("  • Lines added: {}", contributor.additions);
    println!("  • Lines deleted: {}", contributor.deletions);
    println!("  • Files changed: {}", contributor.files_changed.len());

    println!("\n📁 Most Modified Files");
    println!("──────────────────");
    for (file, count) in &contributor.most_modified_files {
        println!("  • {} ({} modifications)", file, count);
    }

    println!("\n🔧 File Types");
    println!("────────────");
    let mut file_types: Vec<_> = contributor.file_types.iter().collect();
    file_types.sort_by(|a, b| b.1.cmp(a.1));
    for (ext, count) in file_types {
        println!("  • {}: {} files", ext, count);
    }

    println!("\n📈 Largest Contributions");
    println!("─────────────────────");
    for (additions, deletions, message) in &contributor.largest_commits {
        println!("  • +{} -{} : {}", additions, deletions, message);
    }
}

fn format_contributor_stats(
    contributor: &git::ContributorStats,
    repo: &Repository,
) -> Result<String, Box<dyn Error>> {
    let commits = git::get_contributor_commits(
        repo,
        &contributor.name,
        &contributor.email
    )?;

    println!("\n🔄 Recent Commits");
    println!("───────────────");
    for commit in commits.iter().take(5) {
        println!("• {}", commit);
    }

    Ok(format!(
        "Contributor: {} <{}>

Statistics:
- Total commits: {}
- Lines added: {}
- Lines deleted: {}
- Files modified: {}

Most frequently modified files:
{}

File type distribution:
{}

Largest contributions:
{}

Recent commits:
{}

Modified files:
{}",
        contributor.name,
        contributor.email,
        contributor.commit_count,
        contributor.additions,
        contributor.deletions,
        contributor.files_changed.len(),
        contributor.most_modified_files.iter()
            .map(|(file, count)| format!("- {} ({} modifications)", file, count))
            .collect::<Vec<_>>()
            .join("\n"),
        contributor.file_types.iter()
            .map(|(ext, count)| format!("- {}: {} files", ext, count))
            .collect::<Vec<_>>()
            .join("\n"),
        contributor.largest_commits.iter()
            .map(|(add, del, msg)| format!("- +{} -{} : {}", add, del, msg))
            .collect::<Vec<_>>()
            .join("\n"),
        commits.iter()
            .take(5)
            .map(|c| format!("- {}", c))
            .collect::<Vec<_>>()
            .join("\n"),
        contributor.files_changed.iter()
            .map(|f| format!("- {}", f))
            .collect::<Vec<_>>()
            .join("\n")
    ))
} 