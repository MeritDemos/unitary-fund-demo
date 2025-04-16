use std::error::Error;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use indicatif::{ProgressBar, ProgressStyle};
use termimad::{MadSkin, gray, StyledChar};

use crate::modes::Mode;

/// Renders markdown text in the terminal with proper styling
pub fn print_markdown(text: &str) {
    let mut skin = MadSkin::default();
    // Configure markdown styling
    skin.set_headers_fg(gray(255));  // Bright white for headers
    skin.bold.set_fg(gray(200));     // Light gray for bold
    skin.italic.set_fg(gray(180));   // Slightly darker for italic
    skin.bullet = StyledChar::from_fg_char(gray(180), '•');
    skin.quote_mark = StyledChar::from_fg_char(gray(180), '▐');
    skin.code_block.set_fg(gray(71)); // Light green for code blocks
    
    // Add a newline before and after for better spacing
    println!();
    skin.print_text(text);
    println!();
}

/// Prints a section header with a title
pub fn print_section(title: &str) {
    println!("\n{}", title);
    println!("{}\n", "═".repeat(title.chars().count()));
}

/// Prints a subsection header with a title
pub fn print_subsection(title: &str) {
    println!("\n{}", title);
    println!("{}", "─".repeat(title.chars().count()));
}

pub fn create_spinner(message: &str) -> Result<ProgressBar, Box<dyn Error>> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner} {msg}...")?
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner.set_message(format!("{}...", message));
    Ok(spinner)
}

pub fn show_selection_menu<T: AsRef<str> + ToString>(prompt: &str, items: &[T], default: usize) -> Result<usize, Box<dyn Error>> {
    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact()?)
}

pub async fn select_mode() -> Result<Mode, Box<dyn Error>> {
    let modes = [
        Mode::CommitMessage.description(),
        Mode::FileAnalysis.description(),
        Mode::ContributorAnalysis.description(),
    ];
    
    let selection = show_selection_menu("What would you like to do?", &modes, 0)?;
    
    Ok(match selection {
        0 => Mode::CommitMessage,
        1 => Mode::FileAnalysis,
        _ => Mode::ContributorAnalysis,
    })
}

pub fn get_repository_path(default: &str) -> Result<String, Box<dyn Error>> {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter repository path")
        .default(default.into())
        .allow_empty(true)
        .interact()?;
    Ok(if path.is_empty() { ".".to_string() } else { path })
}