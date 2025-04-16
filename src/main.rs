use std::error::Error;

pub mod providers;
pub mod ui;
pub mod modes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let providers = providers::get_available_providers();

    println!("Available providers:");
    for (i, provider) in providers.iter().enumerate() {
        println!("{}: {}", i, provider.name());
    }

    let selected_idx = providers::select_provider(&providers)?;
    let provider = &providers[selected_idx];

    println!("Selected provider: {}", provider.name());

    let mode = ui::select_mode().await?;

    println!("Selected mode: {}", mode.description());

    Ok(())
}
