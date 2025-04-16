use std::error::Error;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};

use crate::modes::Mode;

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