use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use hget_core::{self, http_file::HttpFile, repository::{self, Repository}, variable::variables_to_map};

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
    Add,
    Init {
        path: Option<PathBuf>,
    },
    Run {
        file: PathBuf,
    },
    Preview {
        file: PathBuf,
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Preview { file }) => {
            let repo = Repository::open(&file)?;

            let httpfile = repo.get_http_file(&file)?;

            println!("{:?}", httpfile);
            
            println!("{}", httpfile.to_string());
        }
        Some(Command::Add) => {
            let content = editor::open_editor_and_get_string();
            println!("{content}");
        }
        Some(Command::Init { path }) => {
            let absolute = std::path::absolute(path.unwrap_or(".".into())).unwrap();
            let repo = Repository::init(&absolute).map_err(|e| e.to_string())?;

            println!("{:?}", repo);
        }
        Some(Command::Run { mut file }) => {
            let extension = file.extension().map(|e| e.to_os_string());

            if extension.is_none() {
                file.set_extension("http");
            }
            
            let target = std::path::absolute(".").unwrap().join(&file);

            if !target.exists() {
                panic!("{}: file not found", target.to_str().unwrap_or(""));
            }

            let repo = Repository::open(&target).unwrap();
            let http_file = repo.get_http_file(&target).unwrap();
            let target_http_req = http_file.first().expect("http file doesn't have a request to run");
            println!("{:?}", target_http_req.queries);
            let response = target_http_req.run(variables_to_map(&http_file.variables)).await;

            println!("{response:?}");
        }
        None => {
            let file = cli.file.expect("provide a .http file or use 'hget add'");

            // let content = fs::read_to_string(&file).unwrap();
            // let http_requests = hget_core::parser::parse(&content);

            // if http_requests.len() == 0 {
            //     panic!("no requests found");
            // }

            // let response = http_requests.get(0).unwrap().run().await.unwrap();

            // let output = match serde_json::from_str::<serde_json::Value>(&response.body) {
            //     Ok(value) => serde_json::to_string_pretty(&value).unwrap(),
            //     Err(_) => response.body,
            // };

            // println!("{output}");
        }
    }

    Ok(())
}
