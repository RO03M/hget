use std::fs;

use clap::{Parser, Subcommand};
use hget_core;

mod editor;

#[derive(Parser)]
#[command(name = "hget", about = "HTTP client powered by .http files")]
struct Cli {
    /// Path to a .http file to run
    file: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Open editor to write a new .http request
    Add,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Add) => {
            let content = editor::open_editor_and_get_string();
            println!("{content}");
        }
        None => {
            let file = cli.file.expect("provide a .http file or use 'hget add'");

            let content = fs::read_to_string(&file).unwrap();
            let http_requests = hget_core::parser::parse(&content);

            if http_requests.len() == 0 {
                panic!("no requests found");
            }

            let response = http_requests.get(0).unwrap().run().await.unwrap();

            let output = match serde_json::from_str::<serde_json::Value>(&response.body) {
                Ok(value) => serde_json::to_string_pretty(&value).unwrap(),
                Err(_) => response.body,
            };

            println!("{output}");
        }
    }

    Ok(())
}
