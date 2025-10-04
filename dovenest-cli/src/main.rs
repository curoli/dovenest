use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

/// 🕊️  DoveNest CLI – "coo"
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Pfad zu einer Konfigurationsdatei
    #[arg(short, long)]
    config: Option<String>,
}

/// Beispiel-Config, die wir mit serde laden
#[derive(Debug, Deserialize)]
struct Config {
    model: String,
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(path) = cli.config {
        let text = std::fs::read_to_string(&path)?;
        // Hier nehmen wir erstmal TOML als Beispiel
        let cfg: Config = toml::from_str(&text)?;
        println!("Konfiguration geladen: {:?}", cfg);
    }

    println!("🕊️  coo – DoveNest CLI");
    println!("Lib sagt: {}", dovenest::hello());

    Ok(())
}
