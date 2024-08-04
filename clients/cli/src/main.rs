use clap::Parser;
use reqwest::blocking::Client;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "oxidai_cli")]
#[command(about = "Fetches data from localhost:8080", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "/hello")]
    endpoint: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let client = Client::new();

    let url = format!("http://localhost:8080{}", cli.endpoint);

    let response = client.get(&url).send()?;

    println!("{}", response.text()?);

    Ok(())
}