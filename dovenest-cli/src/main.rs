use anyhow::Result;
use clap::Parser;
use dovenest::DoveNest;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Optional prompt (non-REPL mode)
    #[arg(short, long)]
    prompt: Option<String>,

    /// Model to use (default via env DOVENEST_MODEL or gpt-4.1-mini)
    #[arg(short = 'm', long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let dn = DoveNest::new();
    let model = args
        .model
        .or_else(|| std::env::var("DOVENEST_MODEL").ok())
        .unwrap_or_else(|| "gpt-4.1-mini".to_string());

    if let Some(p) = args.prompt {
        stream_once(&dn, &model, p).await?;
        return Ok(());
    }

    println!("🕊️  coo – DoveNest CLI (Streaming). Tippe 'exit' zum Beenden.");
    let stdin = io::stdin();
    loop {
        print!("you > ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        if line == "exit" {
            break;
        }
        stream_once(&dn, &model, line).await?;
        println!();
    }
    Ok(())
}

async fn stream_once(dn: &DoveNest, model: &str, user_input: String) -> Result<()> {
    print!("ai  > ");
    io::stdout().flush()?;
    dn.chat_stream_to(model, &user_input, |delta| {
        print!("{delta}");
        io::stdout().flush()?;
        Ok(())
    })
    .await?;
    println!(); // done
    Ok(())
}
