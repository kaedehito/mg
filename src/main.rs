mod compress;
mod get_ignore;
mod init;
mod labels;
mod list;
mod paths;
mod remove;
mod spinner;
mod uuid;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save a current progress
    Save {
        /// Make a break to save the current progress.
        label: Option<String>,
    },
    /// Initalize a new progress
    Init {
        /// Make a break to save the current progress.
        label: Option<String>,
    },
    /// Remove a saved progress
    Remove {
        /// Required field to remove to the specified delimiter
        label: String,
    },
    /// List all saved progress
    List,
    /// Reset the progress
    Reset {
        /// Required field to return to the specified delimiter.
        /// If not specified, the last saved progress will be used.
        label: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Save { label } => {
            if let Some(label) = label {
                compress::compress_file(&label).unwrap_or_else(|e| {
                    eprintln!("Failed to save files: {}", e);
                    std::process::exit(1);
                });
            } else {
                println!("\x1b[33mWARN:\x1b[0m No label provided. Save to latest label.");
                let latest_label = labels::Labels::get_latest_label().unwrap_or_else(|e| {
                    eprintln!("Error getting latest label: {}", e);
                    std::process::exit(1);
                });
                compress::compress_file(&latest_label).unwrap_or_else(|e| {
                    eprintln!("Failed to save files: {}", e);
                    std::process::exit(1);
                });
            }
        }

        Commands::Init { label } => {
            init::init(label);
        }

        Commands::Remove { label } => {
            remove::remove(&label);
        }

        Commands::List => {
            list::list();
        }

        Commands::Reset { label } => {
            if let Some(label) = label {
                // Reset logic here
                println!("Resetting to label: {}", label);
            } else {
                // Reset to last saved progress
                println!("Resetting to last saved progress.");
            }
        }
    }
}
