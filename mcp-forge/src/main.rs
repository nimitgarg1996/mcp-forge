use clap::{Parser, Subcommand};
use std::path::PathBuf;
use env_logger;
use log::{info, warn, error};

#[derive(Parser)]
#[clap(name = "mcp-forge", version = "0.1.0", author = "Your Name", about = "A CLI tool for generating MCP servers.")]
struct Cli {
    /// Enable verbose output
    #[clap(long, global = true)]
    verbose: bool,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize MCP-Forge in the current directory
    Init {
        #[clap(short, long)]
        name: String,
    },
    /// Build MCP server from the specified codebase
    Build {
        source: PathBuf,
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    /// Watch mode with auto-rebuild
    Watch {
        source: PathBuf,
        #[clap(short, long, default_value = "3000")]
        port: u16,
    },
    /// Show codebase statistics
    Analyze {
        source: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    if cli.verbose {
        std::env::set_var("RUST_LOG", "info");
    } else {
        std::env::set_var("RUST_LOG", "warn");
    }
    env_logger::init();
    info!("Starting MCP-Forge CLI");

    match cli.command {
        Commands::Init { name } => {
            info!("Initializing MCP-Forge for project: {}", name);
            println!("MCP-Forge initialized for project: {}", name);
        }
        Commands::Build { source, output } => {
            let output_path = output.unwrap_or_else(|| std::path::PathBuf::from("./mcp-server"));
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match mcp_forge::server::builder::build(source.clone(), output_path.clone()).await {
                    Ok(_) => info!("MCP server generated at {:?}", output_path),
                    Err(e) => {
                        error!("Build failed: {}", e);
                        println!("Error: Build failed. See logs for details.");
                    }
                }
            });
        }
        }
        Commands::Watch { source, port } => {
            use mcp_forge::incremental::incremental::IncrementalBuilder;
            use std::path::PathBuf;
            let cache_dir = PathBuf::from(".mcp-forge-cache");
            let builder = IncrementalBuilder::new(cache_dir.clone());
            info!("Starting watch mode on {:?} (port {})...", source, port);
            println!("Watching for file changes. Press Ctrl+C to exit.");
            std::thread::spawn(move || {
                builder.watch();
            });
            loop {
                std::thread::sleep(std::time::Duration::from_secs(60));
            }
        }
        Commands::Analyze { source } => {
            info!("Analyzing codebase at {:?}", source);
            println!("Analysis started for codebase: {:?}", source);
            // ...existing code...
        }
    }
}