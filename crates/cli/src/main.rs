use std::fs;

use clap::{Parser, Subcommand};
use hget_core;

#[derive(Parser)]
#[command(name = "hget", about = "HTTP client powered by .http files")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a .http file
    Run {
        /// Path to the .http file
        file: String,

        /// Environment name (e.g. staging, prod)
        #[arg(short, long)]
        env: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { file, env } => {
            println!("Running: {file}");
            if let Some(env) = env {
                println!("Environment: {env}");
            }

            let file = fs::read_to_string(file).unwrap();

            let http_requests = hget_core::parser::parse(&file);

            if http_requests.len() == 0 {
                panic!("no requests found");
            }

            let response = hget_core::executor::execute(http_requests.get(0).unwrap()).await.unwrap();

            let output = match serde_json::from_str::<serde_json::Value>(&response.body) {
                Ok(value) => serde_json::to_string_pretty(&value).unwrap(),
                Err(_) => response.body
            };
            
            println!("{output}");
        }
    }

    Ok(())
}
