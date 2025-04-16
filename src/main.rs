use std::error::Error;

mod providers;

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

    Ok(())
}
