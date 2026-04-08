use clap::{Parser, Subcommand};

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
            todo!("parse and execute the .http file")
        }
    }
}
