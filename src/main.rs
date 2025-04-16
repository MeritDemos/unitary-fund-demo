use dotenv;
use merit_cli_demo::run;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let repo_path = env::args().nth(1);
    run(repo_path).await
}