use std::{error::Error, fs, process::Command};
use git2::{Repository, StatusOptions};

pub fn get_diff(repo: &Repository) -> Result<String, Box<dyn Error>> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    
    let statuses = repo.statuses(Some(&mut status_opts))?;
    if statuses.is_empty() {
        return Err("No changes to commit".into());
    }

    let mut diff_opts = git2::DiffOptions::new();
    let mut diff = String::new();

    for status in statuses.iter() {
        let path = status.path().unwrap();
        
        if status.status().is_wt_new() {
            // For new files, show their entire content
            if let Ok(content) = fs::read_to_string(path) {
                diff.push_str(&format!("New file: {}\n{}\n", path, content));
            }
        } else {
            // For modified files, show the diff
            let old_tree = repo.head()?.peel_to_tree()?;
            let diff_result = repo.diff_tree_to_workdir_with_index(Some(&old_tree), Some(&mut diff_opts))?;
            let mut diff_str = String::new();
            diff_result.print(git2::DiffFormat::Patch, |_, _, line| {
                diff_str.push_str(&format!("{}", String::from_utf8_lossy(line.content())));
                true
            })?;
            diff.push_str(&diff_str);
        }
    }

    Ok(diff)
}

pub fn get_file_diffs(repo: &Repository) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    
    let statuses = repo.statuses(Some(&mut status_opts))?;
    if statuses.is_empty() {
        return Err("No changes to commit".into());
    }

    let mut diff_opts = git2::DiffOptions::new();
    let mut file_diffs = Vec::new();

    for status in statuses.iter() {
        let path = status.path().unwrap().to_string();
        
        if status.status().is_wt_new() {
            // For new files, show their entire content
            if let Ok(content) = fs::read_to_string(&path) {
                file_diffs.push((path, format!("New file:\n{}", content)));
            }
        } else {
            // For modified files, show only this file's diff
            let old_tree = repo.head()?.peel_to_tree()?;
            diff_opts.pathspec(path.clone());
            let diff_result = repo.diff_tree_to_workdir_with_index(Some(&old_tree), Some(&mut diff_opts))?;
            let mut diff_str = String::new();
            diff_result.print(git2::DiffFormat::Patch, |_, _, line| {
                diff_str.push_str(&format!("{}", String::from_utf8_lossy(line.content())));
                true
            })?;
            if !diff_str.is_empty() {
                file_diffs.push((path, diff_str));
            }
            // Reset pathspec for next iteration
            diff_opts = git2::DiffOptions::new();
        }
    }

    Ok(file_diffs)
}

pub fn stage_and_commit(repo: &Repository, message: &str) -> Result<(), Box<dyn Error>> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    
    // Use git command directly instead of git2
    // Uses local signing config instead of recreating logic
    let output = Command::new("git")
        .args(["commit", "-S", "-m", message])
        .output()?;
        
    if !output.status.success() {
        return Err(format!(
            "Failed to create commit: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }
    
    Ok(())
}

#[derive(Clone)]
pub struct ContributorStats {
    pub name: String,
    pub email: String,
    pub commit_count: usize,
    pub additions: usize,
    pub deletions: usize,
    pub files_changed: Vec<String>,
    pub last_commit: String,
    pub file_types: std::collections::HashMap<String, usize>,
    pub commit_timeline: Vec<(i64, String)>, // timestamp and message
    pub largest_commits: Vec<(usize, usize, String)>, // (additions, deletions, message)
    pub most_modified_files: Vec<(String, usize)>, // (file path, modification count)
}

pub fn get_contributors(repo: &Repository) -> Result<Vec<ContributorStats>, Box<dyn Error>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut contributors = std::collections::HashMap::new();
    let mut file_modifications = Vec::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let author = commit.author();
        let name = author.name().unwrap_or("Unknown").to_string();
        let email = author.email().unwrap_or("unknown").to_string();

        let stats = contributors.entry((name.clone(), email.clone())).or_insert_with(|| ContributorStats {
            name: name.clone(),
            email: email.clone(),
            commit_count: 0,
            additions: 0,
            deletions: 0,
            files_changed: Vec::new(),
            last_commit: String::new(),
            file_types: std::collections::HashMap::new(),
            commit_timeline: Vec::new(),
            largest_commits: Vec::new(),
            most_modified_files: Vec::new(),
        });

        stats.commit_count += 1;
        stats.commit_timeline.push((
            commit.time().seconds(),
            commit.message().unwrap_or("No message").to_string()
        ));

        let mut commit_additions = 0;
        let mut commit_deletions = 0;

        if let Ok(parent) = commit.parent(0) {
            if let Ok(diff) = repo.diff_tree_to_tree(
                Some(&parent.tree()?),
                Some(&commit.tree()?),
                None,
            ) {
                if let Ok(stats_diff) = diff.stats() {
                    commit_additions = stats_diff.insertions();
                    commit_deletions = stats_diff.deletions();
                    stats.additions += commit_additions;
                    stats.deletions += commit_deletions;
                }

                diff.foreach(
                    &mut |delta, _| {
                        if let Some(path) = delta.new_file().path() {
                            if let Some(path_str) = path.to_str() {
                                let path_string = path_str.to_string();
                                
                                // Track file modifications
                                file_modifications.push((name.clone(), email.clone(), path_string.clone()));

                                // Track file types
                                if let Some(extension) = path.extension() {
                                    if let Some(ext_str) = extension.to_str() {
                                        let count = stats.file_types.entry(ext_str.to_string()).or_insert(0);
                                        *count += 1;
                                    }
                                }

                                if !stats.files_changed.contains(&path_string) {
                                    stats.files_changed.push(path_string);
                                }
                            }
                        }
                        true
                    },
                    None,
                    None,
                    None,
                )?;
            }
        }

        // Track large commits
        stats.largest_commits.push((
            commit_additions,
            commit_deletions,
            commit.message().unwrap_or("No message").to_string()
        ));
        stats.largest_commits.sort_by(|a, b| (b.0 + b.1).cmp(&(a.0 + a.1)));
        stats.largest_commits.truncate(5);

        if stats.last_commit.is_empty() {
            stats.last_commit = commit.message().unwrap_or("No message").to_string();
        }
    }

    // Process file modifications for each contributor
    let mut processed_contributors = contributors.clone();
    for (contributor_key, stats) in processed_contributors.iter_mut() {
        // Count file modifications
        let mut file_counts = std::collections::HashMap::new();
        for (name, email, path) in &file_modifications {
            if name == &contributor_key.0 && email == &contributor_key.1 {
                *file_counts.entry(path.clone()).or_insert(0) += 1;
            }
        }

        let mut file_mods: Vec<_> = file_counts.into_iter().collect();
        file_mods.sort_by(|a, b| b.1.cmp(&a.1));
        stats.most_modified_files = file_mods.into_iter().take(10).collect();
    }

    Ok(processed_contributors.into_iter().map(|(_, stats)| stats).collect())
}

pub fn get_contributor_commits(repo: &Repository, author_name: &str, author_email: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commits = Vec::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let author = commit.author();
        
        if author.name().unwrap_or("") == author_name && author.email().unwrap_or("") == author_email {
            let message = commit.message().unwrap_or("No message").to_string();
            let time = commit.time();
            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(time.seconds(), 0)
                .unwrap_or_else(|| chrono::Utc::now())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            
            commits.push(format!("{}: {}", datetime, message));
        }
    }

    Ok(commits)
} 